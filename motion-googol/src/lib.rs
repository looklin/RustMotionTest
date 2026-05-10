//! 固高 (Googol) GTS 系列板卡适配器

use motion_core::config::CardConfig;
use motion_core::error::{MotionError, Result};
use motion_core::models::*;
use motion_core::traits::{Axis as AxisTrait, MotionCard};
use std::collections::HashMap;

mod bindings;
use bindings::*;

/// 固高板卡
pub struct GoogolCard {
    initialized: bool,
    config: Option<CardConfig>,
    axes: HashMap<usize, GoogolAxis>,
    error_count: u32,
    last_error: String,
    init_time: std::time::Instant,
}

impl GoogolCard {
    pub fn new() -> Self {
        Self {
            initialized: false,
            config: None,
            axes: HashMap::new(),
            error_count: 0,
            last_error: String::new(),
            init_time: std::time::Instant::now(),
        }
    }

    fn check_result(&mut self, code: i16, context: &str) -> Result<()> {
        if code == 0 {
            Ok(())
        } else {
            self.error_count += 1;
            self.last_error = format!("{} (code: {})", context, code);
            Err(MotionError::SdkError {
                code,
                message: self.last_error.clone(),
            })
        }
    }
}

impl MotionCard for GoogolCard {
    fn initialize(&mut self, config: &CardConfig) -> Result<()> {
        let code = unsafe { GT_Init() };
        self.check_result(code, "GT_Init")?;

        self.config = Some(config.clone());
        self.initialized = true;
        self.init_time = std::time::Instant::now();

        for i in 0..config.axis_count {
            self.axes.insert(
                i,
                GoogolAxis::new(i as i16),
            );
        }

        Ok(())
    }

    fn close(&mut self) {
        self.initialized = false;
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
        "固高 Googol"
    }

    fn card_type(&self) -> &str {
        "GTS Series"
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
        self.axes
            .get_mut(&index)
            .map(|a| a as &mut dyn AxisTrait)
            .ok_or(MotionError::AxisOutOfRange { index, max })
    }

    fn read_input(&self, port: u16) -> bool {
        unsafe { (GT_GetDI(port as i16) & 1) != 0 }
    }

    fn write_output(&mut self, port: u16, value: bool) {
        unsafe {
            GT_SetDO(port as i16, if value { 1 } else { 0 });
        }
    }

    fn read_all_inputs(&self) -> u16 {
        unsafe { GT_GetDI(0) as u16 }
    }

    fn write_all_outputs(&mut self, value: u16) {
        unsafe {
            GT_SetDO(0, value as i16);
        }
    }

    fn move_absolute(&mut self, axis: usize, position: f64, velocity: f64) -> Result<()> {
        let code = unsafe {
            GT_PrfT(axis as i16);
            GT_SetPos(axis as i16, position as i32);
            GT_SetVel(axis as i16, velocity)
        };
        self.check_result(code, "GT_SetPos/SetVel")
    }

    fn move_relative(&mut self, axis: usize, distance: f64, velocity: f64) -> Result<()> {
        let code = unsafe {
            GT_PrfT(axis as i16);
            let cur = GT_GetPrfPos(axis as i16) as f64;
            GT_SetPos(axis as i16, (cur + distance) as i32);
            GT_SetVel(axis as i16, velocity)
        };
        self.check_result(code, "GT_SetPos/SetVel (relative)")
    }

    fn move_home(&mut self, axis: usize, _mode: HomeMode) -> Result<()> {
        let code = unsafe { GT_Home(axis as i16, 0) };
        self.check_result(code, "GT_Home")
    }

    fn stop_axis(&mut self, axis: usize, _mode: StopMode) {
        unsafe {
            GT_Stop(axis as i16);
        }
    }

    fn stop_all(&mut self) {
        for i in 0..self.axis_count() {
            self.stop_axis(i, StopMode::Immediate);
        }
    }
}

/// 固高单轴适配器
pub struct GoogolAxis {
    axis_index: i16,
}

impl GoogolAxis {
    pub fn new(axis_index: i16) -> Self {
        Self { axis_index }
    }
}

impl AxisTrait for GoogolAxis {
    fn axis_index(&self) -> usize {
        self.axis_index as usize
    }

    fn is_enabled(&self) -> bool {
        unsafe { GT_ClrStp(self.axis_index) == 0 }
    }

    fn is_moving(&self) -> bool {
        unsafe {
            let mut crash = 0i32;
            GT_GetAxisStatus(self.axis_index, &mut crash);
            crash != 0
        }
    }

    fn is_alarm(&self) -> bool {
        unsafe {
            let mut status = [0i32; 4];
            GT_GetAxisStatus(self.axis_index, &mut status[0] as *mut i32);
            (status[0] & 0x01) != 0
        }
    }

    fn current_position(&self) -> f64 {
        unsafe { GT_GetPrfPos(self.axis_index) as f64 }
    }

    fn current_velocity(&self) -> f64 {
        unsafe { GT_GetPrfVel(self.axis_index) }
    }

    fn status(&self) -> AxisStatus {
        AxisStatus {
            is_ready: self.is_enabled() && !self.is_alarm(),
            is_homing: false,
            is_limit_positive: false,
            is_limit_negative: false,
            is_servo_on: self.is_enabled(),
            is_alarm: self.is_alarm(),
        }
    }

    fn enable(&mut self) {
        unsafe {
            GT_AxisOn(self.axis_index);
        }
    }

    fn disable(&mut self) {
        unsafe {
            GT_AxisOff(self.axis_index);
        }
    }

    fn move_absolute(&mut self, position: f64, velocity: f64) {
        unsafe {
            GT_PrfT(self.axis_index);
            GT_SetPos(self.axis_index, position as i32);
            GT_SetVel(self.axis_index, velocity);
        }
    }

    fn move_relative(&mut self, distance: f64, velocity: f64) {
        unsafe {
            let cur = GT_GetPrfPos(self.axis_index) as f64;
            GT_PrfT(self.axis_index);
            GT_SetPos(self.axis_index, (cur + distance) as i32);
            GT_SetVel(self.axis_index, velocity);
        }
    }

    fn jog_positive(&mut self, velocity: f64) {
        unsafe {
            GT_PrfJog(self.axis_index);
            GT_SetVel(self.axis_index, velocity);
        }
    }

    fn jog_negative(&mut self, velocity: f64) {
        unsafe {
            GT_PrfJog(self.axis_index);
            GT_SetVel(self.axis_index, -velocity);
        }
    }

    fn stop(&mut self, _mode: StopMode) {
        unsafe {
            GT_Stop(self.axis_index);
        }
    }

    fn move_home(&mut self, _mode: HomeMode) {
        unsafe {
            GT_Home(self.axis_index, 0);
        }
    }

    fn set_velocity(&mut self, velocity: f64) {
        unsafe {
            GT_SetVel(self.axis_index, velocity);
        }
    }

    fn set_acceleration(&mut self, accel: f64) {
        unsafe {
            GT_SetAcc(self.axis_index, accel);
        }
    }

    fn set_deceleration(&mut self, decel: f64) {
        unsafe {
            GT_SetDec(self.axis_index, decel);
        }
    }

    fn set_position(&mut self, position: f64) {
        unsafe {
            GT_SetPos(self.axis_index, position as i32);
        }
    }

    fn enable_soft_limit(&mut self, _min: f64, _max: f64) {
        // 固高通过限位输入实现软限位
    }

    fn disable_soft_limit(&mut self) {}
}
