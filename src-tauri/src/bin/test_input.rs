//! 键盘输入模拟测试
//!
//! 测试 enigo 键盘输入功能
//!
//! ## 用法
//! ```bash
//! # 1. 运行测试程序
//! cargo run --bin test_input
//!
//! # 2. 在 5 秒倒计时内，点击一个文本编辑器（如记事本）
//! # 3. 观察文本是否自动输入
//! ```

use ta_ting_lib::system::input::InputSimulator;
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();

    println!("=== TaTing 键盘输入测试 ===\n");

    println!("准备测试键盘输入模拟功能");
    println!("\n⚠️  重要提示:");
    println!("1. 程序将在 5 秒后开始输入文本");
    println!("2. 请在倒计时期间点击一个文本编辑器（如记事本、Word）");
    println!("3. 确保光标位于输入位置\n");

    // 倒计时
    for i in (1..=5).rev() {
        println!("   {} 秒后开始输入...", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n🎯 开始输入测试文本...\n");

    // 创建输入模拟器
    let mut simulator = match InputSimulator::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ 创建输入模拟器失败: {}", e);
            return;
        }
    };

    // 测试 1: 中文输入
    println!("测试 1: 中文输入");
    if let Err(e) = simulator.type_text("你好，世界！") {
        eprintln!("   ❌ 中文输入失败: {}", e);
    } else {
        println!("   ✅ 中文输入完成");
    }

    thread::sleep(Duration::from_millis(500));

    // 测试 2: 回车换行
    println!("\n测试 2: 换行");
    if let Err(e) = simulator.press_enter() {
        eprintln!("   ❌ 回车失败: {}", e);
    } else {
        println!("   ✅ 回车完成");
    }

    thread::sleep(Duration::from_millis(500));

    // 测试 3: 英文输入
    println!("\n测试 3: 英文输入");
    if let Err(e) = simulator.type_text("Hello, TaTing!") {
        eprintln!("   ❌ 英文输入失败: {}", e);
    } else {
        println!("   ✅ 英文输入完成");
    }

    thread::sleep(Duration::from_millis(500));

    // 测试 4: 混合文本
    if let Err(e) = simulator.press_enter() {
        eprintln!("   ❌ 回车失败: {}", e);
    }
    println!("\n测试 4: 中英文混合");
    if let Err(e) = simulator.type_text("这是一个测试 Test 123 !@#") {
        eprintln!("   ❌ 混合输入失败: {}", e);
    } else {
        println!("   ✅ 混合输入完成");
    }

    println!("\n=== 测试完成 ===");
    println!("\n请检查文本编辑器中的输入结果:");
    println!("----------------------------------------");
    println!("你好，世界！");
    println!("Hello, TaTing!");
    println!("这是一个测试 Test 123 !@#");
    println!("----------------------------------------");
}
