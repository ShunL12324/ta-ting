//! Sherpa-ONNX 语音识别引擎
//!
//! 比 Whisper 快 5-10 倍的离线 ASR 引擎

use anyhow::Result;
use log::info;
use sherpa_rs::zipformer::{ZipFormer, ZipFormerConfig};
use std::path::Path;
use crate::punctuation::RuleBasedPunctuationRestorer;

/// Sherpa 语音识别引擎
pub struct SherpaEngine {
    recognizer: ZipFormer,
    punctuation: RuleBasedPunctuationRestorer,
}

impl SherpaEngine {
    /// 创建新的 Sherpa 引擎
    ///
    /// # 参数
    /// - `model_dir`: 模型目录路径（包含 encoder/decoder/joiner/tokens）
    ///
    /// # 返回
    /// - `Ok(SherpaEngine)`: 成功创建
    /// - `Err`: 模型加载失败
    pub fn new<P: AsRef<Path>>(model_dir: P) -> Result<Self> {
        let model_dir = model_dir.as_ref();

        info!("正在加载 Sherpa 模型: {:?}", model_dir);

        let config = ZipFormerConfig {
            encoder: model_dir.join("encoder-epoch-20-avg-1.onnx")
                .to_string_lossy()
                .to_string(),
            decoder: model_dir.join("decoder-epoch-20-avg-1.onnx")
                .to_string_lossy()
                .to_string(),
            joiner: model_dir.join("joiner-epoch-20-avg-1.onnx")
                .to_string_lossy()
                .to_string(),
            tokens: model_dir.join("tokens.txt")
                .to_string_lossy()
                .to_string(),
            ..Default::default()
        };

        let recognizer = ZipFormer::new(config)
            .map_err(|e| anyhow::anyhow!("Failed to create Sherpa recognizer: {}", e))?;

        info!("✅ Sherpa 模型加载成功");

        Ok(Self {
            recognizer,
            punctuation: RuleBasedPunctuationRestorer::new(),
        })
    }

    /// 转录音频
    ///
    /// # 参数
    /// - `audio_data`: 音频数据（16kHz 单声道 f32 格式）
    ///
    /// # 返回
    /// - `Ok(String)`: 转录的文本
    /// - `Err`: 转录失败
    pub fn transcribe(&mut self, audio_data: &[f32]) -> Result<String> {
        info!(
            "开始 Sherpa 转录: {} 个采样点 ({:.2} 秒)",
            audio_data.len(),
            audio_data.len() as f32 / 16000.0,
        );

        // Sherpa 解码
        let text = self.recognizer.decode(16000, audio_data.to_vec());
        let text = text.trim().to_string();

        info!("✅ Sherpa 转录完成（无标点）: \"{}\" ({} 字符)", text, text.chars().count());

        // 添加标点符号
        let text_with_punct = self.punctuation.restore(&text)?;

        info!("✅ 添加标点后: \"{}\"", text_with_punct);

        Ok(text_with_punct)
    }
}
