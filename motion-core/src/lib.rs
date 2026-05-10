//! MotionTest Core — 通用运动控制板卡抽象层
//!
//! 提供统一的 `MotionCard` 和 `Axis` Trait，兼容主流板卡厂商。

pub mod traits;
pub mod models;
pub mod error;
pub mod config;
pub mod factory;

pub use error::{MotionError, Result};
pub use traits::{MotionCard, Axis};
pub use models::*;
pub use config::CardConfig;
pub use factory::CardFactory;
