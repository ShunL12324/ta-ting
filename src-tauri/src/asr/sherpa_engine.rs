//! Sherpa-ONNX speech recognition engine

use anyhow::Result;
use log::info;
use sherpa_rs::punctuate::{Punctuation, PunctuationConfig};
use sherpa_rs::zipformer::{ZipFormer, ZipFormerConfig};
use std::path::Path;

pub struct SherpaEngine {
    recognizer: ZipFormer,
    punctuation: Option<Punctuation>,
}

impl SherpaEngine {
    pub fn new<P: AsRef<Path>>(model_dir: P, punct_model: Option<&str>) -> Result<Self> {
        let model_dir = model_dir.as_ref();

        info!("Loading Sherpa ASR model: {:?}", model_dir);

        let config = ZipFormerConfig {
            encoder: model_dir
                .join("encoder-epoch-20-avg-1.onnx")
                .to_string_lossy()
                .to_string(),
            decoder: model_dir
                .join("decoder-epoch-20-avg-1.onnx")
                .to_string_lossy()
                .to_string(),
            joiner: model_dir
                .join("joiner-epoch-20-avg-1.onnx")
                .to_string_lossy()
                .to_string(),
            tokens: model_dir.join("tokens.txt").to_string_lossy().to_string(),
            ..Default::default()
        };

        let recognizer = ZipFormer::new(config)
            .map_err(|e| anyhow::anyhow!("Failed to create Sherpa recognizer: {}", e))?;

        info!("Sherpa ASR model loaded");

        let punctuation = punct_model
            .filter(|p| !p.is_empty() && Path::new(p).exists())
            .and_then(|path| {
                match Punctuation::new(PunctuationConfig {
                    model: path.to_string(),
                    ..Default::default()
                }) {
                    Ok(p) => {
                        info!("Punctuation model loaded");
                        Some(p)
                    }
                    Err(e) => {
                        log::warn!("Failed to load punctuation model: {}, skipping", e);
                        None
                    }
                }
            });

        Ok(Self {
            recognizer,
            punctuation,
        })
    }

    /// Transcribe audio data (16kHz mono f32).
    pub fn transcribe(&mut self, audio_data: &[f32]) -> Result<String> {
        info!(
            "Starting transcription: {} samples ({:.2}s)",
            audio_data.len(),
            audio_data.len() as f32 / 16000.0,
        );

        let text = self.recognizer.decode(16000, audio_data.to_vec());
        let text = text.trim().to_string();

        info!(
            "Transcription (raw): \"{}\" ({} chars)",
            text,
            text.chars().count()
        );

        if let Some(ref mut punct) = self.punctuation {
            let result = punct.add_punctuation(&text);
            info!("After punctuation: \"{}\"", result);
            Ok(result)
        } else {
            Ok(text)
        }
    }
}
