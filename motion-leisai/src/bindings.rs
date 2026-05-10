//! 雷赛 LTSMC.dll FFI 绑定

#[cfg(target_os = "windows")]
const DLL_NAME: &str = "LTSMC.dll";

#[cfg(target_os = "linux")]
const DLL_NAME: &str = "libLTSMC.so";

#[cfg(target_os = "macos")]
const DLL_NAME: &str = "libLTSMC.dylib";

// ============ 板卡管理 ============

/// 初始化板卡
#[link(name = "LTSMC")]
extern "C" {
    pub fn nmc_board_init() -> i16;
    pub fn nmc_board_close();
}

// ============ 轴使能 ============

#[link(name = "LTSMC")]
extern "C" {
    /// 设置轴使能
    pub fn nmc_set_axis_enable(card: u16, axis: u16, enable: bool) -> i16;
    /// 获取轴使能状态
    pub fn nmc_axis_get_enable_status(card: u16, axis: u16) -> i16;
}

// ============ 运动控制 ============

#[link(name = "LTSMC")]
extern "C" {
    /// 绝对定位
    pub fn nmc_axis_move_absolute(card: u16, axis: u16, pos: f64, vel: f64) -> i16;
    /// 相对定位
    pub fn nmc_axis_move_relative(card: u16, axis: u16, dist: f64, vel: f64) -> i16;
    /// 速度模式 (点动)
    pub fn nmc_axis_move_vel(card: u16, axis: u16, vel: f64) -> i16;
    /// 停止轴
    pub fn nmc_axis_stop(card: u16, axis: u16) -> i16;
    /// 检查轴运动是否完成
    pub fn nmc_axis_check_done(card: u16, axis: u16) -> i16;
    /// 回零
    pub fn nmc_axis_move_home(card: u16, axis: u16, mode: i16) -> i16;
}

// ============ 位置/速度查询 ============

#[link(name = "LTSMC")]
extern "C" {
    /// 获取实际位置
    pub fn nmc_axis_get_actual_pos(card: u16, axis: u16) -> f64;
    /// 获取实际速度
    pub fn nmc_axis_get_actual_vel(card: u16, axis: u16) -> f64;
    /// 设置位置
    pub fn nmc_axis_set_position(card: u16, axis: u16, pos: f64) -> i16;
}

// ============ 参数设置 ============

#[link(name = "LTSMC")]
extern "C" {
    /// 设置速度
    pub fn nmc_axis_set_vel(card: u16, axis: u16, vel: f64) -> i16;
    /// 设置加速度
    pub fn nmc_axis_set_acc(card: u16, axis: u16, acc: f64) -> i16;
    /// 设置减速度
    pub fn nmc_axis_set_dec(card: u16, axis: u16, dec: f64) -> i16;
    /// 设置软限位
    pub fn nmc_axis_set_soft_limit(card: u16, axis: u16, min: f64, max: f64, enable: bool) -> i16;
}

// ============ 状态查询 ============

#[link(name = "LTSMC")]
extern "C" {
    /// 获取报警状态
    pub fn nmc_axis_get_alarm(card: u16, axis: u16) -> i16;
    /// 正向限位
    pub fn nmc_get_inp(card: u16, axis: u16) -> i16;
    /// 负向限位
    pub fn nmc_get_inn(card: u16, axis: u16) -> i16;
}

// ============ IO 操作 ============

#[link(name = "LTSMC")]
extern "C" {
    /// 读取单路输入
    pub fn nmc_get_inbit(card: u16, port: u16) -> i16;
    /// 写入单路输出
    pub fn nmc_set_outbit(card: u16, port: u16, value: bool) -> i16;
    /// 读取端口输入
    pub fn nmc_get_inport(card: u16, port: u16) -> u16;
    /// 写入端口输出
    pub fn nmc_set_outport(card: u16, port: u16, value: u16) -> i16;
}
