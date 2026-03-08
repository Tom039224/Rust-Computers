//! Control-Craft DynamicMotor ペリフェラル。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ダイナミックモーター ペリフェラル。
pub struct DynamicMotor {
    addr: PeriphAddr,
}

impl Peripheral for DynamicMotor {
    const NAME: &'static str = "controlcraft:dynamic_motor_peripheral";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

macro_rules! book_read_imm {
    ($book:ident, $read:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub fn $book(&mut self) {
            peripheral::book_request(self.addr, $method, &crate::msgpack::array(&[]));
        }
        pub fn $read(&self) -> Result<$ret, PeripheralError> {
            let data = peripheral::read_result(self.addr, $method)?;
            peripheral::decode(&data)
        }
        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.addr, $method, &crate::msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl DynamicMotor {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(
        book_next_get_target_value,
        read_last_get_target_value,
        get_target_value_imm,
        "getTargetValue",
        f64
    );
    book_read_imm!(
        book_next_get_physics,
        read_last_get_physics,
        get_physics_imm,
        "getPhysics",
        crate::msgpack::Value
    );
    book_read_imm!(book_next_get_angle, read_last_get_angle, get_angle_imm, "getAngle", f64);
    book_read_imm!(
        book_next_get_angular_velocity,
        read_last_get_angular_velocity,
        get_angular_velocity_imm,
        "getAngularVelocity",
        f64
    );
    book_read_imm!(
        book_next_get_current_value,
        read_last_get_current_value,
        get_current_value_imm,
        "getCurrentValue",
        f64
    );
    book_read_imm!(
        book_next_get_relative,
        read_last_get_relative,
        get_relative_imm,
        "getRelative",
        [[f64; 3]; 3]
    );
    book_read_imm!(book_next_is_locked, read_last_is_locked, is_locked_imm, "isLocked", bool);

    // ====== 状態変更系 ======

    /// PID ゲインを設定する。
    pub fn book_next_set_pid(&mut self, p: f64, i: f64, d: f64) {
        let args = msgpack::array(&[
            msgpack::float64(p),
            msgpack::float64(i),
            msgpack::float64(d),
        ]);
        peripheral::book_action(self.addr, "setPID", &args);
    }
    pub fn read_last_set_pid(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setPID")?;
        Ok(())
    }

    /// 目標値（角度 deg）を設定する。
    pub fn book_next_set_target_value(&mut self, value: f64) {
        let args = msgpack::array(&[msgpack::float64(value)]);
        peripheral::book_action(self.addr, "setTargetValue", &args);
    }
    pub fn read_last_set_target_value(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setTargetValue")?;
        Ok(())
    }

    /// 出力トルクスケールを設定する。
    pub fn book_next_set_output_torque(&mut self, scale: f64) {
        let args = msgpack::array(&[msgpack::float64(scale)]);
        peripheral::book_action(self.addr, "setOutputTorque", &args);
    }
    pub fn read_last_set_output_torque(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setOutputTorque")?;
        Ok(())
    }

    /// 角度調整モードのオン/オフを切り替える。
    pub fn book_next_set_is_adjusting_angle(&mut self, enabled: bool) {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::book_action(self.addr, "setIsAdjustingAngle", &args);
    }
    pub fn read_last_set_is_adjusting_angle(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setIsAdjustingAngle")?;
        Ok(())
    }

    /// モーターをロック（固定）する。
    pub fn book_next_lock(&mut self) {
        peripheral::book_action(self.addr, "lock", &msgpack::array(&[]));
    }
    pub fn read_last_lock(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "lock")?;
        Ok(())
    }

    /// モーターのロックを解除する。
    pub fn book_next_unlock(&mut self) {
        peripheral::book_action(self.addr, "unlock", &msgpack::array(&[]));
    }
    pub fn read_last_unlock(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "unlock")?;
        Ok(())
    }
}
