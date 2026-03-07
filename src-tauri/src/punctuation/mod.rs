//! 标点恢复模块
//!
//! 提供两种实现:
//! 1. 基于规则 (rule_based) - 轻量级，零依赖 ✅ 可用
//! 2. ONNX 模型 - 高准确度 ⚠️ 待修复 (ort 2.0 API 变化)

mod rule_based;

pub use rule_based::RuleBasedPunctuationRestorer;
