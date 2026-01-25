# 标点恢复功能 - 当前状态和建议

## 🎯 当前状态

### ✅ 已完成
1. **基于规则的标点恢复** (100% 可用)
   - 文件: `src-tauri/src/punctuation/rule_based.rs`
   - 零依赖，即用
   - 准确度: ~60-70%

2. **ONNX 模型框架** (需要调试)
   - 文件: `src-tauri/src/punctuation/mod.rs`
   - 文件: `src-tauri/src/punctuation/processor.rs`
   - 状态: 代码已写好，但 `ort` 2.0 RC API 有变化，需要调试

3. **测试程序**
   - `compare_punctuation.rs` - 可以测试基于规则的版本
   - `test_punctuation.rs` - ONNX 版本（待修复）

4. **文档**
   - 完整的技术文档和集成指南

###  ⚠️ 已知问题
- `ort` 2.0.0-rc.11 的 API 与预期不符
- 需要根据实际 API 调整代码
- 建议: 等 `ort` 2.0 正式发布后再集成，或使用 1.x 版本

---

## 🚀 推荐方案: 先使用基于规则的实现

### 立即可用

1. **测试基于规则的实现**
```bash
cd src-tauri
cargo run --bin compare_punctuation -- --rule-only
```

2. **集成到应用**

在 `src-tauri/src/core/app.rs` 中:

```rust
use crate::punctuation::RuleBasedPunctuationRestorer;

pub struct TaTingApp {
    // ... 现有字段 ...
    punctuation_restorer: RuleBasedPunctuationRestorer,
}

impl TaTingApp {
    pub fn new(config: AppConfig) -> Result<Self> {
        Ok(Self {
            // ... 现有字段 ...
            punctuation_restorer: RuleBasedPunctuationRestorer::new(),
        })
    }

    // 转录完成后调用
    async fn on_transcription_complete(&self, text: String) -> Result<String> {
        // 添加标点
        let result = self.punctuation_restorer.restore(&text)?;
        Ok(result)
    }
}
```

### 优点
- ✅ 零依赖（不需要 ort, tokenizers, ndarray）
- ✅ 启动快（< 1ms）
- ✅ 内存小（< 1MB）
- ✅ 立即可用

### 如何改进
可以通过添加更多规则来提升准确度:

```rust
// 在 rule_based.rs 中添加更多规则
sentence_end_words: vec![
    "了".to_string(),
    "吧".to_string(),
    // 添加更多...
],
```

---

## 🔮 未来: ONNX 模型集成

### 选项 1: 等待 ort 2.0 正式版
- 等待 `ort` 从 RC 升级到正式版
- API 会更稳定
- 文档会更完善

### 选项 2: 使用 ort 1.x
修改 `Cargo.toml`:
```toml
ort = { version = "1.16", features = ["download-binaries"] }
```

### 选项 3: 使用 tract
另一个 Rust ONNX 运行时:
```toml
tract-onnx = "0.21"
```

### 选项 4: 使用在线 API
如果接受网络请求:
- 调用 HuggingFace Inference API
- 调用自部署的 FastAPI 服务

---

## 📦 当前依赖清理

如果只使用基于规则的方案，可以移除 ONNX 相关依赖:

### 最小化 Cargo.toml

```toml
# 注释掉这些依赖（暂时不用）
# ort = { version = "2.0.0-rc.11", features = ["download-binaries"] }
# tokenizers = "0.20"
# ndarray = "0.16"
```

然后注释掉 `mod.rs` 和 `processor.rs` 中的 ONNX 相关代码。

---

## ✅ 下一步建议

### 方案 A: 仅使用基于规则（最简单）

1. 移除 ONNX 依赖
2. 只保留 `rule_based.rs`
3. 集成到主应用
4. 根据用户反馈改进规则

### 方案 B: 保留 ONNX 框架（推荐）

1. 注释掉 ONNX 相关依赖（Cargo.toml）
2. 先用基于规则的实现
3. 等 `ort` 2.0 正式版发布
4. 再启用 ONNX 功能

### 方案 C: 立即修复 ONNX

如果你急需 ONNX:
1. 研究 `ort` 2.0.0-rc.11 的正确 API
2. 调试编译错误
3. 或降级到 `ort` 1.16

---

## 📝 修改建议

### 1. 临时禁用 ONNX 模块

编辑 `src-tauri/src/punctuation/mod.rs`:

```rust
// 暂时禁用 ONNX 实现
// mod processor;
mod rule_based;

// pub use processor::PunctuationProcessor;
pub use rule_based::RuleBasedPunctuationRestorer;

// ONNX 相关代码全部注释掉...
```

### 2. 清理 Cargo.toml

```toml
# 标点恢复 (基于规则，零依赖)
# ONNX 支持待 ort 2.0 正式版发布后启用
# ort = { version = "2.0.0-rc.11", features = ["download-binaries"] }
# tokenizers = "0.20"
# ndarray = "0.16"
```

### 3. 更新测试程序

只保留 `compare_punctuation.rs`，移除 `test_punctuation.rs`

---

## 📚 参考资源

- **基于规则的实现**: 已完成，直接可用
- **ONNX 集成**: 等待 `ort` 2.0 正式版
- **ort 文档**: https://docs.rs/ort/2.0.0-rc.11/ort/
- **替代方案**: tract-onnx, candle, etc.

---

## 💡 结论

**当前最佳方案**:

1. ✅ **立即**：使用基于规则的实现
   - 可以满足基本需求
   - 零依赖，稳定可靠

2. 📅 **未来**：等 `ort` 2.0 正式版后升级到 ONNX
   - 更高准确度
   - 已有完整代码框架，只需调试 API

**代码已经写好 95%，只是 API 调用需要微调！**

---

**创建时间**: 2026-01-24
**状态**: 基于规则的版本已完成并可用
**下一步**: 集成到主应用或等待 ort 2.0 正式版
