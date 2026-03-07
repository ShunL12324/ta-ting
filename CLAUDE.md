# TaTing - Offline AI Dictation IME

Tauri + Rust + Sherpa-ONNX cross-platform offline dictation input method.

---

## Status

Version: v0.1.0-alpha | Phase 1 + frontend/backend integration complete

Done:
- Global hotkey (Ctrl+Shift+V)
- Audio recording (cpal)
- Sherpa-ONNX offline transcription (ZipFormer, 5-10x faster than Whisper)
- Auto-paste to cursor (clipboard + keyboard simulation)
- System tray
- State machine (idle / recording / transcribing / inputting)
- Recording indicator UI
- Settings panel UI (framework only)
- Tauri commands: `get_current_state`, `trigger_hotkey`
- Backend events: `state_changed`, `transcription_result`, `error`
- Frontend event listeners + Zustand state sync
- CI/CD (GitHub Actions build + release workflows)
- Auto-update (tauri-plugin-updater)

Next (Phase 2):
- Real-time waveform display
- VAD auto-stop
- Multi-model support (int8 quantized)
- Custom hotkey
- Settings persistence

---

## Tech Stack

| Layer | Tech |
|-------|------|
| Framework | Tauri 2.x |
| Backend | Rust |
| Frontend | React + TypeScript + Vite |
| ASR | Sherpa-ONNX ZipFormer (sherpa-rs) |
| Audio | cpal |
| Hotkey | global-hotkey |
| Input simulation | enigo + arboard |
| State management | Zustand |

---

## Project Structure

```
ta-ting/
├── src-tauri/src/
│   ├── lib.rs              # Tauri setup, hotkey registration, window management
│   ├── main.rs
│   ├── core/
│   │   ├── app.rs          # Main controller, unified handle_hotkey(Option<AppHandle>)
│   │   └── state_machine.rs
│   ├── audio/recorder.rs   # cpal recording
│   ├── asr/sherpa_engine.rs # Sherpa-ONNX transcription
│   ├── system/
│   │   ├── hotkey.rs       # HotkeyManager (Ctrl+Shift+V)
│   │   ├── input.rs        # Keyboard simulation
│   │   ├── clipboard.rs
│   │   └── tray.rs
│   ├── punctuation/rule_based.rs  # Rule-based punctuation restoration
│   ├── config/settings.rs  # App settings (stub)
│   └── bin/
│       ├── test_tating.rs  # Full end-to-end test (use this)
│       ├── compare_punctuation.rs
│       ├── test_audio.rs   # Disabled in Cargo.toml
│       └── test_input.rs   # Disabled in Cargo.toml
├── src/
│   ├── main.tsx            # Entry point + event listeners
│   ├── recording-main.tsx  # Recording window entry
│   ├── App.tsx
│   ├── components/
│   │   ├── RecordingIndicator.tsx
│   │   ├── SettingsPanel.tsx
│   │   ├── UpdateChecker.tsx
│   │   └── ui/             # shadcn/ui
│   ├── pages/RecordingWindow.tsx
│   └── stores/appStore.ts
├── index.html              # Main window entry
├── recording.html          # Recording window entry (loads recording-main.tsx)
├── scripts/
│   ├── download-models.sh
│   └── download-models.ps1
└── .github/workflows/
    ├── build.yml           # CI on push to master
    └── release.yml         # Release on version tag
```

---

## Model

Model: `sherpa-onnx-zipformer-multi-zh-hans-2023-9-2` (248MB, Chinese)
Location: `src-tauri/resources/models/sherpa-zh/` (not in git, download via script)

Path resolution at runtime uses `tauri::path::BaseDirectory::Resource` — do not hardcode.

---

## Build Commands

```bash
# Dev
npm run tauri dev

# Release build
npm run tauri build

# Test backend only (no Tauri context)
cd src-tauri && cargo run --release --bin test_tating

# Tests
cd src-tauri && cargo test
```

---

## Key Implementation Notes

**Hotkey**: `handle_hotkey(handle: Option<AppHandle>)` — pass `Some(handle)` from Tauri context, `None` from test binaries. Events are only emitted when handle is `Some`.

**Hotkey manager lifetime**: Stored in `static HOTKEY_MANAGER: OnceLock<GlobalHotKeyManager>` to prevent drop.

**Audio callback logs**: Use `log::debug!` (not `info!`) inside the cpal callback — runs at ~100Hz.

**ONNX punctuation**: Disabled pending `ort` crate v2.0 stable. Only rule-based punctuation is active.
