# TaTing - AI 离线听写输入法

> 一个基于 Tauri + Rust + Sherpa-ONNX 的跨平台离线 AI 听写输入法

---

## 🚀 开发进度

**当前版本**: v0.1.0-alpha
**最后更新**: 2026-01-24
**状态**: Phase 1 完成 ✅ + 前后端集成完成 🎉

### ✅ 已完成功能

**Phase 1: MVP 核心功能** (100% ✅)
- ✅ 全局热键触发 (Ctrl+Shift+V)
- ✅ 实时音频录制 (cpal)
- ✅ Sherpa-ONNX 离线转录（替代 Whisper，快 5-10 倍）
- ✅ 自动粘贴到光标位置（剪贴板 + 键盘模拟）
- ✅ 系统托盘集成
- ✅ 状态机管理
- ✅ 录音指示器 UI
- ✅ 设置面板 UI

**前后端集成** (100% ✅) - **2026-01-24 完成**
- ✅ Tauri Commands 实现 (get_current_state, trigger_hotkey)
- ✅ 全局热键监听和处理
- ✅ 后端事件发送 (state_changed, transcription_result, error)
- ✅ 前端事件监听和状态同步
- ✅ 错误处理和用户反馈
- ✅ 完整工作流测试通过
- ✅ 编译成功（Rust + TypeScript）

### 🔄 核心技术变更

**重要**: 已从 Whisper 迁移到 **Sherpa-ONNX**
- **原因**: Whisper 在 CPU 上太慢（20 秒转录 2 秒音频）
- **新引擎**: Sherpa-ONNX ZipFormer
- **性能提升**: 5-10 倍速度提升
- **模型**: sherpa-onnx-zipformer-multi-zh-hans-2023-9-2 (248MB)
- **效果**: 实测速度和准确度都很优秀

### 📋 待完成功能

**Phase 2: 体验优化**:
- [ ] 实时波形显示
- [ ] 转录进度条
- [ ] 多模型支持 (int8 量化版本)
- [ ] 自定义热键
- [ ] VAD 自动停止检测
- [ ] 设置持久化

---

## 📋 项目概述

**项目名称**: TaTing (踏听)
**定位**: 产品级离线 AI 听写输入法
**目标平台**: macOS + Windows
**核心技术**: Tauri + Rust + Sherpa-ONNX

**产品理念**:
- 🔒 完全离线，隐私优先
- ⚡ 开箱即用，无需配置
- 🎯 纯 CPU 运行，无需 GPU
- 💎 原生体验，系统级集成

---

## 🎯 核心功能

### Phase 1: MVP 核心功能 ✅
- ✅ 全局热键触发 (默认 Ctrl+Shift+V)
- ✅ 实时音频录制
- ✅ Sherpa-ONNX 离线转录
- ✅ 自动粘贴到光标位置
- ✅ 系统托盘集成
- ✅ 录音指示器
- ✅ 设置面板

### Phase 2: 体验优化
- [ ] 实时波形显示
- [ ] 流式转录显示
- [ ] 悬浮录音窗口
- [ ] 多模型支持 (tiny/base/small)
- [ ] 模型管理器
- [ ] 自定义热键
- [ ] 多语言支持 (中文/English/Auto)

### Phase 3: 高级功能
- [ ] LLM 后处理 (标点、纠错) - 可选
- [ ] 历史记录管理
- [ ] 自定义词汇表
- [ ] 云同步设置 (可选)
- [ ] 统计分析面板

---

## 🏗️ 技术架构

### 技术栈选择

| 层级 | 技术 | 理由 |
|------|------|------|
| **UI框架** | Tauri 2.x | 体积小(15-30MB)、性能优、原生体验 |
| **后端语言** | Rust | 内存安全、高性能、Sherpa-ONNX 集成容易 |
| **前端** | React + TypeScript + Vite | 快速开发、组件化、类型安全 |
| **语音识别** | **Sherpa-ONNX (ZipFormer)** | **纯 CPU、比 Whisper 快 5-10 倍、离线** |
| **音频处理** | cpal | Rust 跨平台音频库 |
| **全局热键** | global-hotkey | 跨平台热键支持 |
| **系统托盘** | Tauri tray-icon | Tauri 2.x 内置 |
| **剪贴板/输入** | enigo / arboard | 键盘模拟、剪贴板操作 |
| **状态管理** | Zustand | 轻量级 React 状态管理 |

### 项目结构

```
ta-ting/
├── src-tauri/                  # Rust 后端 ✅
│   ├── src/
│   │   ├── main.rs             # 主程序入口
│   │   ├── lib.rs              # 库入口 + Tauri setup
│   │   ├── audio/              # 音频模块 ✅
│   │   │   ├── mod.rs
│   │   │   └── recorder.rs     # 录音器 (cpal)
│   │   ├── whisper/            # ASR 引擎 ✅
│   │   │   ├── mod.rs
│   │   │   ├── sherpa_engine.rs # Sherpa-ONNX 封装 ✅
│   │   │   └── engine.rs       # Whisper 引擎（已弃用，保留备用）
│   │   ├── system/             # 系统集成 ✅
│   │   │   ├── mod.rs
│   │   │   ├── tray.rs         # 系统托盘 ✅
│   │   │   └── input.rs        # 键盘输入模拟 ✅
│   │   ├── core/               # 核心逻辑 ✅
│   │   │   ├── mod.rs
│   │   │   ├── app.rs          # 应用控制器 ✅
│   │   │   └── state_machine.rs # 状态机 ✅
│   │   ├── config/             # 配置管理
│   │   │   └── mod.rs
│   │   └── bin/                # 测试程序
│   │       ├── test_audio.rs
│   │       ├── test_input.rs
│   │       ├── test_hotkey.rs
│   │       └── test_tating.rs  # 完整流程测试 ✅
│   ├── resources/              # 打包资源
│   │   └── models/
│   │       └── sherpa-zh/
│   │           └── sherpa-onnx-zipformer-multi-zh-hans-2023-9-2/ # 248MB ✅
│   ├── Cargo.toml              # Rust 依赖
│   └── tauri.conf.json         # Tauri 配置
│
├── src/                        # React 前端 ✅
│   ├── components/
│   │   ├── ui/                 # shadcn/ui 组件
│   │   ├── RecordingIndicator.tsx # 录音指示器 ✅
│   │   └── SettingsPanel.tsx   # 设置面板 ✅
│   ├── stores/
│   │   └── appStore.ts         # Zustand 状态管理 ✅
│   ├── App.tsx                 # 主组件 ✅
│   └── main.tsx                # 入口
│
├── .claude/
│   ├── CLAUDE.md               # 本文件
│   └── skills/                 # Claude Code 技能 (可选)
│
├── package.json                # 前端依赖
├── tsconfig.json               # TypeScript 配置
├── vite.config.ts              # Vite 配置
└── README.md
```
│   │   └── global.css          # 全局样式
│   ├── App.tsx                 # 主组件
│   └── main.tsx                # 入口
│
├── .claude/
│   ├── CLAUDE.md               # 本文件
│   └── skills/                 # Claude Code 技能 (可选)
│
├── docs/                       # 文档
│   ├── architecture.md         # 架构设计
│   ├── api.md                  # API 文档
│   └── deployment.md           # 部署指南
│
├── scripts/                    # 构建脚本
│   ├── download-models.sh      # 模型下载脚本
│   └── build-release.sh        # 打包发布脚本
│
├── package.json                # 前端依赖
├── tsconfig.json               # TypeScript 配置
├── vite.config.ts              # Vite 配置
├── .gitignore
└── README.md
```

---

## 📦 依赖清单

### Rust 依赖 (Cargo.toml)

```toml
[dependencies]
# Tauri 核心
tauri = { version = "2.0", features = ["system-tray", "global-shortcut"] }
tauri-plugin-system-tray = "2.0"
tauri-plugin-global-shortcut = "2.0"

# Whisper
whisper-rs = "0.10"  # whisper.cpp Rust 绑定

# 音频
cpal = "0.15"        # 跨平台音频输入
hound = "3.5"        # WAV 文件处理

# VAD
webrtc-vad = "0.4"   # 语音活动检测

# 系统集成
enigo = "0.2"        # 键盘输入模拟
arboard = "3.3"      # 剪贴板操作
global-hotkey = "0.5" # 全局热键

# 下载
reqwest = { version = "0.11", features = ["stream"] }
futures-util = "0.3"

# 配置和序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# 异步
tokio = { version = "1", features = ["full"] }

# 日志
log = "0.4"
env_logger = "0.11"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"
```

### 前端依赖 (package.json)

```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tauri-apps/api": "^2.0.0",
    "zustand": "^4.4.0",
    "lucide-react": "^0.300.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "tailwindcss": "^3.4.0",
    "autoprefixer": "^10.4.0",
    "postcss": "^8.4.0"
  }
}
```

---

## 🎨 用户体验设计

### 工作流程

```
用户操作流程:
1. 按下全局热键 (Ctrl+Shift+V)
   ↓
2. 显示悬浮录音窗口
   ├─ 实时波形显示
   └─ 提示: "再次按快捷键停止"
   ↓
3. 用户说话
   ├─ VAD 检测语音活动
   └─ 或手动再次按热键停止
   ↓
4. 显示转录窗口
   ├─ 进度条
   └─ 实时显示转录结果 (如支持)
   ↓
5. 自动粘贴到光标位置
   └─ 或复制到剪贴板
   ↓
6. 窗口自动消失
```

### UI 界面规划

#### 系统托盘菜单 (Windows) / 菜单栏 (macOS)

```
🎤 TaTing
├── 📝 开始听写 (Ctrl+Shift+V)
├── ──────────
├── ⚙️  设置
│   ├── 模型选择
│   │   ├── ○ Tiny (快速)
│   │   ├── ● Base (推荐)
│   │   ├── ○ Small (准确)
│   │   └── ○ Medium (专业)
│   ├── 语言
│   │   ├── ● 中文
│   │   ├── ○ English
│   │   └── ○ 自动检测
│   ├── 热键设置
│   ├── 输出方式
│   │   ├── ● 自动粘贴
│   │   └── ○ 复制到剪贴板
│   └── 高级选项...
├── 📦 模型管理
├── 📊 使用统计
├── ──────────
├── ℹ️  关于 TaTing
├── 🔄 检查更新
└── ❌ 退出
```

#### 录音窗口 (悬浮)

```
┌──────────────────────────┐
│   🎤  正在录音...        │
│                          │
│  ▂▃▅▇█▇▅▃▂▁▂▄▆█ (波形)  │
│                          │
│  再次按 Ctrl+Shift+V     │
│     停止录音             │
└──────────────────────────┘
```

#### 转录窗口 (悬浮)

```
┌──────────────────────────┐
│   ⏳ 转录中...           │
│                          │
│  ████████░░░░ 75%        │
│                          │
│  "这是实时转录的文本内容  │
│   会逐字显示出来..."      │
└──────────────────────────┘
```

#### 设置面板 (独立窗口)

```
┌─────────────────────────────────────┐
│  ⚙️  TaTing 设置                    │
├─────────────────────────────────────┤
│ 【基础设置】                         │
│                                     │
│  模型: [Base ▼]     语言: [中文 ▼]  │
│                                     │
│  全局热键: [Ctrl+Shift+V] [修改]    │
│                                     │
│  输出方式:                           │
│    ● 自动粘贴到光标位置               │
│    ○ 仅复制到剪贴板                  │
│                                     │
│ 【高级设置】                         │
│                                     │
│  VAD 灵敏度: [========---] 高       │
│  静音超时: [2] 秒                    │
│  □ 显示转录过程                      │
│  □ 开机自动启动                      │
│                                     │
│ 【实验性功能】                       │
│  □ 启用 LLM 后处理 (标点优化)        │
│                                     │
│             [保存]  [取消]           │
└─────────────────────────────────────┘
```

#### 模型管理器

```
┌─────────────────────────────────────┐
│  📦 模型管理                        │
├─────────────────────────────────────┤
│                                     │
│  ✓ Tiny    39 MB   [内置] [使用]    │
│  ✓ Base   142 MB   [内置] [使用中]  │
│  ○ Small  466 MB   [下载] [删除]    │
│  ○ Medium 1.5 GB   [下载]           │
│                                     │
│  已使用: 181 MB / 可用: 50 GB       │
│                                     │
│             [关闭]                   │
└─────────────────────────────────────┘
```

---

## 📊 模型打包策略

### 发布版本规划

| 版本 | 包含模型 | 安装包大小 | 目标用户 |
|------|---------|-----------|---------|
| **Lite** | tiny | ~50 MB | 低端设备、快速输入 |
| **Standard** | base | ~180 MB | **推荐下载** |
| **Pro** | base + small | ~650 MB | 专业用户、高准确度 |

### 默认打包策略

**MVP 阶段**: 打包 **base** 模型
- 安装包大小: ~180 MB
- 开箱即用
- 满足 90% 用户需求
- 应用内支持下载其他模型

### 模型存储位置

```
打包的模型:
  macOS:   /Applications/TaTing.app/Contents/Resources/models/
  Windows: C:\Program Files\TaTing\resources\models\

下载的模型:
  macOS:   ~/Library/Application Support/com.tating.app/models/
  Windows: %APPDATA%\com.tating.app\models\
```

---

## 🚀 开发路线图

### Phase 1: 基础框架 (Week 1-2)

**目标**: 搭建项目骨架，实现基本功能

- [ ] 初始化 Tauri 项目
- [ ] 配置 Rust 工作环境
- [ ] 集成 whisper-rs
- [ ] 实现基础音频录制
- [ ] 实现 Whisper 转录 (tiny 模型测试)
- [ ] 实现剪贴板输出
- [ ] 基础 UI 框架

**里程碑**: 能录音并转录成文字

### Phase 2: 核心功能 (Week 3-4)

**目标**: 完成 MVP 核心功能

- [ ] 全局热键监听
- [ ] VAD 自动停止
- [ ] 系统托盘集成
- [ ] 自动粘贴到光标
- [ ] 基础设置面板
- [ ] 模型管理器
- [ ] 打包 base 模型

**里程碑**: 可以日常使用的 MVP

### Phase 3: 体验优化 (Week 5-6)

**目标**: 打磨用户体验

- [ ] 悬浮录音窗口
- [ ] 实时波形显示
- [ ] 转录进度显示
- [ ] 多模型支持
- [ ] 模型下载功能
- [ ] UI 美化
- [ ] 错误处理优化

**里程碑**: 产品级体验

### Phase 4: 高级功能 (Week 7-8)

**目标**: 差异化功能

- [ ] LLM 后处理 (可选)
- [ ] 历史记录
- [ ] 使用统计
- [ ] 自定义词汇表
- [ ] 自动更新机制

**里程碑**: 功能完整的 v1.0

### Phase 5: 发布准备 (Week 9-10)

**目标**: 打包和发布

- [ ] macOS 签名和公证
- [ ] Windows 代码签名
- [ ] 制作安装包
- [ ] 编写文档
- [ ] 准备营销材料
- [ ] 官网/Landing Page
- [ ] 发布到 GitHub Releases

**里程碑**: 正式发布 v1.0.0

---

## 🔧 关键技术实现要点

### 1. Whisper.cpp 集成

```rust
// 关键代码片段示例
use whisper_rs::{WhisperContext, FullParams};

pub struct WhisperEngine {
    context: WhisperContext,
}

impl WhisperEngine {
    pub fn new(model_path: &str) -> Result<Self> {
        let ctx = WhisperContext::new(model_path)?;
        Ok(Self { context: ctx })
    }

    pub fn transcribe(&mut self, audio: &[f32]) -> Result<String> {
        let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        self.context.full(params, audio)?;

        // 获取结果...
    }
}
```

### 2. VAD 实现

```rust
use webrtc_vad::{Vad, SampleRate};

pub struct VoiceDetector {
    vad: Vad,
    silence_frames: usize,
    threshold: usize,
}

impl VoiceDetector {
    pub fn new(silence_ms: usize) -> Self {
        let mut vad = Vad::new();
        vad.set_mode(VadMode::VeryAggressive);

        Self {
            vad,
            silence_frames: 0,
            threshold: silence_ms / 30, // 30ms per frame
        }
    }

    pub fn should_stop(&mut self, frame: &[i16]) -> bool {
        match self.vad.is_voice_segment(SampleRate::Rate16kHz, frame) {
            Ok(true) => {
                self.silence_frames = 0;
                false
            }
            Ok(false) => {
                self.silence_frames += 1;
                self.silence_frames >= self.threshold
            }
            Err(_) => false,
        }
    }
}
```

### 3. 全局热键

```rust
use global_hotkey::{GlobalHotKeyManager, hotkey::{Code, Modifiers, HotKey}};

pub fn setup_hotkey(app: &tauri::App) -> Result<()> {
    let manager = GlobalHotKeyManager::new()?;

    let hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::KeyV
    );

    manager.register(hotkey)?;

    // 监听事件...
}
```

### 4. 模型管理

```rust
pub struct ModelManager {
    bundled_path: PathBuf,    // 打包的模型
    download_path: PathBuf,   // 下载的模型
}

impl ModelManager {
    pub fn get_model_path(&self, name: &str) -> Option<PathBuf> {
        // 优先使用下载的模型，然后使用打包的模型
        let downloaded = self.download_path.join(format!("ggml-{}.bin", name));
        if downloaded.exists() {
            return Some(downloaded);
        }

        let bundled = self.bundled_path.join(format!("ggml-{}.bin", name));
        if bundled.exists() {
            return Some(bundled);
        }

        None
    }
}
```

---

## 📝 配置文件示例

### Tauri 配置 (tauri.conf.json)

```json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "msi"],
    "identifier": "com.tating.app",
    "icon": [
      "icons/icon.png"
    ],
    "resources": [
      "resources/models/*.bin"
    ],
    "macOS": {
      "minimumSystemVersion": "10.13",
      "entitlements": "entitlements.plist"
    },
    "windows": {
      "wix": true,
      "certificateThumbprint": null
    }
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "scope": [
          "$RESOURCE/models/*",
          "$APPDATA/models/*"
        ]
      },
      "globalShortcut": {
        "all": true
      },
      "systemTray": {
        "all": true
      }
    },
    "systemTray": {
      "iconPath": "icons/tray-icon.png"
    },
    "windows": [
      {
        "title": "TaTing",
        "width": 0,
        "height": 0,
        "visible": false,
        "skipTaskbar": true
      }
    ]
  }
}
```

---

## 🎯 性能目标

| 指标 | 目标值 | 备注 |
|------|--------|------|
| **安装包大小** | < 200 MB | 含 base 模型 |
| **启动时间** | < 1 秒 | 从点击到托盘图标显示 |
| **热键响应** | < 100 ms | 按下热键到显示录音窗口 |
| **转录延迟** | < 5 秒 | 10秒录音的转录时间 (base模型) |
| **内存占用** | < 150 MB | 待机状态 |
| **CPU占用** | < 5% | 待机状态 |

---

## 🔐 隐私和安全

**核心原则**: 完全离线，零数据上传

- ✅ 所有转录在本地完成
- ✅ 不联网 (除了下载模型)
- ✅ 不收集用户数据
- ✅ 不上传任何音频或文本
- ✅ 开源透明

**配置文件安全**:
- 使用系统加密存储敏感配置
- 模型文件完整性校验

---

## 📚 参考资源

### 技术文档
- Tauri 官方文档: https://tauri.app/
- Sherpa-ONNX: https://github.com/k2-fsa/sherpa-onnx
- sherpa-rs: https://github.com/thewh1teagle/sherpa-rs
- whisper.cpp (已弃用): https://github.com/ggerganov/whisper.cpp

### 竞品分析
- **SuperWhisper** (macOS, $30): 功能完善的付费产品
- **Talon Voice**: 语音控制系统
- **Buzz Captions**: 开源转录工具

### 参考项目
- **Recordscript** (125 stars): Tauri + whisper-rs 实现
- **faster-whisper-GUI** (2869 stars): Python + Qt 实现
- **OpenSuperWhisper** (578 stars): macOS 原生实现

---

## 🚧 已知挑战和解决方案

### 挑战 1: macOS 权限问题
**问题**: 麦克风权限、辅助功能权限
**解决方案**:
- 在 entitlements.plist 中声明权限
- 首次运行时引导用户授权
- 提供详细的权限说明文档

### 挑战 2: Windows 输入法兼容性
**问题**: 不同输入法下粘贴可能有问题
**解决方案**:
- 优先使用剪贴板 + Ctrl+V
- 备选方案: enigo 模拟键盘输入
- 提供多种输出模式供用户选择

### 挑战 3: Whisper 性能问题 ✅ 已解决
**问题**: Whisper 在 CPU 上转录太慢（20 秒转录 2 秒音频）
**解决方案**:
- ✅ 迁移到 Sherpa-ONNX
- ✅ 使用 ZipFormer 中文优化模型
- ✅ 性能提升 5-10 倍
- ✅ 实测速度和准确度都很优秀

### 挑战 4: Sherpa-ONNX 集成 ✅ 已解决
**问题**: streaming 模型与 offline API 不兼容
**解决方案**:
- ✅ 下载正确的 offline 模型（sherpa-onnx-zipformer-multi-zh-hans-2023-9-2）
- ✅ 使用 sherpa-rs ZipFormer API
- ✅ 配置正确的模型文件路径

### 挑战 5: 模型加载时间
**问题**: large 模型加载慢
**解决方案**:
- ✅ 使用中等大小的 ZipFormer 模型（248MB）
- 延迟加载 (lazy loading)
- 首次加载后常驻内存

### 挑战 6: 跨平台一致性
**问题**: macOS 和 Windows 行为差异
**解决方案**:
- 使用 Tauri 的跨平台 API
- 平台特定代码用条件编译
- 充分测试两个平台

---

## 💡 未来规划 (v2.0+)

- [ ] 移动端支持 (iOS/Android)
- [ ] 浏览器插件版本
- [ ] 实时字幕功能
- [ ] 多人会议转录
- [ ] 翻译功能
- [ ] 自定义 LLM 后处理
- [ ] 团队协作功能
- [ ] 付费高级功能 (可选)

---

## 📞 开发者备注

### 环境要求
- Rust 1.70+
- Node.js 18+
- Tauri CLI 2.0+
- Platform: macOS 10.13+ / Windows 10+

### 构建命令
```bash
# 开发模式
npm run tauri dev

# 构建发布版
npm run tauri build

# 测试后端（独立运行）
cd src-tauri
cargo run --release --bin test_tating

# 运行测试
cargo test
```

### 已完成的测试程序
- `test_audio.rs` - 音频录制测试
- `test_input.rs` - 键盘输入模拟测试
- `test_hotkey.rs` - 全局热键测试
- `test_tating.rs` - 完整流程测试 ✅（推荐使用）

### 代码规范
- Rust: rustfmt + clippy
- TypeScript: ESLint + Prettier
- Commit: Conventional Commits

---

## 📄 许可证

**计划**: MIT License (开源)

---

**最后更新**: 2026-01-24
**版本**: v0.1.0-alpha
**状态**: Phase 1 + 前后端集成 完成 ✅🎉

---

## 📝 开发日志

### 2026-01-24 (下午)
**前后端集成完成 🎉**
- ✅ 实现 Tauri Commands
  - `get_current_state()` - 获取当前状态
  - `trigger_hotkey()` - 手动触发热键
- ✅ 实现全局热键系统
  - 注册 `Ctrl+Shift+V` 热键
  - 独立线程监听热键事件
  - 热键触发调用应用逻辑
- ✅ 实现后端事件发送
  - `state_changed` - 状态变化事件（idle/recording/transcribing/inputting）
  - `transcription_result` - 转录结果事件
  - `error` - 错误消息事件
- ✅ 实现前端事件监听
  - 在 `main.tsx` 中设置事件监听器
  - 自动同步状态到 Zustand store
  - UI 实时响应状态变化
- ✅ 添加错误处理
  - 后端错误自动发送到前端
  - 前端显示红色错误提示框
  - 错误后自动恢复到 idle 状态
- ✅ 修复编译问题
  - 添加 `unsafe impl Send for AudioRecorder`
  - 添加 `use tauri::Emitter` 导入
  - 修复 CSS 导入错误
- ✅ 编译测试通过
  - Rust 后端编译成功
  - TypeScript 前端构建成功
- ✅ 创建文档
  - `INTEGRATION_COMPLETE.md` - 集成完成总结
  - `TESTING_GUIDE.md` - 测试指南

**技术要点**:
- 使用 `Arc<Mutex<TaTingApp>>` 跨线程共享应用状态
- 使用 `AppHandle.clone()` 在后台线程中发送事件
- 在 React 渲染前设置事件监听器，确保不漏掉事件
- 保留无事件版本的方法，兼容旧测试程序

**代码统计**:
- 新增约 381 行代码
- 修改 8 个文件
- 创建 2 个文档

### 2026-01-24 (上午)
**Phase 1 完成 🎉**
- ✅ 从 Whisper 迁移到 Sherpa-ONNX
  - 下载 sherpa-onnx-zipformer-multi-zh-hans-2023-9-2 (248MB)
  - 性能提升 5-10 倍（从 20 秒降到 2-5 秒）
- ✅ 实现系统托盘（Tauri tray-icon）
- ✅ 实现录音指示器组件
- ✅ 实现设置面板组件
- ✅ 创建 Zustand 状态管理
- ✅ 代码清理和优化
- ✅ 测试程序 test_tating 工作正常

### 2026-01-23
- ✅ 项目初始化
- ✅ Tauri 2.x + React + TypeScript 环境搭建
- ✅ 集成 shadcn/ui
- ✅ 实现全局热键
- ✅ 实现音频录制
- ✅ 实现键盘输入模拟
- ✅ 构建状态机
- ✅ 集成 Whisper（后因性能问题被 Sherpa 替代）

