//! Model download and management

use anyhow::Result;
use futures_util::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};
use tauri::{AppHandle, Emitter, Manager};

// ── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub size_mb: u32,
    pub status: String, // "bundled" | "installed" | "not_installed"
    pub is_bundled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub model_id: String,
    pub downloaded: u64,
    pub total: Option<u64>,
    pub percentage: f32,
    pub status: String,
}

// ── Model registry ───────────────────────────────────────────────────────────

struct ModelDef {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    language: &'static str,
    size_mb: u32,
    is_bundled: bool,
    download_url: Option<&'static str>,
    /// Folder name inside the downloaded archive and on disk
    folder_name: &'static str,
    /// Stem shared by encoder/decoder/joiner, e.g. "epoch-20-avg-1"
    model_stem: &'static str,
}

const MODELS: &[ModelDef] = &[
    ModelDef {
        id: "sherpa-zh",
        name: "ZipFormer Chinese",
        description: "Multi-dialect Chinese · bundled",
        language: "zh",
        size_mb: 248,
        is_bundled: true,
        download_url: None,
        folder_name: "sherpa-onnx-zipformer-multi-zh-hans-2023-9-2",
        model_stem: "epoch-20-avg-1",
    },
    ModelDef {
        id: "sherpa-en",
        name: "ZipFormer English",
        description: "English · ~300 MB download",
        language: "en",
        size_mb: 300,
        is_bundled: false,
        download_url: Some(
            "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-zipformer-en-2023-06-26.tar.bz2",
        ),
        folder_name: "sherpa-onnx-zipformer-en-2023-06-26",
        model_stem: "epoch-99-avg-1",
    },
];

// ── Paths ─────────────────────────────────────────────────────────────────────

pub fn models_dir(app: &AppHandle) -> PathBuf {
    app.path()
        .app_local_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models")
}

/// Returns `(model_dir, stem)` for the given model ID, e.g.
/// `(".../sherpa-onnx-zipformer-en-2023-06-26", "epoch-99-avg-1")`.
pub fn model_path(app: &AppHandle, model_id: &str) -> Option<(PathBuf, &'static str)> {
    let def = MODELS.iter().find(|d| d.id == model_id)?;
    let dir = if def.is_bundled {
        app.path()
            .resolve(
                format!("resources/models/sherpa-zh/{}", def.folder_name),
                tauri::path::BaseDirectory::Resource,
            )
            .ok()?
    } else {
        let p = models_dir(app).join(def.folder_name);
        if p.exists() { p } else { return None; }
    };
    Some((dir, def.model_stem))
}

fn model_status(app: &AppHandle, def: &ModelDef) -> String {
    if def.is_bundled {
        return "bundled".to_string();
    }
    let p = models_dir(app).join(def.folder_name);
    if p.exists() { "installed".to_string() } else { "not_installed".to_string() }
}

pub fn list_models(app: &AppHandle) -> Vec<ModelInfo> {
    MODELS
        .iter()
        .map(|def| ModelInfo {
            id: def.id.to_string(),
            name: def.name.to_string(),
            description: def.description.to_string(),
            language: def.language.to_string(),
            size_mb: def.size_mb,
            status: model_status(app, def),
            is_bundled: def.is_bundled,
        })
        .collect()
}

// ── Download ──────────────────────────────────────────────────────────────────

type CancelMap = Arc<Mutex<HashMap<String, tokio::sync::oneshot::Sender<()>>>>;

static CANCEL_MAP: OnceLock<CancelMap> = OnceLock::new();
fn cancel_map() -> &'static CancelMap {
    CANCEL_MAP.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

pub async fn start_download(model_id: String, app: AppHandle) -> Result<()> {
    let def = MODELS
        .iter()
        .find(|d| d.id == model_id)
        .ok_or_else(|| anyhow::anyhow!("Unknown model: {}", model_id))?;

    let url = def
        .download_url
        .ok_or_else(|| anyhow::anyhow!("Model {} is not downloadable", model_id))?;

    let dest_dir = models_dir(&app);
    std::fs::create_dir_all(&dest_dir)?;

    let tmp_path = dest_dir.join(format!("{}.tar.bz2.tmp", model_id));
    let folder_name = def.folder_name.to_string();

    // Register cancellation channel
    let (cancel_tx, mut cancel_rx) = tokio::sync::oneshot::channel::<()>();
    cancel_map().lock().unwrap().insert(model_id.clone(), cancel_tx);

    info!("Downloading {} from {}", model_id, url);

    let response = reqwest::Client::new().get(url).send().await?;
    let total = response.content_length();
    let mut stream = response.bytes_stream();

    let mut file = tokio::fs::File::create(&tmp_path).await?;
    let mut downloaded: u64 = 0;

    loop {
        tokio::select! {
            chunk = stream.next() => {
                match chunk {
                    Some(Ok(bytes)) => {
                        use tokio::io::AsyncWriteExt;
                        file.write_all(&bytes).await?;
                        downloaded += bytes.len() as u64;
                        let percentage = total
                            .map(|t| (downloaded as f32 / t as f32) * 100.0)
                            .unwrap_or(0.0);
                        let _ = app.emit("model_download_progress", DownloadProgress {
                            model_id: model_id.clone(),
                            downloaded,
                            total,
                            percentage,
                            status: "downloading".to_string(),
                        });
                    }
                    Some(Err(e)) => {
                        cancel_map().lock().unwrap().remove(&model_id);
                        let _ = tokio::fs::remove_file(&tmp_path).await;
                        return Err(e.into());
                    }
                    None => break,
                }
            }
            _ = &mut cancel_rx => {
                info!("Download cancelled: {}", model_id);
                drop(file);
                let _ = tokio::fs::remove_file(&tmp_path).await;
                cancel_map().lock().unwrap().remove(&model_id);
                return Ok(());
            }
        }
    }

    {
        use tokio::io::AsyncWriteExt;
        file.flush().await?;
    }
    drop(file);

    // Signal extraction phase
    let _ = app.emit("model_download_progress", DownloadProgress {
        model_id: model_id.clone(),
        downloaded,
        total,
        percentage: 100.0,
        status: "extracting".to_string(),
    });

    // Extract in blocking thread
    info!("Extracting {}", model_id);
    let dest_dir_clone = dest_dir.clone();
    let tmp_path_clone = tmp_path.clone();
    tokio::task::spawn_blocking(move || -> Result<()> {
        let f = std::fs::File::open(&tmp_path_clone)?;
        let bz2 = bzip2::read::BzDecoder::new(f);
        let mut archive = tar::Archive::new(bz2);
        archive.unpack(&dest_dir_clone)?;
        std::fs::remove_file(&tmp_path_clone)?;
        Ok(())
    })
    .await??;

    cancel_map().lock().unwrap().remove(&model_id);
    let _ = app.emit("model_download_complete", model_id.clone());
    info!("Model {} ready at {:?}", model_id, dest_dir.join(&folder_name));

    Ok(())
}

pub fn cancel_download(model_id: &str) {
    if let Some(tx) = cancel_map().lock().unwrap().remove(model_id) {
        let _ = tx.send(());
    }
}

pub fn delete_model(app: &AppHandle, model_id: &str) -> Result<()> {
    let def = MODELS
        .iter()
        .find(|d| d.id == model_id)
        .ok_or_else(|| anyhow::anyhow!("Unknown model: {}", model_id))?;

    if def.is_bundled {
        return Err(anyhow::anyhow!("Cannot delete bundled model"));
    }

    let path = models_dir(app).join(def.folder_name);
    if path.exists() {
        std::fs::remove_dir_all(&path)?;
    }
    Ok(())
}
