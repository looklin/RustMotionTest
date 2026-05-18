//! 研华 pci124x.dll FFI 绑定
//!
//! 各厂商 SDK 的原始函数声明。实际加载由链接器处理。

#[link(name = "pci124x")]
extern "C" {
    // 初始化
    pub fn APS_initial(device: *mut u16) -> i32;
    pub fn APS_close(device: u16) -> i32;

    // 轴使能
    pub fn APS_set_io_svon(device: u16, channel: u16, state: u16) -> i32;
    pub fn APS_get_io_svon(device: u16, channel: u16, state: *mut u16) -> i32;

    // 运动控制
    pub fn APS_move(device: u16, channel: u16, mode: i32, pos: i32, vel: i32) -> i32;
    pub fn APS_conti_t_move_abs(device: u16, channel: u16, pos: i32, vel: i32) -> i32;
    pub fn APS_immediate_stop(device: u16, channel: u16) -> i32;
    #[allow(dead_code)]
    pub fn APS_deceleration_stop(device: u16, channel: u16) -> i32;
    pub fn APS_home(device: u16, channel: u16, mode: i32) -> i32;

    // 位置/速度查询
    pub fn APS_get_command_pos(device: u16, channel: u16, pos: *mut i32) -> i32;
    pub fn APS_get_command_vel(device: u16, channel: u16, vel: *mut i32) -> i32;
    pub fn APS_set_command_pos(device: u16, channel: u16, pos: i32) -> i32;

    // 参数设置
    pub fn APS_set_vel(device: u16, channel: u16, vel: i32) -> i32;
    pub fn APS_set_acc(device: u16, channel: u16, acc: i32) -> i32;
    pub fn APS_set_dec(device: u16, channel: u16, dec: i32) -> i32;

    // 状态查询
    pub fn APS_get_status(device: u16, channel: u16, status: *mut u32) -> i32;

    // IO 操作
    pub fn APS_get_DI(device: u16, di_state: *mut u32) -> i32;
    pub fn APS_set_DO(device: u16, channel: u16, state: u16) -> i32;
}
