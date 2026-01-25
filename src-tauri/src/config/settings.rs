// 应用设置
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub model: String,
    pub language: String,
    pub hotkey: String,
    pub auto_paste: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            model: "base".to_string(),
            language: "zh".to_string(),
            hotkey: "Ctrl+Shift+V".to_string(),
            auto_paste: true,
        }
    }
}

impl AppSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> anyhow::Result<Self> {
        // TODO: 从配置文件加载
        Ok(Self::default())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        // TODO: 保存到配置文件
        Ok(())
    }
}
