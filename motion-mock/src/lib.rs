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
