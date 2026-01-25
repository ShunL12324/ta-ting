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

### 测试后端（无需前端）

```bash
cd src-tauri
cargo run --release --bin test_tating
```

按 **Ctrl+Shift+V** 开始录音，再次按下停止并转录。

### 完整开发模式

```bash
npm install
npm run tauri dev
```

---

## 📦 当前状态

**版本**: v0.1.0-alpha
**阶段**: Phase 1 完成 ✅
**下一步**: 前后端集成

### ✅ 已完成功能

**后端 (Rust)**:
- Sherpa-ONNX 语音识别引擎
- 全局热键 (Ctrl+Shift+V)
- 音频录制 (cpal)
- 键盘输入模拟 (enigo + arboard)
- 状态机管理
- 系统托盘

**前端 (React)**:
- 录音指示器 UI
- 设置面板
- Zustand 状态管理

### ⏳ 待完成

- [ ] Tauri Commands（前后端通信）
- [ ] 状态同步
- [ ] 完整流程集成测试

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
