//! 固高 gts.dll FFI 绑定

#[cfg(target_os = "windows")]
const DLL_NAME: &str = "gts.dll";

#[cfg(target_os = "linux")]
const DLL_NAME: &str = "libgts.so";

// ============ 初始化 ============

#[link(name = "gts")]
extern "C" {
    /// 初始化控制器
    pub fn GT_Init() -> i16;
}

// ============ 轴控制 ============

#[link(name = "gts")]
extern "C" {
    /// 轴使能
    pub fn GT_AxisOn(axis: i16) -> i16;
    /// 轴关闭
    pub fn GT_AxisOff(axis: i16) -> i16;
    /// 清除停止标志
    pub fn GT_ClrStp(axis: i16) -> i16;
}

// ============ 运动模式 ============

#[link(name = "gts")]
extern "C" {
    /// 梯形运动模式
    pub fn GT_PrfT(axis: i16) -> i16;
    /// 点动模式
    pub fn GT_PrfJog(axis: i16) -> i16;
    /// S曲线模式
    pub fn GT_PrfS(axis: i16) -> i16;
}

// ============ 参数设置 ============

#[link(name = "gts")]
extern "C" {
    /// 设置目标位置
    pub fn GT_SetPos(axis: i16, pos: i32) -> i16;
    /// 设置目标速度
    pub fn GT_SetVel(axis: i16, vel: f64) -> i16;
    /// 设置加速度
    pub fn GT_SetAcc(axis: i16, acc: f64) -> i16;
    /// 设置减速度
    pub fn GT_SetDec(axis: i16, dec: f64) -> i16;
}

// ============ 状态查询 ============

#[link(name = "gts")]
extern "C" {
    /// 获取规划位置
    pub fn GT_GetPrfPos(axis: i16) -> i32;
    /// 获取规划速度
    pub fn GT_GetPrfVel(axis: i16) -> f64;
    /// 获取轴状态
    pub fn GT_GetAxisStatus(axis: i16, p_status: *mut i32) -> i16;
}

// ============ 停止/回零 ============

#[link(name = "gts")]
extern "C" {
    /// 停止轴
    pub fn GT_Stop(axis: i16) -> i16;
    /// 回零
    pub fn GT_Home(axis: i16, mode: i16) -> i16;
}

// ============ IO 操作 ============

#[link(name = "gts")]
extern "C" {
    /// 读取数字输入
    pub fn GT_GetDI(port: i16) -> i16;
    /// 设置数字输出
    pub fn GT_SetDO(port: i16, value: i16) -> i16;
}
