use thiserror::Error;

/// 运动控制错误类型
#[derive(Error, Debug)]
pub enum MotionError {
    #[error("板卡未初始化")]
    NotInitialized,

    #[error("板卡连接失败: {0}")]
    ConnectionFailed(String),

    #[error("轴索引越界: {index}, 最大: {max}")]
    AxisOutOfRange { index: usize, max: usize },

    #[error("运动失败: {0}")]
    MoveFailed(String),

    #[error("IO 操作失败: {0}")]
    IoFailed(String),

    #[error("不支持的板卡类型: {0}")]
    UnsupportedVendor(String),

    #[error("SDK 调用失败, 错误码: {code}")]
    SdkError { code: i16, message: String },

    #[error("动态库加载失败: {0}")]
    LibraryLoadFailed(String),

    #[error("配置错误: {0}")]
    ConfigError(String),
}

/// 统一结果类型
pub type Result<T> = std::result::Result<T, MotionError>;
