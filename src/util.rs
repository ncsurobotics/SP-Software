use std::f32::consts::PI;

use anyhow::bail;
use derive_getters::Getters;

/// See <https://cdn-shop.adafruit.com/datasheets/BST_BNO055_DS000_12.pdf>,
/// page 25
//[derive(Debug)]
pub enum BNO055AxisConfig {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
}

impl TryFrom<u8> for BNO055AxisConfig {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::P0),
            1 => Ok(Self::P1),
            2 => Ok(Self::P2),
            3 => Ok(Self::P3),
            4 => Ok(Self::P4),
            5 => Ok(Self::P5),
            6 => Ok(Self::P6),
            7 => Ok(Self::P7),
            x => bail!("{x} is >= 8, invalid axis config value"),
        }
    }
}

impl From<BNO055AxisConfig> for u8 {
    fn from(val: BNO055AxisConfig) -> Self {
        match val {
            BNO055AxisConfig::P0 => 0,
            BNO055AxisConfig::P1 => 1,
            BNO055AxisConfig::P2 => 2,
            BNO055AxisConfig::P3 => 3,
            BNO055AxisConfig::P4 => 4,
            BNO055AxisConfig::P5 => 5,
            BNO055AxisConfig::P6 => 6,
            BNO055AxisConfig::P7 => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct Angles {
    quat_w: f32,
    quat_x: f32,
    quat_y: f32,
    quat_z: f32,
    pitch: f32,
    roll: f32,
    yaw: f32,
}

impl Angles {
    pub fn from_raw(raw: [u8; 4 * 7]) -> Self {
        let quat_w = f32::from_le_bytes(raw[0..4].try_into().unwrap());
        let quat_x = f32::from_le_bytes(raw[4..8].try_into().unwrap());
        let quat_y = f32::from_le_bytes(raw[8..12].try_into().unwrap());
        let quat_z = f32::from_le_bytes(raw[12..16].try_into().unwrap());

        let pitch = 180.0 * (2.0 * (quat_y * quat_z + quat_w * quat_x)).asin() / PI;

        let gimbal_lock = (90.0 - pitch.abs()).abs() < 0.1;

        let yaw = if gimbal_lock {
            // Pitch is +/- 90 degrees
            // This is gimbal lock scenario
            // Roll and yaw mean the same thing
            // roll + yaw = 2 * atan2(q.y, q.w)
            // Can split among roll and yaw any way (not unique)
            2.0 * 180.0 * quat_y.atan2(quat_w) / PI
        } else {
            let yaw_numer = -2.0 * (quat_x * quat_y - quat_w * quat_z);
            let yaw_denom = 1.0 - 2.0 * (quat_x * quat_x + quat_z * quat_z);
            180.0 * yaw_numer.atan2(yaw_denom) / PI
        };

        let roll = if gimbal_lock {
            // Pitch is +/- 90 degrees
            // This is gimbal lock scenario
            // Roll and yaw mean the same thing
            // roll + yaw = 2 * atan2(q.y, q.w)
            // Can split among roll and yaw any way (not unique)
            0.0
        } else {
            let roll_numer = 2.0 * (quat_w * quat_y - quat_x * quat_z);
            let roll_denom = 1.0 - 2.0 * (quat_x * quat_x + quat_y * quat_y);
            180.0 * roll_numer.atan2(roll_denom) / PI
        };

        Self {
            quat_w,
            quat_x,
            quat_y,
            quat_z,
            pitch,
            roll,
            yaw,
        }
    }

    pub struct PIDController {
        kp: f32,
        ki: f32,
        kd: f32,
        setpoint: f32,
        process: f32,
        prev_error: f32,
        integral: f32,
        dt: f32,
        output_min: Option<f32>,
        output_max: Option<f32>,
        input_min: Option<f32>,
        input_max: Option<f32>,
    }

    impl PIDController {

        pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
            Self { kp, ki, kd }
        }

        pub fn set_setpoint(&mut self, set_point: f32) {
            self.setpoint = set_point;
        }

        pub fn calculate(&mut self, process: f32) -> f32 {
            self.process = process;
            let error = self.setpoint - self.process;
            self.integral += error * self.dt;
            let derivative = (error - self.prev_error) / self.dt;
            self.prev_error = error;
            let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
            if let (Some(min), Some(max)) = (self.output_min, self.output_max) {
                output.clamp(min, max)
            } else {
                output
            }
        }

        pub fn calculate(&mut self, setpoint: f32, process: f32) -> f32 {
            self.set_setpoint(setpoint);
            self.calculate(process)
        }

        pub fn limit_output(mut self, min: f32, max: f32) -> Self {
            self.output_min = Some(min);
            self.output_max = Some(max);
            self
        }

        pub fn enable_continuous_input(mut self, min_input: f32, max_input: f32) -> Self {
            // TODO: Implement continuous input handling.
            // For angles, wrap around at 360 degrees
            self.input_min = Some(min_input);
            self.input_max = Some(max_input);
            self
        }
    }
}
