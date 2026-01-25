# TaTing (踏听)

> 完全离线的 AI 听写输入法 | Offline AI Dictation Input Method

基于 **Tauri 2.x + Rust + Sherpa-ONNX** 构建的跨平台离线语音输入工具。

---

## ✨ 特性

- 🔒 **完全离线** - 所有转录在本地完成，保护隐私
- ⚡ **极速转录** - 使用 Sherpa-ONNX，比 Whisper 快 5-10 倍
- 🎯 **纯 CPU** - 无需 GPU，所有设备都能流畅运行
- 🎤 **即按即用** - Ctrl+Shift+V 开始/停止录音
- 💎 **原生体验** - 系统托盘集成，轻量启动

---

## 🚀 快速开始

### 1. 下载模型文件

模型文件较大（约 248MB），不包含在 Git 仓库中，需要单独下载：

```bash
# Windows PowerShell
.\scripts\download-models.ps1

# Linux/macOS
./scripts/download-models.sh
```

或手动下载：
- 访问 [Sherpa-ONNX Releases](https://github.com/k2-fsa/sherpa-onnx/releases)
- 下载 `sherpa-onnx-zipformer-multi-zh-hans-2023-9-2.tar.bz2`
- 解压到 `src-tauri/resources/models/sherpa-zh/`

### 2. 测试后端（无需前端）

```bash
cd src-tauri
cargo run --release --bin test_tating
```

按 **Ctrl+Shift+V** 开始录音，再次按下停止并转录。

### 3. 完整开发模式

```bash
npm install
npm run tauri dev
```

---

## 📦 当前状态

**版本**: v0.1.0-alpha
**阶段**: Phase 1 完成 ✅ + 前后端集成完成 🎉

### ✅ 已完成功能

**核心功能**:
- Sherpa-ONNX 离线转录（ZipFormer 中文模型）
- 全局热键 (Ctrl+Shift+V)
- 实时音频录制和波形显示
- 自动标点符号恢复
- 键盘输入模拟和自动粘贴
- 系统托盘集成

**前后端集成**:
- Tauri Commands（get_current_state, trigger_hotkey）
- 事件系统（state_changed, transcription_result, error）
- 录音指示器窗口（实时波形动画）
- 设置面板 UI

### ⏳ 待完成

- [ ] 实时波形优化
- [ ] 转录进度条
- [ ] 多模型支持
- [ ] 自定义热键
- [ ] 设置持久化

---

## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| UI框架 | Tauri 2.x |
| 后端 | Rust |
| 前端 | React + TypeScript + Vite |
| 语音识别 | **Sherpa-ONNX (ZipFormer)** |
| 音频 | cpal |
| 热键 | global-hotkey |
| 状态管理 | Zustand |

---

## 📖 文档

详细项目文档请查看 **[CLAUDE.md](./CLAUDE.md)**

---

## 🧪 性能测试

**实测数据**（Windows 11, Intel i5）:
- 录音时长: 1.76 秒
- 转录时间: 2-5 秒
- 实时因子: 1-3x ⚡
- 准确度: 优秀（中文）

---

## 📄 许可证

MIT License

---

**最后更新**: 2026-01-24
