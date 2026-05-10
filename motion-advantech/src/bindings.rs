//! 研华 pci124x.dll FFI 绑定

// ============ 初始化 ============

#[link(name = "pci124x")]
extern "C" {
    /// 初始化板卡
    pub fn APS_initial(device: *mut u16) -> i32;
    /// 关闭板卡
    pub fn APS_close(device: u16) -> i32;
}

// ============ 轴使能 ============

#[link(name = "pci124x")]
extern "C" {
    /// 设置伺服使能
    pub fn APS_set_io_svon(device: u16, channel: u16, state: u16) -> i32;
    /// 获取伺服使能状态
    pub fn APS_get_io_svon(device: u16, channel: u16, state: *mut u16) -> i32;
}

// ============ 运动控制 ============

#[link(name = "pci124x")]
extern "C" {
    /// 定位运动 (mode: 0=绝对, 1=相对)
    pub fn APS_move(device: u16, channel: u16, mode: i32, pos: i32, vel: i32) -> i32;
    /// 连续绝对运动 (点动)
    pub fn APS_conti_t_move_abs(device: u16, channel: u16, pos: i32, vel: i32) -> i32;
    /// 立即停止
    pub fn APS_immediate_stop(device: u16, channel: u16) -> i32;
    /// 减速停止
    pub fn APS_deceleration_stop(device: u16, channel: u16) -> i32;
    /// 回零
    pub fn APS_home(device: u16, channel: u16, mode: i32) -> i32;
}

// ============ 位置/速度查询 ============

#[link(name = "pci124x")]
extern "C" {
    /// 获取命令位置
    pub fn APS_get_command_pos(device: u16, channel: u16, pos: *mut i32) -> i32;
    /// 获取命令速度
    pub fn APS_get_command_vel(device: u16, channel: u16, vel: *mut i32) -> i32;
    /// 设置命令位置
    pub fn APS_set_command_pos(device: u16, channel: u16, pos: i32) -> i32;
}

// ============ 参数设置 ============

#[link(name = "pci124x")]
extern "C" {
    /// 设置速度
    pub fn APS_set_vel(device: u16, channel: u16, vel: i32) -> i32;
    /// 设置加速度
    pub fn APS_set_acc(device: u16, channel: u16, acc: i32) -> i32;
    /// 设置减速度
    pub fn APS_set_dec(device: u16, channel: u16, dec: i32) -> i32;
}

// ============ 状态查询 ============

#[link(name = "pci124x")]
extern "C" {
    /// 获取轴状态
    pub fn APS_get_status(device: u16, channel: u16, status: *mut u32) -> i32;
}

// ============ IO 操作 ============

#[link(name = "pci124x")]
extern "C" {
    /// 读取数字输入
    pub fn APS_get_DI(device: u16, di_state: *mut u32) -> i32;
    /// 设置数字输出
    pub fn APS_set_DO(device: u16, channel: u16, state: u16) -> i32;
}
