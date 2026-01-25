//! TaTing 状态机
//!
//! 管理应用的状态转换：
//! Idle -> Recording -> Transcribing -> Inputting -> Idle

use anyhow::Result;
use log::{debug, info, warn};
use std::fmt;

/// 应用状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    /// 空闲状态，等待热键触发
    Idle,
    /// 正在录音
    Recording,
    /// 正在转录
    Transcribing,
    /// 正在输入文本
    Inputting,
    /// 错误状态
    Error,
}

impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppState::Idle => write!(f, "空闲"),
            AppState::Recording => write!(f, "录音中"),
            AppState::Transcribing => write!(f, "转录中"),
            AppState::Inputting => write!(f, "输入中"),
            AppState::Error => write!(f, "错误"),
        }
    }
}

/// 状态转换事件
#[derive(Debug, Clone)]
pub enum StateEvent {
    /// 热键按下
    HotkeyPressed,
    /// 录音完成（有音频数据）
    RecordingCompleted(Vec<f32>),
    /// 转录完成（有文本）
    TranscriptionCompleted(String),
    /// 输入完成
    InputCompleted,
    /// 发生错误
    Error(String),
}

/// 状态机
pub struct StateMachine {
    current_state: AppState,
}

impl StateMachine {
    /// 创建新的状态机（初始状态为 Idle）
    pub fn new() -> Self {
        info!("状态机初始化");
        Self {
            current_state: AppState::Idle,
        }
    }

    /// 获取当前状态
    pub fn current_state(&self) -> AppState {
        self.current_state
    }

    /// 处理事件并转换状态
    ///
    /// # 返回
    /// - `Ok(new_state)`: 成功转换到新状态
    /// - `Err`: 无效的状态转换
    pub fn handle_event(&mut self, event: StateEvent) -> Result<AppState> {
        let old_state = self.current_state;

        let new_state = match (&self.current_state, &event) {
            // Idle -> Recording (热键按下)
            (AppState::Idle, StateEvent::HotkeyPressed) => {
                info!("事件: 热键按下，开始录音");
                AppState::Recording
            }

            // Recording -> Transcribing (热键再次按下或录音完成)
            (AppState::Recording, StateEvent::HotkeyPressed) => {
                info!("事件: 热键按下，停止录音");
                AppState::Transcribing
            }

            (AppState::Recording, StateEvent::RecordingCompleted(_)) => {
                info!("事件: 录音完成（VAD 检测）");
                AppState::Transcribing
            }

            // Transcribing -> Inputting (转录完成)
            (AppState::Transcribing, StateEvent::TranscriptionCompleted(text)) => {
                info!("事件: 转录完成 ({} 字符)", text.chars().count());
                AppState::Inputting
            }

            // Inputting -> Idle (输入完成)
            (AppState::Inputting, StateEvent::InputCompleted) => {
                info!("事件: 输入完成，回到空闲");
                AppState::Idle
            }

            // Error -> Idle (错误后恢复)
            (AppState::Error, _) => {
                warn!("从错误状态恢复到空闲");
                AppState::Idle
            }

            // 任何状态遇到错误 -> Error
            (_, StateEvent::Error(msg)) => {
                warn!("错误发生: {}", msg);
                AppState::Error
            }

            // 无效的状态转换
            _ => {
                let msg = format!(
                    "无效的状态转换: {} -> {:?}",
                    old_state, event
                );
                warn!("{}", msg);
                return Err(anyhow::anyhow!(msg));
            }
        };

        if old_state != new_state {
            debug!("状态转换: {} -> {}", old_state, new_state);
            self.current_state = new_state;
        }

        Ok(new_state)
    }

    /// 重置状态机到 Idle
    pub fn reset(&mut self) {
        info!("重置状态机");
        self.current_state = AppState::Idle;
    }

    /// 检查是否可以处理热键（只在 Idle 或 Recording 状态）
    pub fn can_handle_hotkey(&self) -> bool {
        matches!(self.current_state, AppState::Idle | AppState::Recording)
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_flow() {
        let mut sm = StateMachine::new();
        assert_eq!(sm.current_state(), AppState::Idle);

        // Idle -> Recording
        sm.handle_event(StateEvent::HotkeyPressed).unwrap();
        assert_eq!(sm.current_state(), AppState::Recording);

        // Recording -> Transcribing
        sm.handle_event(StateEvent::HotkeyPressed).unwrap();
        assert_eq!(sm.current_state(), AppState::Transcribing);

        // Transcribing -> Inputting
        sm.handle_event(StateEvent::TranscriptionCompleted("测试".to_string()))
            .unwrap();
        assert_eq!(sm.current_state(), AppState::Inputting);

        // Inputting -> Idle
        sm.handle_event(StateEvent::InputCompleted).unwrap();
        assert_eq!(sm.current_state(), AppState::Idle);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new();

        // 在 Idle 状态下收到 RecordingCompleted 是无效的
        let result = sm.handle_event(StateEvent::RecordingCompleted(vec![]));
        assert!(result.is_err());
    }

    #[test]
    fn test_error_handling() {
        let mut sm = StateMachine::new();
        sm.handle_event(StateEvent::HotkeyPressed).unwrap();

        // 任何状态遇到错误都会转到 Error
        sm.handle_event(StateEvent::Error("测试错误".to_string()))
            .unwrap();
        assert_eq!(sm.current_state(), AppState::Error);

        // Error 状态下任何事件都会回到 Idle
        sm.handle_event(StateEvent::HotkeyPressed).unwrap();
        assert_eq!(sm.current_state(), AppState::Idle);
    }
}
