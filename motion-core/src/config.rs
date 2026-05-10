/// 板卡配置
#[derive(Debug, Clone)]
pub struct CardConfig {
    /// 板卡编号/ID
    pub board_id: u16,
    /// 轴数量
    pub axis_count: usize,
    /// 配置文件路径 (可选)
    pub config_path: Option<String>,
    /// 通讯超时 (毫秒)
    pub timeout_ms: u64,
    /// 是否启用调试日志
    pub debug: bool,
}

impl Default for CardConfig {
    fn default() -> Self {
        Self {
            board_id: 0,
            axis_count: 4,
            config_path: None,
            timeout_ms: 5000,
            debug: false,
        }
    }
}

impl CardConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置轴数量
    pub fn with_axis_count(mut self, count: usize) -> Self {
        self.axis_count = count;
        self
    }

    /// 设置板卡 ID
    pub fn with_board_id(mut self, id: u16) -> Self {
        self.board_id = id;
        self
    }

    /// 设置超时
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// 启用调试模式
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}
