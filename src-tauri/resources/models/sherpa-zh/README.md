# Sherpa-ONNX 中文语音识别模型

## 当前使用模型

**sherpa-onnx-zipformer-multi-zh-hans-2023-9-2**

- **类型**: Offline ZipFormer (非流式)
- **大小**: 248MB
- **语言**: 简体中文
- **性能**: CPU 友好，比 Whisper 快 5-10 倍
- **来源**: https://github.com/k2-fsa/sherpa-onnx

## 模型文件

```
sherpa-onnx-zipformer-multi-zh-hans-2023-9-2/
├── encoder-epoch-20-avg-1.onnx  (248MB)
├── decoder-epoch-20-avg-1.onnx  (5.0MB)
├── joiner-epoch-20-avg-1.onnx   (4.0MB)
├── tokens.txt                   (19KB)
└── bpe.model                    (258KB)
```

## 使用说明

该模型已集成到 `SherpaEngine`，在 `src-tauri/src/whisper/sherpa_engine.rs` 中使用。

## 性能指标

- **实时因子**: 1-3x (1秒音频需要 1-3 秒处理)
- **准确率**: 高（中文优化）
- **内存占用**: ~500MB
- **线程数**: 4 (可配置)

## 下载模型

**注意**: 模型文件由于太大（约 248MB）不包含在 Git 仓库中，需要单独下载。

### 下载地址

```bash
# 使用 wget
wget https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-zipformer-multi-zh-hans-2023-9-2.tar.bz2

# 或使用 curl
curl -LO https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-zipformer-multi-zh-hans-2023-9-2.tar.bz2

# 解压到当前目录
tar xvf sherpa-onnx-zipformer-multi-zh-hans-2023-9-2.tar.bz2
```

### 自动下载脚本

或者运行项目提供的下载脚本：

```bash
# Windows PowerShell
.\scripts\download-models.ps1

# Linux/macOS
./scripts/download-models.sh
```

## 更新记录

- **2026-01-24**: 从 Whisper 迁移到 Sherpa-ONNX
  - 移除了 streaming 模型（不兼容 offline API）
  - 使用 offline 模型配合 ZipFormer API
  - 性能提升 5-10 倍
