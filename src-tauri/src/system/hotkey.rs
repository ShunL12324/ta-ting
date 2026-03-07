//! Global hotkey manager
//!
//! Wraps `global-hotkey` to support registering/re-registering a single
//! configurable hotkey at runtime — needed for the Phase 2 custom hotkey feature.

use anyhow::{Context, Result};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use log::{debug, info, warn};

/// Hotkey event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    Pressed,
    Released,
}

/// Manages a single registered global hotkey.
///
/// Call [`register_default`] or [`register`] to set the active hotkey.
/// Re-calling [`register`] atomically swaps the old hotkey for the new one.
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    hotkey: Option<HotKey>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new()
            .context("Failed to create GlobalHotKeyManager")?;
        info!("HotkeyManager initialized");
        Ok(Self { manager, hotkey: None })
    }

    /// Register the default hotkey: Ctrl+Shift+V
    pub fn register_default(&mut self) -> Result<()> {
        self.register(Modifiers::CONTROL | Modifiers::SHIFT, Code::KeyV)
    }

    /// Register a custom hotkey, replacing the previously registered one.
    pub fn register(&mut self, modifiers: Modifiers, key: Code) -> Result<()> {
        if let Some(old) = self.hotkey.take() {
            self.manager
                .unregister(old)
                .context("Failed to unregister old hotkey")?;
            debug!("Unregistered old hotkey: {:?}", old);
        }

        let hotkey = HotKey::new(Some(modifiers), key);
        self.manager
            .register(hotkey)
            .context("Failed to register hotkey")?;

        info!("Registered hotkey: {:?}", hotkey);
        self.hotkey = Some(hotkey);
        Ok(())
    }

    /// Unregister the current hotkey.
    pub fn unregister(&mut self) -> Result<()> {
        if let Some(hotkey) = self.hotkey.take() {
            self.manager
                .unregister(hotkey)
                .context("Failed to unregister hotkey")?;
            info!("Unregistered hotkey: {:?}", hotkey);
        }
        Ok(())
    }

    /// Returns the ID of the currently registered hotkey, if any.
    pub fn hotkey_id(&self) -> Option<u32> {
        self.hotkey.as_ref().map(|h| h.id())
    }

    /// Non-blocking poll for a hotkey event.
    pub fn poll_event(&self) -> Option<HotkeyEvent> {
        match GlobalHotKeyEvent::receiver().try_recv() {
            Ok(event) => {
                if Some(event.id) != self.hotkey_id() {
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
        Self::new().expect("Failed to create HotkeyManager")
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
        assert!(HotkeyManager::new().is_ok());
    }

    #[test]
    fn test_register_default_hotkey() {
        let mut manager = HotkeyManager::new().unwrap();
        assert!(manager.register_default().is_ok());
    }

    #[test]
    fn test_unregister_hotkey() {
        let mut manager = HotkeyManager::new().unwrap();
        manager.register_default().unwrap();
        assert!(manager.unregister().is_ok());
    }
}
