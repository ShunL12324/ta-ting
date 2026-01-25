// 模块声明
pub mod audio;
pub mod config;
pub mod core;
pub mod system;
pub mod asr;
pub mod punctuation;

use core::app::{AppConfig, TaTingApp};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};

// ==================== 全局状态 ====================

/// 全局应用状态包装器
pub struct AppState {
    pub app: Arc<Mutex<TaTingApp>>,
}

// ==================== Tauri Commands ====================

/// 获取当前状态（字符串格式，方便 TypeScript 使用）
#[tauri::command]
fn get_current_state(state: State<AppState>) -> String {
    let app = state.app.lock().unwrap();
    format!("{}", app.current_state())
}

/// 手动触发热键（用于托盘菜单或测试）
#[tauri::command]
fn trigger_hotkey(state: State<AppState>, app_handle: AppHandle) -> Result<(), String> {
    info!("收到手动触发热键命令");
    let app = state.app.lock().unwrap();
    app.handle_hotkey_with_events(&app_handle)
        .map_err(|e| e.to_string())
}

// ==================== 录音窗口管理 ====================

use tauri::{WebviewUrl, WebviewWindowBuilder, WebviewWindow};

/// 创建录音窗口
pub fn create_recording_window<R: tauri::Runtime>(app_handle: &AppHandle<R>) -> Result<WebviewWindow<R>, String> {
    info!("创建录音窗口");

    // 先计算位置 - 稍大的窗口（留边距给圆角抗锯齿）
    let (x, y) = if let Ok(Some(monitor)) = app_handle.primary_monitor() {
        let screen = monitor.size();
        let window_width = 400;  // 比实际内容大20px
        let window_height = 90;  // 比实际内容大20px

        let x = (screen.width as i32 - window_width) / 2;
        // 放在屏幕垂直方向的 70% 位置（中下方）
        let y = (screen.height as f32 * 0.7) as i32;
        (x as f64, y as f64)
    } else {
        (0.0, 0.0)
    };

    let window = WebviewWindowBuilder::new(
        app_handle,
        "recording",
        WebviewUrl::App("recording.html".into()),
    )
    .title("录音中")
    .inner_size(400.0, 90.0)  // 留边距
    .position(x, y)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .transparent(true)
    .visible(true)
    .build()
    .map_err(|e| e.to_string())?;

    // 不使用 Windows 圆角处理，让浏览器 CSS 处理
    // 这样可以获得浏览器原生的抗锯齿效果

    Ok(window)
}

/// 关闭录音窗口
pub fn close_recording_window<R: tauri::Runtime>(app_handle: &AppHandle<R>) {
    if let Some(window) = app_handle.get_webview_window("recording") {
        info!("关闭录音窗口");
        let _ = window.close();
    }
}

// ==================== 全局热键管理 ====================

/// 注册全局热键并监听事件
fn setup_global_hotkey(app_handle: AppHandle, app_state: Arc<Mutex<TaTingApp>>) -> anyhow::Result<()> {
    info!("注册全局热键 Ctrl+Shift+D...");

    // 创建热键管理器
    let hotkey_manager = GlobalHotKeyManager::new()
        .map_err(|e| anyhow::anyhow!("创建热键管理器失败: {}", e))?;

    // 定义热键：Ctrl+Shift+D (D for Dictation)
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyD);

    // 注册热键
    hotkey_manager
        .register(hotkey)
        .map_err(|e| anyhow::anyhow!("注册热键失败: {}", e))?;

    info!("✅ 全局热键 Ctrl+Shift+D 注册成功");

    // 🔥 关键修复：防止 hotkey_manager 被 drop，否则热键会失效！
    std::mem::forget(hotkey_manager);
    info!("✅ 热键管理器已固定在内存中");

    // 在独立线程中监听热键事件
    std::thread::spawn(move || {
        info!("热键监听线程已启动");
        let receiver = GlobalHotKeyEvent::receiver();
        info!("热键事件接收器已创建");

        loop {
            match receiver.try_recv() {
                Ok(event) => {
                    info!("🔥 检测到全局热键事件: {:?}", event);

                    // 触发应用的热键处理逻辑
                    let app_lock = app_state.lock().unwrap();
                    if let Err(e) = app_lock.handle_hotkey_with_events(&app_handle) {
                        error!("处理热键事件失败: {}", e);
                    }
                }
                Err(e) if e.is_empty() => {
                    // 没有事件，继续等待
                }
                Err(e) => {
                    error!("热键接收器错误: {:?}", e);
                    break;
                }
            }

            // 避免 CPU 占用过高
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        error!("热键监听线程已退出！");
    });

    Ok(())
}

// ==================== Tauri 入口 ====================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 配置日志插件（在 debug 和 release 都启用）
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;

            info!("TaTing 启动中...");

            // 1. 创建 TaTing 应用实例
            info!("初始化 TaTing 应用实例...");
            let config = AppConfig::default();
            let tating_app = TaTingApp::new(config)
                .map_err(|e| format!("创建应用失败: {}", e))?;

            // 2. 初始化所有组件（Sherpa、录音器、输入模拟器）
            tating_app
                .initialize()
                .map_err(|e| format!("初始化组件失败: {}", e))?;

            // 3. 包装成 Arc<Mutex> 用于跨线程共享
            let app_state = Arc::new(Mutex::new(tating_app));

            // 4. 注册全局热键
            setup_global_hotkey(app.handle().clone(), Arc::clone(&app_state))
                .map_err(|e| format!("注册全局热键失败: {}", e))?;

            // 5. 创建系统托盘
            system::tray::create_tray(app.handle())?;

            // 6. 将应用状态注册到 Tauri State 管理
            app.manage(AppState { app: app_state });

            info!("🎉 应用初始化完成");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_current_state,
            trigger_hotkey,
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}
