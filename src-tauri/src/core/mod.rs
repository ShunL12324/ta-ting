// TaTing 核心状态机模块
pub mod state_machine;
pub mod app;

pub use state_machine::{AppState, StateMachine};
pub use app::TaTingApp;
