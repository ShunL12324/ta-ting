// ASR 引擎模块
// pub mod engine;  // Whisper 引擎（已被 Sherpa 替代，保留代码备用）
pub mod sherpa_engine;

// pub use engine::{WhisperEngine, Language, Segment};
pub use sherpa_engine::SherpaEngine;
