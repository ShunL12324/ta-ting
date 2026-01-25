# 标点恢复功能 - 命令速查表

## 🚀 快速测试（推荐先运行这个）

```bash
# 测试基于规则的实现（无需任何依赖）
cd src-tauri
cargo run --bin compare_punctuation -- --rule-only
```

## 📦 完整功能测试（需要下载模型）

### 步骤 1: 安装 Python 依赖

```bash
pip install optimum[onnxruntime] transformers
```

### 步骤 2: 下载 ONNX 模型

```bash
python download_punctuation_model.py
```

### 步骤 3: 测试 ONNX 模型

```bash
cd src-tauri
cargo run --bin test_punctuation
```

### 步骤 4: 对比两种方案

```bash
cargo run --bin compare_punctuation
```

## 📁 生成的文件

- `src-tauri/src/punctuation/` - 核心模块
  - `mod.rs` - ONNX 引擎
  - `processor.rs` - 异步包装器
  - `rule_based.rs` - 基于规则的实现

- `src-tauri/src/bin/` - 测试程序
  - `test_punctuation.rs` - ONNX 测试
  - `compare_punctuation.rs` - 对比测试

- `docs/` - 文档
  - `PUNCTUATION_RESTORATION.md` - 详细技术文档
  - `PUNCTUATION_IMPLEMENTATION_SUMMARY.md` - 实现总结

- `download_punctuation_model.py` - 模型下载脚本

## 📚 查看文档

- 快速开始: `PUNCTUATION_README.md`
- 详细文档: `docs/PUNCTUATION_RESTORATION.md`
- 实现总结: `docs/PUNCTUATION_IMPLEMENTATION_SUMMARY.md`

## ✅ 推荐流程

1. **先试基于规则的方案**（最简单）
   ```bash
   cargo run --bin compare_punctuation -- --rule-only
   ```

2. **如果需要更高准确度，下载 ONNX 模型**
   ```bash
   pip install optimum[onnxruntime] transformers
   python download_punctuation_model.py
   ```

3. **对比两种方案**
   ```bash
   cargo run --bin compare_punctuation
   ```

## 🔧 集成到应用

查看 `docs/PUNCTUATION_IMPLEMENTATION_SUMMARY.md` 中的"集成到 TaTing 主应用"部分。
