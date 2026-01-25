//! 键盘输入模拟器
//!
//! 负责将文本通过剪贴板粘贴的方式输出到当前光标位置。
//!
//! ## 功能
//! - 通过剪贴板输入文本（支持中文）
//! - 自动保存和恢复原剪贴板内容
//! - 兼容性好，适用于所有应用
//!
//! ## 使用示例
//! ```rust,no_run
//! use ta_ting::system::input::InputSimulator;
//!
//! let mut simulator = InputSimulator::new().unwrap();
//! simulator.type_text("你好，世界！").unwrap();
//! ```

use anyhow::Result;
use arboard::Clipboard;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use log::{debug, info, warn};
use std::thread;
use std::time::Duration;

/// 键盘输入模拟器
pub struct InputSimulator {
    enigo: Enigo,
}

impl InputSimulator {
    /// 创建新的输入模拟器
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&Settings::default())?;
        Ok(Self { enigo })
    }

    /// 模拟键盘输入文本（通过剪贴板粘贴）
    ///
    /// # 参数
    /// - `text`: 要输入的文本（支持中英文、标点符号等）
    ///
    /// # 返回
    /// - `Ok(())`: 输入成功
    /// - `Err`: 输入失败
    ///
    /// # 实现说明
    /// 1. 保存当前剪贴板内容
    /// 2. 将文本复制到剪贴板
    /// 3. 模拟 Ctrl+V 粘贴
    /// 4. 恢复原剪贴板内容（保证不影响用户剪贴板）
    pub fn type_text(&mut self, text: &str) -> Result<()> {
        if text.is_empty() {
            debug!("文本为空，跳过输入");
            return Ok(());
        }

        info!("开始通过剪贴板输入: {} 字符", text.chars().count());

        // 短暂延迟，确保目标窗口已获得焦点
        thread::sleep(Duration::from_millis(100));

        // 创建剪贴板实例
        let mut clipboard = Clipboard::new()?;

        // 1. 保存原剪贴板内容
        let original_content = match clipboard.get_text() {
            Ok(content) => {
                info!("已保存原剪贴板内容 ({} 字符)", content.chars().count());
                Some(content)
            }
            Err(_) => {
                debug!("剪贴板为空或无文本内容");
                None
            }
        };

        // 2. 将文本复制到剪贴板
        clipboard.set_text(text)?;
        debug!("文本已复制到剪贴板");

        // 短暂延迟，确保剪贴板操作完成
        thread::sleep(Duration::from_millis(50));

        // 3. 模拟 Ctrl+V 粘贴
        self.enigo.key(Key::Control, Press)?;
        thread::sleep(Duration::from_millis(10));
        self.enigo.key(Key::Unicode('v'), Click)?;
        thread::sleep(Duration::from_millis(10));
        self.enigo.key(Key::Control, Release)?;

        debug!("粘贴操作完成");

        // 延迟一下再恢复剪贴板，确保粘贴完成
        thread::sleep(Duration::from_millis(150));

        // 4. 恢复原剪贴板内容
        match original_content {
            Some(original) => {
                match clipboard.set_text(&original) {
                    Ok(_) => {
                        info!("✅ 已恢复原剪贴板内容");
                    }
                    Err(e) => {
                        warn!("⚠️  恢复剪贴板内容失败: {}", e);
                    }
                }
            }
            None => {
                // 如果原来剪贴板是空的，清空剪贴板
                if let Err(e) = clipboard.clear() {
                    debug!("清空剪贴板失败: {}", e);
                } else {
                    info!("✅ 已清空剪贴板（原本就是空的）");
                }
            }
        }

        info!("键盘输入完成");

        Ok(())
    }

    /// 模拟按下回车键
    pub fn press_enter(&mut self) -> Result<()> {
        self.enigo.key(Key::Return, Click)?;
        Ok(())
    }

    /// 模拟按下退格键
    pub fn press_backspace(&mut self) -> Result<()> {
        self.enigo.key(Key::Backspace, Click)?;
        Ok(())
    }

    /// 模拟按下 Ctrl+V（粘贴）
    pub fn paste(&mut self) -> Result<()> {
        self.enigo.key(Key::Control, Press)?;
        self.enigo.key(Key::Unicode('v'), Click)?;
        self.enigo.key(Key::Control, Release)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_creation() {
        let _simulator = InputSimulator::new();
        assert!(_simulator.is_ok());
    }

    // 注意：实际输入测试需要在有窗口焦点的环境中运行
    // 这里只测试不会 panic
    #[test]
    fn test_empty_text() {
        let mut simulator = InputSimulator::new().unwrap();
        let result = simulator.type_text("");
        assert!(result.is_ok());
    }
}
