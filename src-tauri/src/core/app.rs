//! TaTing 应用主控制器
//!
//! 整合所有模块，管理完整的工作流

use crate::audio::recorder::AudioRecorder;
use crate::core::state_machine::{AppState, StateEvent, StateMachine};
use crate::system::input::InputSimulator;
use crate::asr::sherpa_engine::SherpaEngine;
use anyhow::{Context, Result};
use log::{error, info, warn};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};

/// TaTing 应用配置
pub struct AppConfig {
    /// Sherpa 模型目录路径
    pub model_path: String,
    /// 录音采样率（设备自适应）
    pub sample_rate: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model_path: "resources/models/sherpa-zh/sherpa-onnx-zipformer-multi-zh-hans-2023-9-2".to_string(),
            sample_rate: 16000,
        }
    }
}

/// TaTing 应用主控制器
pub struct TaTingApp {
    /// 状态机
    state_machine: Arc<Mutex<StateMachine>>,
    /// 音频录制器
    recorder: Arc<Mutex<Option<AudioRecorder>>>,
    /// Sherpa 引擎
    sherpa: Arc<Mutex<Option<SherpaEngine>>>,
    /// 键盘输入模拟器
    input_simulator: Arc<Mutex<Option<InputSimulator>>>,
    /// 配置
    config: AppConfig,
}

impl TaTingApp {
    /// 创建新的应用实例
    pub fn new(config: AppConfig) -> Result<Self> {
        info!("初始化 TaTing 应用...");

        Ok(Self {
            state_machine: Arc::new(Mutex::new(StateMachine::new())),
            recorder: Arc::new(Mutex::new(None)),
            sherpa: Arc::new(Mutex::new(None)),
            input_simulator: Arc::new(Mutex::new(None)),
            config,
        })
    }

    /// 初始化所有组件
    pub fn initialize(&self) -> Result<()> {
        info!("正在初始化组件...");

        // 1. 初始化 Sherpa 引擎
        info!("加载 Sherpa 模型: {}", self.config.model_path);
        let sherpa = SherpaEngine::new(&self.config.model_path)
            .context("Failed to load Sherpa model")?;
        *self.sherpa.lock().unwrap() = Some(sherpa);
        info!("✅ Sherpa 引擎初始化完成");

        // 2. 初始化音频录制器
        let recorder = AudioRecorder::new().context("Failed to create audio recorder")?;
        *self.recorder.lock().unwrap() = Some(recorder);
        info!("✅ 音频录制器初始化完成");

        // 3. 初始化输入模拟器
        let simulator = InputSimulator::new().context("Failed to create input simulator")?;
        *self.input_simulator.lock().unwrap() = Some(simulator);
        info!("✅ 输入模拟器初始化完成");

        info!("🎉 所有组件初始化完成");
        Ok(())
    }

    /// 获取当前状态
    pub fn current_state(&self) -> AppState {
        self.state_machine.lock().unwrap().current_state()
    }

    /// 处理热键按下事件
    ///
    /// - `handle`: Tauri AppHandle (`Some` for UI context, `None` for tests)
    pub fn handle_hotkey(&self, handle: Option<AppHandle>) -> Result<()> {
        let current_state = self.current_state();
        info!("热键触发，当前状态: {}", current_state);

        if !self.state_machine.lock().unwrap().can_handle_hotkey() {
            warn!("当前状态 {} 不能处理热键", current_state);
            return Ok(());
        }

        match current_state {
            AppState::Idle => {
                self.start_recording(handle)?;
            }
            AppState::Recording => {
                self.stop_recording_and_transcribe(handle)?;
            }
            _ => {
                warn!("意外的状态: {}", current_state);
            }
        }

        Ok(())
    }

    /// 开始录音
    fn start_recording(&self, handle: Option<AppHandle>) -> Result<()> {
        info!("开始录音...");

        self.state_machine
            .lock()
            .unwrap()
            .handle_event(StateEvent::HotkeyPressed)?;

        if let Some(ref h) = handle {
            h.emit("state_changed", "recording")
                .map_err(|e| anyhow::anyhow!("发送事件失败: {}", e))?;
            crate::create_recording_window(h)
                .map_err(|e| anyhow::anyhow!("创建录音窗口失败: {}", e))?;
        }

        let mut recorder_lock = self.recorder.lock().unwrap();
        if let Some(recorder) = recorder_lock.as_mut() {
            if let Some(ref h) = handle {
                let app_handle_clone = h.clone();
                let mut frame_count = 0;

                recorder.set_audio_callback(move |audio_chunk: &[f32]| {
                    // 降低更新频率：每3帧发送一次（让动画更稳定）
                    frame_count += 1;
                    if frame_count % 3 != 0 {
                        return;
                    }

                    // 降采样：每20个采样点取一个
                    let samples: Vec<f32> = audio_chunk.iter()
                        .step_by(20)
                        .copied()
                        .collect();

                    if !samples.is_empty() {
                        if let Some(window) = app_handle_clone.get_webview_window("recording") {
                            let _ = window.emit::<Vec<f32>>("audio_data", samples);
                        }
                    }
                });
            }

            recorder.start_recording()?;
            info!("✅ 录音已开始");
        } else {
            let err_msg = "录音器未初始化";
            if let Some(ref h) = handle {
                let _ = h.emit("error", err_msg);
                crate::close_recording_window(h);
            }
            return Err(anyhow::anyhow!(err_msg));
        }

        Ok(())
    }

    /// 停止录音并开始转录
    fn stop_recording_and_transcribe(&self, handle: Option<AppHandle>) -> Result<()> {
        info!("停止录音...");

        if let Some(ref h) = handle {
            crate::close_recording_window(h);
        }

        let audio_data = {
            let mut recorder_lock = self.recorder.lock().unwrap();
            if let Some(recorder) = recorder_lock.as_mut() {
                let data = recorder.stop_recording()?;
                let sample_rate = recorder.sample_rate();
                let duration_secs = data.len() as f32 / sample_rate as f32;

                info!(
                    "✅ 录音完成: {} 采样点, {} Hz, {:.2}秒",
                    data.len(),
                    sample_rate,
                    duration_secs
                );

                if duration_secs < 0.3 {
                    warn!(
                        "⚠️  录音时间太短 ({:.2}秒)，请说话后再按热键停止",
                        duration_secs
                    );
                    if let Some(ref h) = handle {
                        let err_msg = format!("录音时间太短 ({:.2}秒)", duration_secs);
                        let _ = h.emit("error", err_msg);
                        let _ = h.emit("state_changed", "idle");
                    }
                    self.state_machine.lock().unwrap().reset();
                    return Ok(());
                }

                (data, sample_rate)
            } else {
                let err_msg = "录音器未初始化";
                if let Some(ref h) = handle {
                    let _ = h.emit("error", err_msg);
                }
                return Err(anyhow::anyhow!(err_msg));
            }
        };

        self.state_machine
            .lock()
            .unwrap()
            .handle_event(StateEvent::HotkeyPressed)?;

        if let Some(ref h) = handle {
            h.emit("state_changed", "transcribing")
                .map_err(|e| anyhow::anyhow!("发送事件失败: {}", e))?;
        }

        let state_machine = Arc::clone(&self.state_machine);
        let sherpa = Arc::clone(&self.sherpa);
        let input_simulator = Arc::clone(&self.input_simulator);
        let handle_for_error = handle.clone();

        thread::spawn(move || {
            if let Err(e) = Self::transcribe_and_input(
                state_machine,
                sherpa,
                input_simulator,
                audio_data.0,
                audio_data.1,
                handle,
            ) {
                error!("转录或输入失败: {}", e);
                if let Some(h) = handle_for_error {
                    let _ = h.emit("error", format!("转录失败: {}", e));
                    let _ = h.emit("state_changed", "idle");
                }
            }
        });

        Ok(())
    }

    /// 转录音频并输入文本（在后台线程运行）
    fn transcribe_and_input(
        state_machine: Arc<Mutex<StateMachine>>,
        sherpa: Arc<Mutex<Option<SherpaEngine>>>,
        input_simulator: Arc<Mutex<Option<InputSimulator>>>,
        audio_data: Vec<f32>,
        sample_rate: u32,
        handle: Option<AppHandle>,
    ) -> Result<()> {
        let audio_16k = if sample_rate != 16000 {
            info!("重采样: {} Hz -> 16000 Hz", sample_rate);
            Self::resample_audio(&audio_data, sample_rate, 16000)
        } else {
            audio_data
        };

        info!("开始转录 ({} 采样点)...", audio_16k.len());
        let text = {
            let mut sherpa_lock = sherpa.lock().unwrap();
            if let Some(engine) = sherpa_lock.as_mut() {
                engine.transcribe(&audio_16k)?
            } else {
                let err_msg = "Sherpa 引擎未初始化";
                if let Some(ref h) = handle {
                    let _ = h.emit("error", err_msg);
                }
                return Err(anyhow::anyhow!(err_msg));
            }
        };

        info!("✅ 转录完成: \"{}\"", text);

        if let Some(ref h) = handle {
            h.emit("transcription_result", &text)
                .map_err(|e| anyhow::anyhow!("发送转录结果失败: {}", e))?;
        }

        state_machine
            .lock()
            .unwrap()
            .handle_event(StateEvent::TranscriptionCompleted(text.clone()))?;

        if !text.is_empty() {
            info!("开始输入文本...");

            if let Some(ref h) = handle {
                h.emit("state_changed", "inputting")
                    .map_err(|e| anyhow::anyhow!("发送事件失败: {}", e))?;
            }

            let mut simulator_lock = input_simulator.lock().unwrap();
            if let Some(simulator) = simulator_lock.as_mut() {
                simulator.type_text(&text)?;
                info!("✅ 文本输入完成");
            } else {
                let err_msg = "输入模拟器未初始化";
                if let Some(ref h) = handle {
                    let _ = h.emit("error", err_msg);
                }
                return Err(anyhow::anyhow!(err_msg));
            }
        } else {
            warn!("转录结果为空，跳过输入");
        }

        state_machine
            .lock()
            .unwrap()
            .handle_event(StateEvent::InputCompleted)?;

        if let Some(ref h) = handle {
            h.emit("state_changed", "idle")
                .map_err(|e| anyhow::anyhow!("发送事件失败: {}", e))?;
        }

        info!("🎉 完整流程完成");

        Ok(())
    }

    /// 简单的线性重采样
    fn resample_audio(audio: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
        if from_rate == to_rate {
            return audio.to_vec();
        }

        let ratio = from_rate as f32 / to_rate as f32;
        let new_len = (audio.len() as f32 / ratio) as usize;
        let mut resampled = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let pos = i as f32 * ratio;
            let index = pos as usize;

            if index < audio.len() {
                if index + 1 < audio.len() {
                    let frac = pos - index as f32;
                    let sample = audio[index] * (1.0 - frac) + audio[index + 1] * frac;
                    resampled.push(sample);
                } else {
                    resampled.push(audio[index]);
                }
            }
        }

        resampled
    }
}
