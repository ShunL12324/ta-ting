// App settings with JSON persistence
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Hotkey in internal format, e.g. "Ctrl+Shift+KeyV"
    pub hotkey: String,
    pub auto_paste: bool,
    /// Active ASR model ID (e.g. "sherpa-zh", "sherpa-en")
    #[serde(default = "default_model")]
    pub active_model: String,
}

fn default_model() -> String {
    "sherpa-zh".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hotkey: "Ctrl+Shift+KeyV".to_string(),
            auto_paste: true,
            active_model: default_model(),
        }
    }
}

impl AppSettings {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
