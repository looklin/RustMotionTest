//! 核心 Trait 定义

use crate::config::CardConfig;
use crate::error::Result;
use crate::models::*;

/// 运动控制板卡统一接口
///
/// 所有厂商适配器都需要实现此 Trait。
pub trait MotionCard: Send + Sync {
    /// 初始化板卡
    fn initialize(&mut self, config: &CardConfig) -> Result<()>;

    /// 关闭板卡连接
    fn close(&mut self);

    /// 获取板卡状态
    fn status(&self) -> CardStatus;

    /// 板卡名称
    fn card_name(&self) -> &str;

    /// 板卡类型描述
    fn card_type(&self) -> &str;

    /// 轴数量
    fn axis_count(&self) -> usize;

    /// 输入端口数量
    fn input_count(&self) -> usize;

    /// 输出端口数量
    fn output_count(&self) -> usize;

    /// 获取指定轴 (可变引用)
    fn get_axis(&mut self, index: usize) -> Result<&mut dyn Axis>;

    /// 读取数字输入
    fn read_input(&self, port: u16) -> bool;

    /// 写入数字输出
    fn write_output(&mut self, port: u16, value: bool);

    /// 读取所有输入状态 (位掩码)
    fn read_all_inputs(&self) -> u16;

    /// 写入所有输出状态 (位掩码)
    fn write_all_outputs(&mut self, value: u16);

    /// 绝对定位运动
    fn move_absolute(&mut self, axis: usize, position: f64, velocity: f64) -> Result<()>;

    /// 相对定位运动
    fn move_relative(&mut self, axis: usize, distance: f64, velocity: f64) -> Result<()>;

    /// 回零运动
    fn move_home(&mut self, axis: usize, mode: HomeMode) -> Result<()>;

    /// 停止指定轴
    fn stop_axis(&mut self, axis: usize, mode: StopMode);

    /// 停止所有轴
    fn stop_all(&mut self);
}

/// 运动轴统一接口
pub trait Axis {
    /// 轴索引
    fn axis_index(&self) -> usize;

    /// 是否已使能
    fn is_enabled(&self) -> bool;

    /// 是否正在运动
    fn is_moving(&self) -> bool;

    /// 是否有报警
    fn is_alarm(&self) -> bool;

    /// 当前位置
    fn current_position(&self) -> f64;

    /// 当前速度
    fn current_velocity(&self) -> f64;

    /// 轴状态
    fn status(&self) -> AxisStatus;

    /// 使能轴
    fn enable(&mut self);

    /// 关闭轴使能
    fn disable(&mut self);

    /// 绝对定位
    fn move_absolute(&mut self, position: f64, velocity: f64);

    /// 相对定位
    fn move_relative(&mut self, distance: f64, velocity: f64);

    /// 正向点动
    fn jog_positive(&mut self, velocity: f64);

    /// 负向点动
    fn jog_negative(&mut self, velocity: f64);

    /// 停止运动
    fn stop(&mut self, mode: StopMode);

    /// 回零
    fn move_home(&mut self, mode: HomeMode);

    /// 设置速度
    fn set_velocity(&mut self, velocity: f64);

    /// 设置加速度
    fn set_acceleration(&mut self, accel: f64);

    /// 设置减速度
    fn set_deceleration(&mut self, decel: f64);

    /// 设置当前位置 (软件偏移)
    fn set_position(&mut self, position: f64);

    /// 启用软限位
    fn enable_soft_limit(&mut self, min: f64, max: f64);

    /// 禁用软限位
    fn disable_soft_limit(&mut self);
}
