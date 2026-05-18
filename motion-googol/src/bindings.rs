//! 固高 gts.dll FFI 绑定
//!
//! 各厂商 SDK 的原始函数声明。实际加载由链接器处理。

#[cfg(target_os = "windows")]
#[allow(dead_code)]
const DLL_NAME: &str = "gts.dll";

#[cfg(target_os = "linux")]
#[allow(dead_code)]
const DLL_NAME: &str = "libgts.so";

#[link(name = "gts")]
extern "C" {
    // 初始化
    pub fn GT_Init() -> i16;

    // 轴控制
    pub fn GT_AxisOn(axis: i16) -> i16;
    pub fn GT_AxisOff(axis: i16) -> i16;
    pub fn GT_ClrStp(axis: i16) -> i16;

    // 运动模式
    pub fn GT_PrfT(axis: i16) -> i16;
    pub fn GT_PrfJog(axis: i16) -> i16;
    #[allow(dead_code)]
    pub fn GT_PrfS(axis: i16) -> i16;

    // 参数设置
    pub fn GT_SetPos(axis: i16, pos: i32) -> i16;
    pub fn GT_SetVel(axis: i16, vel: f64) -> i16;
    pub fn GT_SetAcc(axis: i16, acc: f64) -> i16;
    pub fn GT_SetDec(axis: i16, dec: f64) -> i16;

    // 状态查询
    pub fn GT_GetPrfPos(axis: i16) -> i32;
    pub fn GT_GetPrfVel(axis: i16) -> f64;
    pub fn GT_GetAxisStatus(axis: i16, p_status: *mut i32) -> i16;

    // 停止/回零
    pub fn GT_Stop(axis: i16) -> i16;
    pub fn GT_Home(axis: i16, mode: i16) -> i16;

    // IO 操作
    pub fn GT_GetDI(port: i16) -> i16;
    pub fn GT_SetDO(port: i16, value: i16) -> i16;
}
