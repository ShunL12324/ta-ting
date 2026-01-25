//! 标点恢复模块
//!
//! 提供两种实现:
//! 1. 基于规则 (rule_based) - 轻量级，零依赖 ✅ 可用
//! 2. ONNX 模型 - 高准确度 ⚠️ 待修复 (ort 2.0 API 变化)

mod rule_based;

pub use rule_based::RuleBasedPunctuationRestorer;

// ONNX 模型支持 - 待 ort 2.0 正式版发布后启用
// 当前使用 ort 2.0.0-rc.11，API 还在变化中
//
// TODO: 等 ort 2.0 稳定后启用以下模块
// mod processor;
// pub use processor::PunctuationProcessor;

// ========== 以下是 ONNX 实现（待修复）==========
//
// use anyhow::Result;
// use ort::session::Session;
// use std::path::Path;
// use tokenizers::Tokenizer;
//
// pub struct PunctuationRestorer {
//     session: Session,
//     tokenizer: Tokenizer,
//     label_map: Vec<String>,
// }
//
// impl PunctuationRestorer {
//     pub fn new(model_path: impl AsRef<Path>, tokenizer_path: impl AsRef<Path>) -> Result<Self> {
//         // ... 实现待修复 ...
//         unimplemented!("等待 ort 2.0 正式版")
//     }
// }
