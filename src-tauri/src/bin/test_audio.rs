//! 音频录制功能测试
//!
//! 测试麦克风录音和音频数据采集
//!
//! ## 用法
//! ```bash
//! cargo run --bin test_audio
//! ```

use ta_ting_lib::audio::recorder::{AudioRecorder, WHISPER_SAMPLE_RATE};
use std::time::Duration;
use std::thread;

fn main() {
    // 初始化日志
    env_logger::init();

    println!("=== TaTing 音频录制测试 ===\n");

    // 创建录音器
    println!("1. 创建音频录音器...");
    let mut recorder = match AudioRecorder::new() {
        Ok(r) => {
            println!("   ✅ 录音器创建成功");
            r
        }
        Err(e) => {
            eprintln!("   ❌ 创建录音器失败: {}", e);
            eprintln!("\n可能的原因:");
            eprintln!("- 没有可用的麦克风");
            eprintln!("- 麦克风权限未授予");
            eprintln!("- 音频设备被其他程序占用");
            return;
        }
    };

    println!("   状态: {:?}", recorder.state());
    println!();

    // 开始录音
    println!("2. 开始录音...");
    if let Err(e) = recorder.start_recording() {
        eprintln!("   ❌ 开始录音失败: {}", e);
        return;
    }
    println!("   ✅ 录音已开始 (16kHz 单声道)");
    println!();

    // 录制 3 秒
    println!("🎤 请对着麦克风说话，将录制 3 秒钟...\n");

    for i in 1..=3 {
        thread::sleep(Duration::from_secs(1));
        let duration = recorder.duration();
        let buffer_size = recorder.buffer_size();
        println!(
            "   [{}/3] 已录制: {:.2}秒 ({} 采样点)",
            i, duration, buffer_size
        );
    }

    println!();

    // 停止录音
    println!("3. 停止录音...");
    let audio_data = match recorder.stop_recording() {
        Ok(data) => {
            println!("   ✅ 录音已停止");
            data
        }
        Err(e) => {
            eprintln!("   ❌ 停止录音失败: {}", e);
            return;
        }
    };

    println!();

    // 分析录音数据
    println!("4. 音频数据分析:");
    println!("   - 采样点数: {}", audio_data.len());
    println!("   - 时长: {:.2} 秒", audio_data.len() as f32 / WHISPER_SAMPLE_RATE as f32);
    println!("   - 采样率: {} Hz", WHISPER_SAMPLE_RATE);
    println!("   - 声道: 单声道");
    println!("   - 格式: f32 (32位浮点)");

    // 计算音频统计
    if !audio_data.is_empty() {
        let max_amplitude = audio_data.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        let avg_amplitude = audio_data.iter().map(|&x| x.abs()).sum::<f32>() / audio_data.len() as f32;
        let rms = (audio_data.iter().map(|&x| x * x).sum::<f32>() / audio_data.len() as f32).sqrt();

        println!();
        println!("   音频质量:");
        println!("   - 最大振幅: {:.4}", max_amplitude);
        println!("   - 平均振幅: {:.4}", avg_amplitude);
        println!("   - RMS: {:.4}", rms);

        if max_amplitude < 0.01 {
            println!("\n   ⚠️  警告: 音频信号很弱，请检查麦克风音量或距离");
        } else if max_amplitude > 0.9 {
            println!("\n   ⚠️  警告: 音频信号过强，可能发生削波");
        } else {
            println!("\n   ✅ 音频信号质量良好");
        }
    }

    println!();
    println!("=== 测试完成 ===");
}
