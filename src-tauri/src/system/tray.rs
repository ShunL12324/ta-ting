//! 系统托盘管理
//!
//! 提供系统托盘图标和菜单

use anyhow::Result;
use log::info;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime,
};

/// 创建系统托盘
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    info!("创建系统托盘...");

    // 创建托盘菜单
    let start_item = MenuItemBuilder::with_id("start_dictation", "开始听写 (Ctrl+Shift+D)")
        .build(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&start_item)
        .item(&settings_item)
        .separator()
        .item(&quit_item)
        .build()?;

    // 创建托盘图标
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("TaTing - AI 离线听写输入法")
        .on_menu_event(move |app_handle: &AppHandle<R>, event| {
            handle_menu_event(app_handle, event.id.as_ref());
        })
        .on_tray_icon_event(|tray_icon: &tauri::tray::TrayIcon<R>, event| {
            if let TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } = event
            {
                info!("托盘图标被左键点击");
                let app = tray_icon.app_handle();
                // 显示主窗口
                if let Some(window) = app.get_webview_window("main") {
                    let _: Result<(), tauri::Error> = window.show();
                    let _: Result<(), tauri::Error> = window.set_focus();
                }
            }
        })
        .build(app)?;

    info!("✅ 系统托盘创建成功");
    Ok(())
}

/// 处理托盘菜单事件
fn handle_menu_event<R: tauri::Runtime>(app: &AppHandle<R>, event_id: &str) {
    info!("托盘菜单事件: {}", event_id);

    match event_id {
        "start_dictation" => {
            info!("触发开始听写");
            // 发射热键事件，由前端或后端监听
            if let Err(e) = app.emit("hotkey_pressed", ()) {
                log::error!("发送热键事件失败: {}", e);
            }
        }
        "settings" => {
            info!("打开设置");
            // 显示设置窗口
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "quit" => {
            info!("退出应用");
            app.exit(0);
        }
        _ => {
            log::warn!("未知的菜单事件: {}", event_id);
        }
    }
}
