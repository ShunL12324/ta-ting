// 模块声明
pub mod audio;
pub mod config;
pub mod core;
pub mod system;
pub mod asr;
pub mod punctuation;

use config::settings::AppSettings;
use core::app::{AppConfig, TaTingApp};
use global_hotkey::GlobalHotKeyEvent;
use log::{error, info};
use std::sync::{Arc, Mutex, OnceLock};
use tauri::{AppHandle, Manager, State};

// ==================== 全局状态 ====================

pub struct AppState {
    pub app: Arc<Mutex<TaTingApp>>,
}

// ==================== Settings helpers ====================

fn settings_path(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_local_data_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("settings.json")
}

// ==================== Tauri Commands ====================

#[tauri::command]
fn get_current_state(state: State<AppState>) -> String {
    let app = state.app.lock().unwrap();
    format!("{}", app.current_state())
}

#[tauri::command]
fn trigger_hotkey(state: State<AppState>, app_handle: AppHandle) -> Result<(), String> {
    info!("收到手动触发热键命令");
    let app = state.app.lock().unwrap();
    app.handle_hotkey(Some(app_handle))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    AppSettings::load_from_file(&settings_path(&app)).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_hotkey(hotkey: String, app: AppHandle) -> Result<(), String> {
    // 1. Parse + register
    let manager = HOTKEY_MANAGER
        .get()
        .ok_or("HotkeyManager not initialized")?;
    manager
        .lock()
        .unwrap()
        .register_str(&hotkey)
        .map_err(|e| e.to_string())?;

    info!("热键已更新: {}", hotkey);

    // 2. Persist
    let path = settings_path(&app);
    let mut settings = AppSettings::load_from_file(&path).unwrap_or_default();
    settings.hotkey = hotkey;
    settings.save_to_file(&path).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn check_for_updates(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_updater::UpdaterExt;

    info!("开始检查更新...");

    let updater = app.updater_builder().build().map_err(|e| {
        error!("创建更新器失败: {}", e);
        format!("创建更新器失败: {}", e)
    })?;

    match updater.check().await {
        Ok(Some(update)) => {
            info!("发现新版本: {}", update.version);
            Ok(format!("发现新版本: {}，点击更新按钮下载", update.version))
        }
        Ok(None) => {
            info!("已是最新版本");
            Ok("已是最新版本".to_string())
        }
        Err(e) => {
            error!("检查更新失败: {}", e);
            Err(format!("检查更新失败: {}", e))
        }
    }
}

// ==================== 录音窗口管理 ====================

use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub fn create_recording_window<R: tauri::Runtime>(
    app_handle: &AppHandle<R>,
) -> Result<WebviewWindow<R>, String> {
    info!("创建录音窗口");

    let (x, y) = if let Ok(Some(monitor)) = app_handle.primary_monitor() {
        let screen = monitor.size();
        let window_width = 400;
        let window_height = 90;
        let x = (screen.width as i32 - window_width) / 2;
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
    .inner_size(400.0, 90.0)
    .position(x, y)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .transparent(true)
    .visible(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(window)
}

pub fn close_recording_window<R: tauri::Runtime>(app_handle: &AppHandle<R>) {
    if let Some(window) = app_handle.get_webview_window("recording") {
        info!("关闭录音窗口");
        let _ = window.close();
    }
}

// ==================== 全局热键管理 ====================

static HOTKEY_MANAGER: OnceLock<std::sync::Mutex<system::HotkeyManager>> = OnceLock::new();

fn setup_global_hotkey(
    hotkey_str: &str,
    app_handle: AppHandle,
    app_state: Arc<Mutex<TaTingApp>>,
) -> anyhow::Result<()> {
    info!("注册全局热键 {}...", hotkey_str);

    let mut manager = system::HotkeyManager::new()?;
    manager.register_str(hotkey_str)?;
    HOTKEY_MANAGER.get_or_init(|| std::sync::Mutex::new(manager));
    info!("✅ 全局热键 {} 注册成功", hotkey_str);

    std::thread::spawn(move || {
        info!("热键监听线程已启动");
        let receiver = GlobalHotKeyEvent::receiver();

        loop {
            match receiver.try_recv() {
                Ok(event) => {
                    info!("🔥 检测到全局热键事件: {:?}", event);
                    let app_lock = app_state.lock().unwrap();
                    if let Err(e) = app_lock.handle_hotkey(Some(app_handle.clone())) {
                        error!("处理热键事件失败: {}", e);
                    }
                }
                Err(e) if e.is_empty() => {}
                Err(e) => {
                    error!("热键接收器错误: {:?}", e);
                    break;
                }
            }
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
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;

            info!("TaTing 启动中...");

            // 1. Load persisted settings
            let settings_file = settings_path(app.handle());
            let settings = AppSettings::load_from_file(&settings_file).unwrap_or_default();
            info!("已加载设置: 热键={}", settings.hotkey);

            // 2. Init TaTing app
            let model_path = app
                .path()
                .resolve(
                    "resources/models/sherpa-zh/sherpa-onnx-zipformer-multi-zh-hans-2023-9-2",
                    tauri::path::BaseDirectory::Resource,
                )
                .map_err(|e| format!("无法解析模型路径: {}", e))?
                .to_string_lossy()
                .into_owned();

            let punct_model_path = app
                .path()
                .resolve(
                    "resources/models/sherpa-punct/sherpa-onnx-punct-ct-transformer-zh-en-vocab272727-2024-04-12/model.onnx",
                    tauri::path::BaseDirectory::Resource,
                )
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default();

            let config = AppConfig {
                model_path,
                punct_model_path,
                ..AppConfig::default()
            };
            let tating_app =
                TaTingApp::new(config).map_err(|e| format!("创建应用失败: {}", e))?;

            tating_app
                .initialize()
                .map_err(|e| format!("初始化组件失败: {}", e))?;

            let app_state = Arc::new(Mutex::new(tating_app));

            // 3. Register hotkey from settings
            setup_global_hotkey(&settings.hotkey, app.handle().clone(), Arc::clone(&app_state))
                .map_err(|e| format!("注册全局热键失败: {}", e))?;

            // 4. Create tray with hotkey display label
            let display = system::hotkey::hotkey_to_display(&settings.hotkey);
            system::tray::create_tray(app.handle(), &display)?;

            // 5. Auto-update plugin
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;

            // 6. Register app state
            app.manage(AppState { app: app_state });

            info!("🎉 应用初始化完成");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_current_state,
            trigger_hotkey,
            get_settings,
            set_hotkey,
            check_for_updates,
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}
