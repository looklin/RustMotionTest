//! 研华 (Advantech) PCI-124x 系列板卡适配器

use motion_core::config::CardConfig;
use motion_core::error::{MotionError, Result};
use motion_core::models::*;
use motion_core::traits::{Axis as AxisTrait, MotionCard};
use std::collections::HashMap;

mod bindings;
use bindings::*;

/// 研华板卡
pub struct AdvantechCard {
    device: u16,
    initialized: bool,
    config: Option<CardConfig>,
    axes: HashMap<usize, AdvantechAxis>,
    error_count: u32,
    last_error: String,
    init_time: std::time::Instant,
}

impl AdvantechCard {
    pub fn new() -> Self {
        Self {
            device: 0,
            initialized: false,
            config: None,
            axes: HashMap::new(),
            error_count: 0,
            last_error: String::new(),
            init_time: std::time::Instant::now(),
        }
    }

    fn check_result(&mut self, code: i32, context: &str) -> Result<()> {
        if code == 0 {
            Ok(())
        } else {
            self.error_count += 1;
            self.last_error = format!("{} (code: {})", context, code);
            Err(MotionError::SdkError {
                code: code as i16,
                message: self.last_error.clone(),
            })
        }
    }
}

impl MotionCard for AdvantechCard {
    fn initialize(&mut self, config: &CardConfig) -> Result<()> {
        let mut device: u16 = 0;
        let code = unsafe { APS_initial(&mut device) };
        self.check_result(code, "APS_initial")?;

        self.device = device;
        self.config = Some(config.clone());
        self.initialized = true;
        self.init_time = std::time::Instant::now();

        for i in 0..config.axis_count {
            self.axes.insert(
                i,
                AdvantechAxis::new(self.device, i as u16),
            );
        }

        Ok(())
    }

    fn close(&mut self) {
        unsafe {
            APS_close(self.device);
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
        "研华 Advantech"
    }

    fn card_type(&self) -> &str {
        "PCI-124x Series"
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
        unsafe {
            let mut state: u32 = 0;
            APS_get_DI(self.device, &mut state);
            (state & (1 << port)) != 0
        }
    }

    fn write_output(&mut self, port: u16, value: bool) {
        unsafe {
            APS_set_DO(self.device, port, if value { 1 } else { 0 });
        }
    }

    fn read_all_inputs(&self) -> u16 {
        unsafe {
            let mut state: u32 = 0;
            APS_get_DI(self.device, &mut state);
            state as u16
        }
    }

    fn write_all_outputs(&mut self, value: u16) {
        unsafe {
            for i in 0..16 {
                APS_set_DO(self.device, i, ((value >> i) & 1) as u16);
            }
        }
    }

    fn move_absolute(&mut self, axis: usize, position: f64, velocity: f64) -> Result<()> {
        let code = unsafe {
            APS_move(
                self.device,
                axis as u16,
                0, // absolute mode
                position as i32,
                velocity as i32,
            )
        };
        self.check_result(code, "APS_move (absolute)")
    }

    fn move_relative(&mut self, axis: usize, distance: f64, velocity: f64) -> Result<()> {
        let code = unsafe {
            APS_move(
                self.device,
                axis as u16,
                1, // relative mode
                distance as i32,
                velocity as i32,
            )
        };
        self.check_result(code, "APS_move (relative)")
    }

    fn move_home(&mut self, axis: usize, _mode: HomeMode) -> Result<()> {
        let code = unsafe { APS_home(self.device, axis as u16, 0) };
        self.check_result(code, "APS_home")
    }

    fn stop_axis(&mut self, axis: usize, _mode: StopMode) {
        unsafe {
            APS_immediate_stop(self.device, axis as u16);
        }
    }

    fn stop_all(&mut self) {
        for i in 0..self.axis_count() {
            self.stop_axis(i, StopMode::Immediate);
        }
    }
}

/// 研华单轴适配器
pub struct AdvantechAxis {
    device: u16,
    axis_index: u16,
}

impl AdvantechAxis {
    pub fn new(device: u16, axis_index: u16) -> Self {
        Self {
            device,
            axis_index,
        }
    }
}

impl AxisTrait for AdvantechAxis {
    fn axis_index(&self) -> usize {
        self.axis_index as usize
    }

    fn is_enabled(&self) -> bool {
        unsafe {
            let mut state: u16 = 0;
            APS_get_io_svon(self.device, self.axis_index, &mut state);
            state != 0
        }
    }

    fn is_moving(&self) -> bool {
        unsafe {
            let mut status: u32 = 0;
            APS_get_status(self.device, self.axis_index, &mut status);
            (status & 0x01) != 0
        }
    }

    fn is_alarm(&self) -> bool {
        unsafe {
            let mut status: u32 = 0;
            APS_get_status(self.device, self.axis_index, &mut status);
            (status & 0x02) != 0
        }
    }

    fn current_position(&self) -> f64 {
        unsafe {
            let mut pos: i32 = 0;
            APS_get_command_pos(self.device, self.axis_index, &mut pos);
            pos as f64
        }
    }

    fn current_velocity(&self) -> f64 {
        unsafe {
            let mut vel: i32 = 0;
            APS_get_command_vel(self.device, self.axis_index, &mut vel);
            vel as f64
        }
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
            APS_set_io_svon(self.device, self.axis_index, 1);
        }
    }

    fn disable(&mut self) {
        unsafe {
            APS_set_io_svon(self.device, self.axis_index, 0);
        }
    }

    fn move_absolute(&mut self, position: f64, velocity: f64) {
        unsafe {
            APS_move(
                self.device,
                self.axis_index,
                0,
                position as i32,
                velocity as i32,
            );
        }
    }

    fn move_relative(&mut self, distance: f64, velocity: f64) {
        unsafe {
            APS_move(
                self.device,
                self.axis_index,
                1,
                distance as i32,
                velocity as i32,
            );
        }
    }

    fn jog_positive(&mut self, velocity: f64) {
        unsafe {
            APS_conti_t_move_abs(self.device, self.axis_index, i32::MAX, velocity as i32);
        }
    }

    fn jog_negative(&mut self, velocity: f64) {
        unsafe {
            APS_conti_t_move_abs(self.device, self.axis_index, i32::MIN, velocity as i32);
        }
    }

    fn stop(&mut self, _mode: StopMode) {
        unsafe {
            APS_immediate_stop(self.device, self.axis_index);
        }
    }

    fn move_home(&mut self, _mode: HomeMode) {
        unsafe {
            APS_home(self.device, self.axis_index, 0);
        }
    }

    fn set_velocity(&mut self, velocity: f64) {
        unsafe {
            APS_set_vel(self.device, self.axis_index, velocity as i32);
        }
    }

    fn set_acceleration(&mut self, accel: f64) {
        unsafe {
            APS_set_acc(self.device, self.axis_index, accel as i32);
        }
    }

    fn set_deceleration(&mut self, decel: f64) {
        unsafe {
            APS_set_dec(self.device, self.axis_index, decel as i32);
        }
    }

    fn set_position(&mut self, position: f64) {
        unsafe {
            APS_set_command_pos(self.device, self.axis_index, position as i32);
        }
    }

    fn enable_soft_limit(&mut self, _min: f64, _max: f64) {}

    fn disable_soft_limit(&mut self) {}
}

impl AdvantechAxis {
    fn is_limit_positive(&self) -> bool {
        unsafe {
            let mut status: u32 = 0;
            APS_get_status(self.device, self.axis_index, &mut status);
            (status & 0x04) != 0
        }
    }

    fn is_limit_negative(&self) -> bool {
        unsafe {
            let mut status: u32 = 0;
            APS_get_status(self.device, self.axis_index, &mut status);
            (status & 0x08) != 0
        }
    }
}
