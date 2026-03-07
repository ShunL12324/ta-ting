//! Global hotkey manager
//!
//! Supports runtime hotkey re-registration for the Phase 2 custom hotkey feature.
//! Hotkeys are stored in internal format: "Ctrl+Shift+KeyV"
//! (modifiers joined with key code from KeyboardEvent.code).

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
/// Call [`register_default`] or [`register_str`] to set the active hotkey.
/// Re-calling replaces the previous registration atomically.
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    hotkey: Option<HotKey>,
    /// Internal format: "Ctrl+Shift+KeyV"
    hotkey_str: Option<String>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new()
            .context("Failed to create GlobalHotKeyManager")?;
        info!("HotkeyManager initialized");
        Ok(Self {
            manager,
            hotkey: None,
            hotkey_str: None,
        })
    }

    /// Register the default hotkey: Ctrl+Shift+V
    pub fn register_default(&mut self) -> Result<()> {
        self.register_str("Ctrl+Shift+KeyV")
    }

    /// Register a hotkey from internal string format (e.g. "Ctrl+Shift+KeyV").
    /// Replaces any previously registered hotkey.
    pub fn register_str(&mut self, s: &str) -> Result<()> {
        let (modifiers, code) = parse_hotkey(s)?;
        self.register(modifiers, code)?;
        self.hotkey_str = Some(s.to_string());
        Ok(())
    }

    /// Register using raw modifiers + code.
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

    pub fn unregister(&mut self) -> Result<()> {
        if let Some(hotkey) = self.hotkey.take() {
            self.manager
                .unregister(hotkey)
                .context("Failed to unregister hotkey")?;
            info!("Unregistered hotkey: {:?}", hotkey);
        }
        Ok(())
    }

    /// Internal format string, e.g. "Ctrl+Shift+KeyV"
    pub fn hotkey_string(&self) -> Option<&str> {
        self.hotkey_str.as_deref()
    }

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

// ── Parsing ──────────────────────────────────────────────────────────────────

/// Parse an internal hotkey string into (Modifiers, Code).
///
/// Format: modifier(s) + KeyboardEvent.code key, joined by '+'.
/// Examples: "Ctrl+Shift+KeyV", "Alt+F1", "Ctrl+Digit1"
pub fn parse_hotkey(s: &str) -> Result<(Modifiers, Code)> {
    let mut modifiers = Modifiers::empty();
    let mut key_code: Option<Code> = None;

    for part in s.split('+') {
        match part.trim() {
            "Ctrl" | "Control" => modifiers |= Modifiers::CONTROL,
            "Shift" => modifiers |= Modifiers::SHIFT,
            "Alt" => modifiers |= Modifiers::ALT,
            "Win" | "Super" | "Meta" => modifiers |= Modifiers::SUPER,
            k => {
                key_code = Some(str_to_code(k).with_context(|| format!("Unknown key: {k}"))?);
            }
        }
    }

    let code = key_code.context("No key specified in hotkey string")?;
    if modifiers.is_empty() {
        return Err(anyhow::anyhow!("Hotkey must include at least one modifier key"));
    }
    Ok((modifiers, code))
}

/// Convert a hotkey internal string to a human-readable label.
/// "Ctrl+Shift+KeyV" → "Ctrl+Shift+V"
pub fn hotkey_to_display(s: &str) -> String {
    s.split('+')
        .map(|part| {
            if let Some(k) = part.strip_prefix("Key") {
                k.to_string()
            } else if let Some(d) = part.strip_prefix("Digit") {
                d.to_string()
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn str_to_code(s: &str) -> Result<Code> {
    let code = match s {
        // Letters
        "KeyA" => Code::KeyA, "KeyB" => Code::KeyB, "KeyC" => Code::KeyC,
        "KeyD" => Code::KeyD, "KeyE" => Code::KeyE, "KeyF" => Code::KeyF,
        "KeyG" => Code::KeyG, "KeyH" => Code::KeyH, "KeyI" => Code::KeyI,
        "KeyJ" => Code::KeyJ, "KeyK" => Code::KeyK, "KeyL" => Code::KeyL,
        "KeyM" => Code::KeyM, "KeyN" => Code::KeyN, "KeyO" => Code::KeyO,
        "KeyP" => Code::KeyP, "KeyQ" => Code::KeyQ, "KeyR" => Code::KeyR,
        "KeyS" => Code::KeyS, "KeyT" => Code::KeyT, "KeyU" => Code::KeyU,
        "KeyV" => Code::KeyV, "KeyW" => Code::KeyW, "KeyX" => Code::KeyX,
        "KeyY" => Code::KeyY, "KeyZ" => Code::KeyZ,
        // Digits
        "Digit0" => Code::Digit0, "Digit1" => Code::Digit1, "Digit2" => Code::Digit2,
        "Digit3" => Code::Digit3, "Digit4" => Code::Digit4, "Digit5" => Code::Digit5,
        "Digit6" => Code::Digit6, "Digit7" => Code::Digit7, "Digit8" => Code::Digit8,
        "Digit9" => Code::Digit9,
        // Function keys
        "F1"  => Code::F1,  "F2"  => Code::F2,  "F3"  => Code::F3,
        "F4"  => Code::F4,  "F5"  => Code::F5,  "F6"  => Code::F6,
        "F7"  => Code::F7,  "F8"  => Code::F8,  "F9"  => Code::F9,
        "F10" => Code::F10, "F11" => Code::F11, "F12" => Code::F12,
        // Other common keys
        "Space"     => Code::Space,
        "Tab"       => Code::Tab,
        "Enter"     => Code::Enter,
        "Backspace" => Code::Backspace,
        "Delete"    => Code::Delete,
        "Insert"    => Code::Insert,
        "Home"      => Code::Home,
        "End"       => Code::End,
        "PageUp"    => Code::PageUp,
        "PageDown"  => Code::PageDown,
        "ArrowUp"   => Code::ArrowUp,
        "ArrowDown" => Code::ArrowDown,
        "ArrowLeft" => Code::ArrowLeft,
        "ArrowRight"=> Code::ArrowRight,
        _ => return Err(anyhow::anyhow!("Unsupported key code: {s}")),
    };
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey() {
        let (mods, code) = parse_hotkey("Ctrl+Shift+KeyV").unwrap();
        assert!(mods.contains(Modifiers::CONTROL));
        assert!(mods.contains(Modifiers::SHIFT));
        assert_eq!(code, Code::KeyV);
    }

    #[test]
    fn test_parse_hotkey_no_modifier_fails() {
        assert!(parse_hotkey("KeyV").is_err());
    }

    #[test]
    fn test_hotkey_to_display() {
        assert_eq!(hotkey_to_display("Ctrl+Shift+KeyV"), "Ctrl+Shift+V");
        assert_eq!(hotkey_to_display("Alt+Digit1"), "Alt+1");
        assert_eq!(hotkey_to_display("Ctrl+F1"), "Ctrl+F1");
    }

    #[test]
    fn test_hotkey_manager_creation() {
        assert!(HotkeyManager::new().is_ok());
    }

    #[test]
    fn test_register_default_hotkey() {
        let mut manager = HotkeyManager::new().unwrap();
        assert!(manager.register_default().is_ok());
        assert_eq!(manager.hotkey_string(), Some("Ctrl+Shift+KeyV"));
    }
}
