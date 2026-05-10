/// 板卡厂商枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardVendor {
    /// 雷赛 (Leadshine)
    Leisai,
    /// 固高 (Googol)
    Googol,
    /// 研华 (Advantech)
    Advantech,
    /// 模拟板卡 (用于无硬件时测试)
    Mock,
}

impl std::fmt::Display for CardVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardVendor::Leisai => write!(f, "雷赛 Leadshine"),
            CardVendor::Googol => write!(f, "固高 Googol"),
            CardVendor::Advantech => write!(f, "研华 Advantech"),
            CardVendor::Mock => write!(f, "模拟 Mock"),
        }
    }
}

/// 回零模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeMode {
    /// 自动回零
    Auto,
    /// 正向限位回零
    PositiveLimit,
    /// 负向限位回零
    NegativeLimit,
    /// 编码器Z相信号回零
    EncoderZ,
}

/// 停止模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopMode {
    /// 立即停止 (急停)
    Immediate,
    /// 减速停止 (平滑停止)
    Decelerate,
}

/// 轴状态
#[derive(Debug, Clone, Copy, Default)]
pub struct AxisStatus {
    /// 轴是否就绪
    pub is_ready: bool,
    /// 是否正在回零
    pub is_homing: bool,
    /// 正向限位触发
    pub is_limit_positive: bool,
    /// 负向限位触发
    pub is_limit_negative: bool,
    /// 伺服是否开启
    pub is_servo_on: bool,
    /// 是否有报警
    pub is_alarm: bool,
}

/// 板卡状态
#[derive(Debug, Clone)]
pub struct CardStatus {
    /// 是否已初始化
    pub is_initialized: bool,
    /// 错误计数
    pub error_count: u32,
    /// 最后错误信息
    pub last_error: String,
    /// 运行时间
    pub uptime: std::time::Duration,
}

impl Default for CardStatus {
    fn default() -> Self {
        Self {
            is_initialized: false,
            error_count: 0,
            last_error: String::new(),
            uptime: std::time::Duration::ZERO,
        }
    }
}
