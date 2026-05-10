//! 雷赛 (Leadshine) LTSMC 系列板卡适配器

use motion_core::config::CardConfig;
use motion_core::error::{MotionError, Result};
use motion_core::models::*;
use motion_core::traits::{Axis as AxisTrait, MotionCard};
use std::collections::HashMap;

mod bindings;
use bindings::*;

/// 雷赛板卡
pub struct LeisaiCard {
    card_id: u16,
    initialized: bool,
    config: Option<CardConfig>,
    axes: HashMap<usize, LeisaiAxis>,
    error_count: u32,
    last_error: String,
    init_time: std::time::Instant,
}

impl LeisaiCard {
    pub fn new() -> Self {
        Self {
            card_id: 0,
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

impl MotionCard for LeisaiCard {
    fn initialize(&mut self, config: &CardConfig) -> Result<()> {
        let code = unsafe { nmc_board_init() };
        self.check_result(code, "nmc_board_init")?;

        self.card_id = config.board_id;
        self.config = Some(config.clone());
        self.initialized = true;
        self.init_time = std::time::Instant::now();

        for i in 0..config.axis_count {
            self.axes.insert(i, LeisaiAxis::new(self.card_id, i as u16));
        }

        Ok(())
    }

    fn close(&mut self) {
        unsafe {
            nmc_board_close();
        }
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
        "雷赛 Leadshine"
    }

    fn card_type(&self) -> &str {
        "LTSMC Series"
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
        unsafe { nmc_get_inbit(self.card_id, port) != 0 }
    }

    fn write_output(&mut self, port: u16, value: bool) {
        unsafe {
            nmc_set_outbit(self.card_id, port, value);
        }
    }

    fn read_all_inputs(&self) -> u16 {
        unsafe { nmc_get_inport(self.card_id, 0) }
    }

    fn write_all_outputs(&mut self, value: u16) {
        unsafe {
            nmc_set_outport(self.card_id, 0, value);
        }
    }

    fn move_absolute(&mut self, axis: usize, position: f64, velocity: f64) -> Result<()> {
        let code = unsafe {
            nmc_axis_move_absolute(self.card_id, axis as u16, position, velocity)
        };
        self.check_result(code, "nmc_axis_move_absolute")
    }

    fn move_relative(&mut self, axis: usize, distance: f64, velocity: f64) -> Result<()> {
        let code = unsafe {
            nmc_axis_move_relative(self.card_id, axis as u16, distance, velocity)
        };
        self.check_result(code, "nmc_axis_move_relative")
    }

    fn move_home(&mut self, axis: usize, _mode: HomeMode) -> Result<()> {
        let code = unsafe { nmc_axis_move_home(self.card_id, axis as u16, 0) };
        self.check_result(code, "nmc_axis_move_home")
    }

    fn stop_axis(&mut self, axis: usize, _mode: StopMode) {
        unsafe {
            nmc_axis_stop(self.card_id, axis as u16);
        }
    }

    fn stop_all(&mut self) {
        for i in 0..self.axis_count() {
            self.stop_axis(i, StopMode::Immediate);
        }
    }
}

/// 雷赛单轴适配器 (自包含, 无生命周期)
pub struct LeisaiAxis {
    card_id: u16,
    axis_index: u16,
}

impl LeisaiAxis {
    pub fn new(card_id: u16, axis_index: u16) -> Self {
        Self { card_id, axis_index }
    }

    fn is_limit_positive(&self) -> bool {
        unsafe { nmc_get_inp(self.card_id, self.axis_index) != 0 }
    }

    fn is_limit_negative(&self) -> bool {
        unsafe { nmc_get_inn(self.card_id, self.axis_index) != 0 }
    }
}

impl AxisTrait for LeisaiAxis {
    fn axis_index(&self) -> usize {
        self.axis_index as usize
    }

    fn is_enabled(&self) -> bool {
        unsafe { nmc_axis_get_enable_status(self.card_id, self.axis_index) != 0 }
    }

    fn is_moving(&self) -> bool {
        unsafe { nmc_axis_check_done(self.card_id, self.axis_index) == 0 }
    }

    fn is_alarm(&self) -> bool {
        unsafe { nmc_axis_get_alarm(self.card_id, self.axis_index) != 0 }
    }

    fn current_position(&self) -> f64 {
        unsafe { nmc_axis_get_actual_pos(self.card_id, self.axis_index) }
    }

    fn current_velocity(&self) -> f64 {
        unsafe { nmc_axis_get_actual_vel(self.card_id, self.axis_index) }
    }

    fn status(&self) -> AxisStatus {
        AxisStatus {
            is_ready: self.is_enabled() && !self.is_alarm(),
            is_homing: false,
            is_limit_positive: self.is_limit_positive(),
            is_limit_negative: self.is_limit_negative(),
            is_servo_on: self.is_enabled(),
            is_alarm: self.is_alarm(),
        }
    }

    fn enable(&mut self) {
        unsafe {
            nmc_set_axis_enable(self.card_id, self.axis_index, true);
        }
    }

    fn disable(&mut self) {
        unsafe {
            nmc_set_axis_enable(self.card_id, self.axis_index, false);
        }
    }

    fn move_absolute(&mut self, position: f64, velocity: f64) {
        unsafe {
            nmc_axis_move_absolute(self.card_id, self.axis_index, position, velocity);
        }
    }

    fn move_relative(&mut self, distance: f64, velocity: f64) {
        unsafe {
            nmc_axis_move_relative(self.card_id, self.axis_index, distance, velocity);
        }
    }

    fn jog_positive(&mut self, velocity: f64) {
        unsafe {
            nmc_axis_move_vel(self.card_id, self.axis_index, velocity);
        }
    }

    fn jog_negative(&mut self, velocity: f64) {
        unsafe {
            nmc_axis_move_vel(self.card_id, self.axis_index, -velocity);
        }
    }

    fn stop(&mut self, _mode: StopMode) {
        unsafe {
            nmc_axis_stop(self.card_id, self.axis_index);
        }
    }

    fn move_home(&mut self, _mode: HomeMode) {
        unsafe {
            nmc_axis_move_home(self.card_id, self.axis_index, 0);
        }
    }

    fn set_velocity(&mut self, velocity: f64) {
        unsafe {
            nmc_axis_set_vel(self.card_id, self.axis_index, velocity);
        }
    }

    fn set_acceleration(&mut self, accel: f64) {
        unsafe {
            nmc_axis_set_acc(self.card_id, self.axis_index, accel);
        }
    }

    fn set_deceleration(&mut self, decel: f64) {
        unsafe {
            nmc_axis_set_dec(self.card_id, self.axis_index, decel);
        }
    }

    fn set_position(&mut self, position: f64) {
        unsafe {
            nmc_axis_set_position(self.card_id, self.axis_index, position);
        }
    }

    fn enable_soft_limit(&mut self, min: f64, max: f64) {
        unsafe {
            nmc_axis_set_soft_limit(self.card_id, self.axis_index, min, max, true);
        }
    }

    fn disable_soft_limit(&mut self) {
        unsafe {
            nmc_axis_set_soft_limit(self.card_id, self.axis_index, 0.0, 0.0, false);
        }
    }
}
