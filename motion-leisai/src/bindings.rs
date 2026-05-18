//! 雷赛 LTSMC.dll FFI 绑定
//!
//! 各厂商 SDK 的原始函数声明。实际加载由链接器处理。

#[cfg(target_os = "windows")]
#[allow(dead_code)]
const DLL_NAME: &str = "LTSMC.dll";

#[cfg(target_os = "linux")]
#[allow(dead_code)]
const DLL_NAME: &str = "libLTSMC.so";

#[cfg(target_os = "macos")]
#[allow(dead_code)]
const DLL_NAME: &str = "libLTSMC.dylib";

#[link(name = "LTSMC")]
extern "C" {
    // 板卡管理
    pub fn nmc_board_init() -> i16;
    pub fn nmc_board_close();

    // 轴使能
    pub fn nmc_set_axis_enable(card: u16, axis: u16, enable: bool) -> i16;
    pub fn nmc_axis_get_enable_status(card: u16, axis: u16) -> i16;

    // 运动控制
    pub fn nmc_axis_move_absolute(card: u16, axis: u16, pos: f64, vel: f64) -> i16;
    pub fn nmc_axis_move_relative(card: u16, axis: u16, dist: f64, vel: f64) -> i16;
    pub fn nmc_axis_move_vel(card: u16, axis: u16, vel: f64) -> i16;
    pub fn nmc_axis_stop(card: u16, axis: u16) -> i16;
    pub fn nmc_axis_check_done(card: u16, axis: u16) -> i16;
    pub fn nmc_axis_move_home(card: u16, axis: u16, mode: i16) -> i16;

    // 位置/速度
    pub fn nmc_axis_get_actual_pos(card: u16, axis: u16) -> f64;
    pub fn nmc_axis_get_actual_vel(card: u16, axis: u16) -> f64;
    pub fn nmc_axis_set_position(card: u16, axis: u16, pos: f64) -> i16;

    // 参数设置
    pub fn nmc_axis_set_vel(card: u16, axis: u16, vel: f64) -> i16;
    pub fn nmc_axis_set_acc(card: u16, axis: u16, acc: f64) -> i16;
    pub fn nmc_axis_set_dec(card: u16, axis: u16, dec: f64) -> i16;
    pub fn nmc_axis_set_soft_limit(card: u16, axis: u16, min: f64, max: f64, enable: bool) -> i16;

    // 状态查询
    pub fn nmc_axis_get_alarm(card: u16, axis: u16) -> i16;
    pub fn nmc_get_inp(card: u16, axis: u16) -> i16;
    pub fn nmc_get_inn(card: u16, axis: u16) -> i16;

    // IO 操作
    pub fn nmc_get_inbit(card: u16, port: u16) -> i16;
    pub fn nmc_set_outbit(card: u16, port: u16, value: bool) -> i16;
    pub fn nmc_get_inport(card: u16, port: u16) -> u16;
    pub fn nmc_set_outport(card: u16, port: u16, value: u16) -> i16;
}
