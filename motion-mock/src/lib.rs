//! 模拟板卡适配器 — 用于无硬件时开发和测试

use motion_core::config::CardConfig;
use motion_core::error::Result;
use motion_core::models::*;
use motion_core::traits::{Axis as AxisTrait, MotionCard};
use std::collections::HashMap;

/// 模拟板卡
pub struct MockCard {
    initialized: bool,
    config: Option<CardConfig>,
    axes: HashMap<usize, MockAxis>,
    inputs: HashMap<u16, bool>,
    outputs: HashMap<u16, bool>,
    error_count: u32,
    last_error: String,
    init_time: std::time::Instant,
}

impl Default for MockCard {
    fn default() -> Self {
        Self::new()
    }
}

impl MockCard {
    pub fn new() -> Self {
        let mut inputs = HashMap::new();
        let mut outputs = HashMap::new();
        for i in 0..16u16 {
            inputs.insert(i, false);
            outputs.insert(i, false);
        }
        Self {
            initialized: false,
            config: None,
            axes: HashMap::new(),
            inputs,
            outputs,
            error_count: 0,
            last_error: String::new(),
            init_time: std::time::Instant::now(),
        }
    }

    /// 设置模拟输入状态 (供测试使用)
    pub fn set_input(&mut self, port: u16, value: bool) {
        self.inputs.insert(port, value);
    }
}

impl MotionCard for MockCard {
    fn initialize(&mut self, config: &CardConfig) -> Result<()> {
        self.config = Some(config.clone());
        self.initialized = true;
        self.init_time = std::time::Instant::now();
        for i in 0..config.axis_count {
            self.axes.insert(i, MockAxis::new(i));
        }
        Ok(())
    }

    fn close(&mut self) {
        self.initialized = false;
        for axis in self.axes.values_mut() {
            axis.disable();
        }
        self.axes.clear();
    }

    fn status(&self) -> CardStatus {
        CardStatus {
            is_initialized: self.initialized,
            error_count: self.error_count,
            last_error: self.last_error.clone(),
            uptime: if self.initialized {
                self.init_time.elapsed()
            } else {
                std::time::Duration::ZERO
            },
        }
    }

    fn card_name(&self) -> &str {
        "Mock Test Card"
    }

    fn card_type(&self) -> &str {
        "Mock"
    }

    fn axis_count(&self) -> usize {
        self.config.as_ref().map(|c| c.axis_count).unwrap_or(4)
    }

    fn input_count(&self) -> usize {
        16
    }

    fn output_count(&self) -> usize {
        16
    }

    fn get_axis(&mut self, index: usize) -> Result<&mut dyn AxisTrait> {
        let max = self.axis_count();
        self.axes.get_mut(&index).map(|a| a as &mut dyn AxisTrait).ok_or(
            motion_core::MotionError::AxisOutOfRange { index, max }
        )
    }

    fn read_input(&self, port: u16) -> bool {
        self.inputs.get(&port).copied().unwrap_or(false)
    }

    fn write_output(&mut self, port: u16, value: bool) {
        self.outputs.insert(port, value);
    }

    fn read_all_inputs(&self) -> u16 {
        let mut result: u16 = 0;
        for i in 0..16 {
            if self.inputs.get(&i).copied().unwrap_or(false) {
                result |= 1 << i;
            }
        }
        result
    }

    fn write_all_outputs(&mut self, value: u16) {
        for i in 0..16 {
            self.outputs.insert(i, (value & (1 << i)) != 0);
        }
    }

    fn move_absolute(&mut self, axis: usize, position: f64, velocity: f64) -> Result<()> {
        if !self.initialized {
            return Err(motion_core::MotionError::NotInitialized);
        }
        if let Some(a) = self.axes.get_mut(&axis) {
            a.move_absolute(position, velocity);
        }
        Ok(())
    }

    fn move_relative(&mut self, axis: usize, distance: f64, velocity: f64) -> Result<()> {
        if !self.initialized {
            return Err(motion_core::MotionError::NotInitialized);
        }
        if let Some(a) = self.axes.get_mut(&axis) {
            a.move_relative(distance, velocity);
        }
        Ok(())
    }

    fn move_home(&mut self, axis: usize, _mode: HomeMode) -> Result<()> {
        if !self.initialized {
            return Err(motion_core::MotionError::NotInitialized);
        }
        if let Some(a) = self.axes.get_mut(&axis) {
            a.move_home(HomeMode::Auto);
        }
        Ok(())
    }

    fn stop_axis(&mut self, axis: usize, _mode: StopMode) {
        if let Some(a) = self.axes.get_mut(&axis) {
            a.stop(StopMode::Immediate);
        }
    }

    fn stop_all(&mut self) {
        for a in self.axes.values_mut() {
            a.stop(StopMode::Immediate);
        }
    }
}

/// 模拟轴
pub struct MockAxis {
    axis_index: usize,
    enabled: bool,
    moving: bool,
    alarm: bool,
    position: f64,
    velocity: f64,
    soft_limit_min: Option<f64>,
    soft_limit_max: Option<f64>,
}

impl MockAxis {
    pub fn new(index: usize) -> Self {
        Self {
            axis_index: index,
            enabled: false,
            moving: false,
            alarm: false,
            position: 0.0,
            velocity: 0.0,
            soft_limit_min: None,
            soft_limit_max: None,
        }
    }
}

impl AxisTrait for MockAxis {
    fn axis_index(&self) -> usize {
        self.axis_index
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn is_moving(&self) -> bool {
        self.moving
    }

    fn is_alarm(&self) -> bool {
        self.alarm
    }

    fn current_position(&self) -> f64 {
        self.position
    }

    fn current_velocity(&self) -> f64 {
        self.velocity
    }

    fn status(&self) -> AxisStatus {
        AxisStatus {
            is_ready: self.enabled && !self.alarm,
            is_homing: false,
            is_limit_positive: false,
            is_limit_negative: false,
            is_servo_on: self.enabled,
            is_alarm: self.alarm,
        }
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
        self.moving = false;
    }

    fn move_absolute(&mut self, position: f64, velocity: f64) {
        self.position = position;
        self.velocity = velocity;
        self.moving = false;
    }

    fn move_relative(&mut self, distance: f64, velocity: f64) {
        self.position += distance;
        self.velocity = velocity;
        self.moving = false;
    }

    fn jog_positive(&mut self, velocity: f64) {
        self.moving = true;
        self.velocity = velocity;
    }

    fn jog_negative(&mut self, velocity: f64) {
        self.moving = true;
        self.velocity = -velocity;
    }

    fn stop(&mut self, _mode: StopMode) {
        self.moving = false;
        self.velocity = 0.0;
    }

    fn move_home(&mut self, _mode: HomeMode) {
        self.position = 0.0;
    }

    fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }

    fn set_acceleration(&mut self, _accel: f64) {}

    fn set_deceleration(&mut self, _decel: f64) {}

    fn set_position(&mut self, position: f64) {
        self.position = position;
    }

    fn enable_soft_limit(&mut self, min: f64, max: f64) {
        self.soft_limit_min = Some(min);
        self.soft_limit_max = Some(max);
    }

    fn disable_soft_limit(&mut self) {
        self.soft_limit_min = None;
        self.soft_limit_max = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use motion_core::config::CardConfig;
    use motion_core::traits::MotionCard;

    fn make_card() -> MockCard {
        let mut card = MockCard::new();
        card.initialize(&CardConfig::default().with_axis_count(4)).unwrap();
        card
    }

    #[test]
    fn test_initialize_and_close() {
        let mut card = MockCard::new();
        assert!(!card.status().is_initialized);

        card.initialize(&CardConfig::default().with_axis_count(2)).unwrap();
        assert!(card.status().is_initialized);
        assert_eq!(card.axis_count(), 2);

        card.close();
        assert!(!card.status().is_initialized);
    }

    #[test]
    fn test_axis_enable_disable() {
        let mut card = make_card();
        let axis = card.get_axis(0).unwrap();
        assert!(!axis.is_enabled());

        card.get_axis(0).unwrap().enable();
        assert!(card.get_axis(0).unwrap().is_enabled());

        card.get_axis(0).unwrap().disable();
        assert!(!card.get_axis(0).unwrap().is_enabled());
    }

    #[test]
    fn test_absolute_and_relative_move() {
        let mut card = make_card();
        card.get_axis(0).unwrap().enable();

        card.move_absolute(0, 100.0, 50.0).unwrap();
        assert_eq!(card.get_axis(0).unwrap().current_position(), 100.0);

        card.move_relative(0, 50.0, 30.0).unwrap();
        assert_eq!(card.get_axis(0).unwrap().current_position(), 150.0);
    }

    #[test]
    fn test_axis_out_of_range() {
        let mut card = make_card();
        // axis_count is 4, so index 4 should be out of range
        assert!(card.get_axis(4).is_err());
    }

    #[test]
    fn test_not_initialized_error() {
        let mut card = MockCard::new();
        let result = card.move_absolute(0, 100.0, 50.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_io_operations() {
        let mut card = make_card();

        card.write_output(0, true);
        assert!(card.read_input(0) == false); // input is separate from output

        // Test bitmask operations
        card.write_all_outputs(0b1010);
        let inputs = card.read_all_inputs();
        assert_eq!(inputs, 0);

        card.set_input(1, true);
        card.set_input(3, true);
        let bitmask = card.read_all_inputs();
        assert_eq!(bitmask, 0b1010);
    }

    #[test]
    fn test_home() {
        let mut card = make_card();
        card.move_absolute(0, 200.0, 50.0).unwrap();
        assert_eq!(card.get_axis(0).unwrap().current_position(), 200.0);

        card.move_home(0, HomeMode::Auto).unwrap();
        assert_eq!(card.get_axis(0).unwrap().current_position(), 0.0);
    }

    #[test]
    fn test_jog_and_stop() {
        let mut card = make_card();
        let axis = card.get_axis(0).unwrap();
        axis.jog_positive(100.0);
        assert!(axis.is_moving());

        card.stop_axis(0, StopMode::Immediate);
        assert!(!card.get_axis(0).unwrap().is_moving());
        assert_eq!(card.get_axis(0).unwrap().current_velocity(), 0.0);
    }

    #[test]
    fn test_soft_limit() {
        let mut card = make_card();
        let axis = card.get_axis(0).unwrap();
        axis.enable_soft_limit(-100.0, 100.0);
        // Mock doesn't enforce limits, but it stores them
        // Verify no panic on enable/disable cycle
        card.get_axis(0).unwrap().disable_soft_limit();
    }

    #[test]
    fn test_card_status_uptime() {
        let card = make_card();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let status = card.status();
        assert!(status.uptime.as_millis() >= 10);
    }
}
