//! 全局热键管理器
//!
//! 负责监听全局热键事件，实现 Push-to-Talk 机制。
//!
//! ## 功能
//! - 注册全局热键（默认 Ctrl+Shift+V）
//! - 检测按键按下/松开事件
//! - 发送事件到状态机
//!
//! ## Push-to-Talk 工作流程
//! 1. 用户按下热键 → 发送 `HotkeyPressed` 事件 → 开始录音
//! 2. 用户松开热键 → 发送 `HotkeyReleased` 事件 → 停止录音
//!
//! ## 使用示例
//! ```rust,no_run
//! use ta_ting::system::hotkey::{HotkeyManager, HotkeyEvent};
//!
//! // 创建并注册热键
//! let manager = HotkeyManager::new().unwrap();
//! manager.register_default().unwrap();
//!
//! // 开始监听
//! loop {
//!     if let Some(event) = manager.poll_event() {
//!         match event {
//!             HotkeyEvent::Pressed => println!("开始录音"),
//!             HotkeyEvent::Released => println!("停止录音"),
//!         }
//!     }
//! }
//! ```

use anyhow::{Context, Result};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use log::{debug, info, warn};

/// 热键事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    /// 热键被按下（开始录音）
    Pressed,
    /// 热键被松开（停止录音）
    Released,
}

/// 全局热键管理器
///
/// 整合了热键注册和事件轮询功能
pub struct HotkeyManager {
    /// global-hotkey 管理器
    manager: GlobalHotKeyManager,
    /// 注册的热键
    hotkey: Option<HotKey>,
}

impl HotkeyManager {
    /// 创建新的热键管理器
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new()
            .context("Failed to create GlobalHotKeyManager")?;

        info!("HotkeyManager initialized");

        Ok(Self {
            manager,
            hotkey: None,
        })
    }

    /// 注册默认热键 (Ctrl+Shift+V)
    pub fn register_default(&mut self) -> Result<()> {
        self.register(Modifiers::CONTROL | Modifiers::SHIFT, Code::KeyV)
    }

    /// 注册自定义热键
    ///
    /// # 参数
    /// - `modifiers`: 修饰键 (Ctrl, Shift, Alt 等)
    /// - `key`: 主键
    ///
    /// # 示例
    /// ```rust,no_run
    /// use global_hotkey::hotkey::{Code, Modifiers};
    ///
    /// // 注册 Ctrl+Shift+V
    /// manager.register(Modifiers::CONTROL | Modifiers::SHIFT, Code::KeyV)?;
    /// ```
    pub fn register(&mut self, modifiers: Modifiers, key: Code) -> Result<()> {
        // 如果已有热键，先取消注册
        if let Some(old_hotkey) = self.hotkey.take() {
            self.manager
                .unregister(old_hotkey)
                .context("Failed to unregister old hotkey")?;
            debug!("Unregistered old hotkey: {:?}", old_hotkey);
        }

        // 创建新热键
        let hotkey = HotKey::new(Some(modifiers), key);

        // 注册热键
        self.manager
            .register(hotkey)
            .context("Failed to register hotkey")?;

        info!("Registered hotkey: {:?}", hotkey);
        self.hotkey = Some(hotkey);

        Ok(())
    }

    /// 取消注册热键
    pub fn unregister(&mut self) -> Result<()> {
        if let Some(hotkey) = self.hotkey.take() {
            self.manager
                .unregister(hotkey)
                .context("Failed to unregister hotkey")?;
            info!("Unregistered hotkey: {:?}", hotkey);
        }
        Ok(())
    }

    /// 轮询热键事件（非阻塞）
    ///
    /// # 返回
    /// - `Some(HotkeyEvent)`: 如果有热键事件
    /// - `None`: 如果没有事件
    pub fn poll_event(&self) -> Option<HotkeyEvent> {
        let global_receiver = GlobalHotKeyEvent::receiver();

        match global_receiver.try_recv() {
            Ok(event) => {
                // 检查是否是我们注册的热键
                let hotkey_id = self.hotkey.as_ref().map(|h| h.id());
                if Some(event.id) != hotkey_id {
                    return None;
                }

                match event.state {
                    global_hotkey::HotKeyState::Pressed => {
                        debug!("Hotkey PRESSED");
                        Some(HotkeyEvent::Pressed)
                    }
                    global_hotkey::HotKeyState::Released => {
                        debug!("Hotkey RELEASED");
                        Some(HotkeyEvent::Released)
                    }
                }
            }
            Err(_) => None,
        }
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default HotkeyManager")
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        if let Err(e) = self.unregister() {
            warn!("Failed to unregister hotkey on drop: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_manager_creation() {
        let manager = HotkeyManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_register_default_hotkey() {
        let mut manager = HotkeyManager::new().unwrap();
        let result = manager.register_default();
        assert!(result.is_ok());
    }

    #[test]
    fn test_unregister_hotkey() {
        let mut manager = HotkeyManager::new().unwrap();
        manager.register_default().unwrap();
        let result = manager.unregister();
        assert!(result.is_ok());
    }
}
