//! TaTing 完整流程测试

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use ta_ting_lib::core::app::{AppConfig, TaTingApp};

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, PeekMessageW, TranslateMessage, PM_REMOVE, MSG,
};

fn main() {
    env_logger::init();

    println!("=== TaTing 完整流程测试 ===\n");

    println!("1. 初始化应用...");
    let config = AppConfig::default();
    let app = match TaTingApp::new(config) {
        Ok(app) => {
            println!("   ✅ 应用创建成功");
            app
        }
        Err(e) => {
            eprintln!("   ❌ 创建应用失败: {}", e);
            return;
        }
    };

    println!("\n2. 加载组件（Whisper 模型等）...");
    if let Err(e) = app.initialize() {
        eprintln!("   ❌ 初始化失败: {}", e);
        return;
    }
    println!("   ✅ 所有组件已就绪");

    println!("\n3. 注册全局热键 Ctrl+Shift+V...");
    let manager = match GlobalHotKeyManager::new() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("   ❌ 创建热键管理器失败: {}", e);
            return;
        }
    };

    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
    if let Err(e) = manager.register(hotkey) {
        eprintln!("   ❌ 注册热键失败: {}", e);
        return;
    }
    println!("   ✅ 热键已注册");

    println!("\n=== 准备就绪 ===\n");
    println!("📌 使用方法:");
    println!("1. 打开一个文本编辑器（记事本）");
    println!("2. 按 Ctrl+Shift+V 开始录音");
    println!("3. 说一句话");
    println!("4. 再按 Ctrl+Shift+V 停止录音");
    println!("5. 等待转录（可能需要几秒钟）");
    println!("6. 文本会自动输入到编辑器\n");
    println!("按 Ctrl+C 退出程序\n");

    let receiver = GlobalHotKeyEvent::receiver();

    // 主事件循环
    #[cfg(target_os = "windows")]
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        loop {
            // 处理 Windows 消息
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            // 检查热键事件（非阻塞）
            if let Ok(_event) = receiver.try_recv() {
                println!("\n🎯 热键触发！");
                if let Err(e) = app.handle_hotkey(None) {
                    eprintln!("❌ 处理热键失败: {}", e);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    #[cfg(not(target_os = "windows"))]
    loop {
        if let Ok(_event) = receiver.recv() {
            println!("\n🎯 热键触发！");
            if let Err(e) = app.handle_hotkey(None) {
                eprintln!("❌ 处理热键失败: {}", e);
            }
        }
    }
}
