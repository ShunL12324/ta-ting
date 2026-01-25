//! 音频录制器
//!
//! 负责从麦克风采集音频数据，用于语音识别。
//!
//! ## 功能
//! - 从默认麦克风采集音频（自动适应设备采样率和声道数）
//! - 实时缓冲录音数据
//! - 线程安全的开始/停止控制
//! - 自动转换多声道为单声道
//!
//! ## 使用示例
//! ```rust,no_run
//! use ta_ting::audio::recorder::AudioRecorder;
//!
//! // 创建录音器
//! let mut recorder = AudioRecorder::new().unwrap();
//!
//! // 开始录音
//! recorder.start_recording().unwrap();
//!
//! // 等待用户说话...
//! std::thread::sleep(std::time::Duration::from_secs(3));
//!
//! // 停止录音并获取数据
//! let audio_data = recorder.stop_recording().unwrap();
//! println!("录制了 {} 个采样点", audio_data.len());
//! ```

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig, SampleFormat};
use log::{debug, error, info, warn};
use std::sync::{Arc, Mutex};

/// Whisper 使用的标准采样率 (16kHz)
pub const WHISPER_SAMPLE_RATE: u32 = 16000;

/// 音频数据回调类型
pub type AudioCallback = Arc<Mutex<dyn FnMut(&[f32]) + Send>>;

/// 音频录制器状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecorderState {
    /// 空闲状态
    Idle,
    /// 正在录音
    Recording,
}

/// 音频录制器
///
/// 使用 cpal 从默认麦克风采集音频
pub struct AudioRecorder {
    /// 录音设备
    device: Device,
    /// 流配置
    config: StreamConfig,
    /// 音频流（录音时存在）
    stream: Option<Stream>,
    /// 录音缓冲区（线程安全）
    buffer: Arc<Mutex<Vec<f32>>>,
    /// 录音器状态
    state: RecorderState,
    /// 实时音频数据回调（可选）
    audio_callback: Option<AudioCallback>,
}

// 标记 AudioRecorder 为 Send，因为：
// 1. Device 和 Stream 虽然不是 Send，但我们只在同一个线程中使用它们
// 2. buffer 已经使用 Arc<Mutex> 保护，是线程安全的
// 3. 我们确保不会在多线程中同时访问 Device 和 Stream
unsafe impl Send for AudioRecorder {}

impl AudioRecorder {
    /// 创建新的音频录音器
    ///
    /// # 返回
    /// - `Ok(AudioRecorder)`: 成功创建
    /// - `Err`: 无法访问音频设备或不支持的配置
    pub fn new() -> Result<Self> {
        // 获取默认主机
        let host = cpal::default_host();
        info!("使用音频主机: {}", host.id().name());

        // 获取默认输入设备（麦克风）
        let device = host
            .default_input_device()
            .context("未找到默认输入设备（麦克风）")?;

        info!("使用输入设备: {}", device.name()?);

        // 获取设备支持的配置
        let supported_config = device
            .default_input_config()
            .context("无法获取设备默认配置")?;

        debug!("设备默认配置: {:?}", supported_config);

        // 使用设备的默认配置（稍后会转换为单声道）
        let config = supported_config.config();

        info!("录音配置: {:?}", config);
        info!(
            "设备: {} 声道, {} Hz (将转换为单声道)",
            config.channels, config.sample_rate.0
        );

        Ok(Self {
            device,
            config,
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            state: RecorderState::Idle,
            audio_callback: None,
        })
    }

    /// 设置实时音频回调
    pub fn set_audio_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&[f32]) + Send + 'static,
    {
        info!("设置音频回调");
        self.audio_callback = Some(Arc::new(Mutex::new(callback)));
    }

    /// 开始录音
    ///
    /// # 返回
    /// - `Ok(())`: 成功开始录音
    /// - `Err`: 无法创建音频流或已在录音中
    pub fn start_recording(&mut self) -> Result<()> {
        if self.state == RecorderState::Recording {
            warn!("录音器已在录音中");
            return Ok(());
        }

        // 清空缓冲区
        self.buffer.lock().unwrap().clear();

        // 创建音频流
        let buffer = Arc::clone(&self.buffer);
        let err_callback = |err| {
            error!("音频流错误: {}", err);
        };

        // 根据设备支持的采样格式创建流
        let supported_config = self.device.default_input_config()?;
        info!("设备采样格式: {:?}", supported_config.sample_format());

        let stream = match supported_config.sample_format() {
            SampleFormat::F32 => {
                self.build_input_stream_f32(buffer, err_callback)?
            }
            SampleFormat::I16 => {
                self.build_input_stream_i16(buffer, err_callback)?
            }
            SampleFormat::U16 => {
                self.build_input_stream_u16(buffer, err_callback)?
            }
            sample_format => {
                return Err(anyhow::anyhow!(
                    "不支持的采样格式: {:?}",
                    sample_format
                ));
            }
        };

        // 启动流
        stream.play().context("无法启动音频流")?;

        self.stream = Some(stream);
        self.state = RecorderState::Recording;

        info!("开始录音");
        Ok(())
    }

    /// 构建 f32 格式的输入流
    fn build_input_stream_f32(
        &self,
        buffer: Arc<Mutex<Vec<f32>>>,
        err_callback: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<Stream> {
        let config = self.config.clone();
        let channels = config.channels as usize;
        let audio_callback = self.audio_callback.clone();

        info!("创建 F32 音频流，回调存在: {}", audio_callback.is_some());

        let stream = self.device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                log::info!("🎵 音频流回调触发: {} 采样点", data.len());

                let mut buffer = buffer.lock().unwrap();
                // 如果是多声道，转换为单声道（取平均值）
                if channels == 1 {
                    buffer.extend_from_slice(data);
                    // 调用回调函数
                    if let Some(ref callback) = audio_callback {
                        log::info!("准备调用回调 (单声道)");
                        if let Ok(mut cb) = callback.lock() {
                            log::info!("回调锁获取成功，调用中...");
                            cb(data);
                        }
                    } else {
                        log::warn!("回调为 None");
                    }
                } else {
                    let mut mono_chunk = Vec::with_capacity(data.len() / channels);
                    for chunk in data.chunks(channels) {
                        let mono_sample: f32 = chunk.iter().sum::<f32>() / channels as f32;
                        buffer.push(mono_sample);
                        mono_chunk.push(mono_sample);
                    }
                    // 调用回调函数
                    if let Some(ref callback) = audio_callback {
                        log::info!("准备调用回调 (多声道, {} 采样点)", mono_chunk.len());
                        if let Ok(mut cb) = callback.lock() {
                            log::info!("回调锁获取成功，调用中...");
                            cb(&mono_chunk);
                        }
                    } else {
                        log::warn!("回调为 None");
                    }
                }
            },
            err_callback,
            None,
        )?;

        Ok(stream)
    }

    /// 构建 i16 格式的输入流
    fn build_input_stream_i16(
        &self,
        buffer: Arc<Mutex<Vec<f32>>>,
        err_callback: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<Stream> {
        let config = self.config.clone();
        let channels = config.channels as usize;
        let audio_callback = self.audio_callback.clone();

        info!("创建 I16 音频流，回调存在: {}", audio_callback.is_some());

        let stream = self.device.build_input_stream(
            &config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                let mut buffer = buffer.lock().unwrap();
                if channels == 1 {
                    let mut f32_data = Vec::with_capacity(data.len());
                    for &sample in data {
                        let value = sample as f32 / 32768.0;
                        buffer.push(value);
                        f32_data.push(value);
                    }
                    // 调用回调函数
                    if let Some(ref callback) = audio_callback {
                        if let Ok(mut cb) = callback.lock() {
                            cb(&f32_data);
                        }
                    }
                } else {
                    let mut mono_chunk = Vec::with_capacity(data.len() / channels);
                    for chunk in data.chunks(channels) {
                        let mono_sample: f32 = chunk.iter()
                            .map(|&s| s as f32 / 32768.0)
                            .sum::<f32>() / channels as f32;
                        buffer.push(mono_sample);
                        mono_chunk.push(mono_sample);
                    }
                    // 调用回调函数
                    if let Some(ref callback) = audio_callback {
                        if let Ok(mut cb) = callback.lock() {
                            cb(&mono_chunk);
                        }
                    }
                }
            },
            err_callback,
            None,
        )?;

        Ok(stream)
    }

    /// 构建 u16 格式的输入流
    fn build_input_stream_u16(
        &self,
        buffer: Arc<Mutex<Vec<f32>>>,
        err_callback: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<Stream> {
        let config = self.config.clone();
        let channels = config.channels as usize;
        let audio_callback = self.audio_callback.clone();

        let stream = self.device.build_input_stream(
            &config,
            move |data: &[u16], _: &cpal::InputCallbackInfo| {
                let mut buffer = buffer.lock().unwrap();
                if channels == 1 {
                    let mut f32_data = Vec::with_capacity(data.len());
                    for &sample in data {
                        let value = (sample as f32 / 32768.0) - 1.0;
                        buffer.push(value);
                        f32_data.push(value);
                    }
                    // 调用回调函数
                    if let Some(ref callback) = audio_callback {
                        if let Ok(mut cb) = callback.lock() {
                            cb(&f32_data);
                        }
                    }
                } else {
                    let mut mono_chunk = Vec::with_capacity(data.len() / channels);
                    for chunk in data.chunks(channels) {
                        let mono_sample: f32 = chunk.iter()
                            .map(|&s| (s as f32 / 32768.0) - 1.0)
                            .sum::<f32>() / channels as f32;
                        buffer.push(mono_sample);
                        mono_chunk.push(mono_sample);
                    }
                    // 调用回调函数
                    if let Some(ref callback) = audio_callback {
                        if let Ok(mut cb) = callback.lock() {
                            cb(&mono_chunk);
                        }
                    }
                }
            },
            err_callback,
            None,
        )?;

        Ok(stream)
    }

    /// 停止录音并返回音频数据
    ///
    /// # 返回
    /// - `Ok(Vec<f32>)`: 录制的音频数据（单声道，f32 格式）
    /// - `Err`: 停止失败或未在录音中
    pub fn stop_recording(&mut self) -> Result<Vec<f32>> {
        if self.state != RecorderState::Recording {
            warn!("录音器未在录音中");
            return Ok(Vec::new());
        }

        // 停止并销毁流
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }

        self.state = RecorderState::Idle;

        // 获取录音数据
        let data = {
            let mut buffer = self.buffer.lock().unwrap();
            let data = buffer.clone();
            buffer.clear();
            data
        };

        let sample_rate = self.config.sample_rate.0;
        info!(
            "停止录音，采集了 {} 个采样点 ({:.2} 秒, {} Hz)",
            data.len(),
            data.len() as f32 / sample_rate as f32,
            sample_rate
        );

        Ok(data)
    }

    /// 获取当前状态
    pub fn state(&self) -> RecorderState {
        self.state
    }

    /// 检查是否正在录音
    pub fn is_recording(&self) -> bool {
        self.state == RecorderState::Recording
    }

    /// 获取当前缓冲区大小（采样点数）
    pub fn buffer_size(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }

    /// 获取当前录音时长（秒）
    pub fn duration(&self) -> f32 {
        let sample_rate = self.config.sample_rate.0;
        self.buffer_size() as f32 / sample_rate as f32
    }

    /// 获取采样率
    pub fn sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }

    /// 获取声道数
    pub fn channels(&self) -> u16 {
        self.config.channels
    }
}

impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new().expect("Failed to create default AudioRecorder")
    }
}

impl Drop for AudioRecorder {
    fn drop(&mut self) {
        if self.is_recording() {
            if let Err(e) = self.stop_recording() {
                warn!("停止录音失败: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_creation() {
        let recorder = AudioRecorder::new();
        assert!(recorder.is_ok());
    }

    #[test]
    fn test_recorder_state() {
        let recorder = AudioRecorder::new().unwrap();
        assert_eq!(recorder.state(), RecorderState::Idle);
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_start_stop_recording() {
        let mut recorder = AudioRecorder::new().unwrap();

        // 开始录音
        let result = recorder.start_recording();
        assert!(result.is_ok());
        assert_eq!(recorder.state(), RecorderState::Recording);
        assert!(recorder.is_recording());

        // 等待一小段时间
        std::thread::sleep(std::time::Duration::from_millis(100));

        // 停止录音
        let data = recorder.stop_recording();
        assert!(data.is_ok());
        assert_eq!(recorder.state(), RecorderState::Idle);

        let samples = data.unwrap();
        assert!(!samples.is_empty()); // 应该录到了一些数据
    }
}
