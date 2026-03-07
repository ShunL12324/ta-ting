//! Punctuation restoration using sherpa-onnx CT-Transformer model
//!
//! Model: sherpa-onnx-punct-ct-transformer-zh-en-vocab272727-2024-04-12
//! Download: scripts/download-models.sh

pub use sherpa_rs::punctuate::{Punctuation, PunctuationConfig};
