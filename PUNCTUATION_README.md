# 标点恢复功能 - 快速开始

## 1. 安装依赖

```bash
pip install optimum[onnxruntime] transformers
```

## 2. 下载模型

```bash
python download_punctuation_model.py
```

## 3. 测试

```bash
cd src-tauri
cargo run --bin test_punctuation
```

## 详细文档

查看 [docs/PUNCTUATION_RESTORATION.md](docs/PUNCTUATION_RESTORATION.md)
