//! Control-Craft DynamicMotor ペリフェラル。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ダイナミックモーター ペリフェラル。
pub struct DynamicMotor {
    addr: PeriphAddr,
}

impl Peripheral for DynamicMotor {
    const NAME: &'static str = "servo";

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
    pub fn read_last_set_pid(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setPID")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 目標値（角度 deg）を設定する。
    pub fn book_next_set_target_value(&mut self, value: f64) {
        let args = msgpack::array(&[msgpack::float64(value)]);
        peripheral::book_action(self.addr, "setTargetValue", &args);
    }
    pub fn read_last_set_target_value(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTargetValue")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 出力トルクスケールを設定する。
    pub fn book_next_set_output_torque(&mut self, scale: f64) {
        let args = msgpack::array(&[msgpack::float64(scale)]);
        peripheral::book_action(self.addr, "setOutputTorque", &args);
    }
    pub fn read_last_set_output_torque(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setOutputTorque")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 角度調整モードのオン/オフを切り替える。
    pub fn book_next_set_is_adjusting_angle(&mut self, enabled: bool) {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::book_action(self.addr, "setIsAdjustingAngle", &args);
    }
    pub fn read_last_set_is_adjusting_angle(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setIsAdjustingAngle")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// モーターをロック（固定）する。
    pub fn book_next_lock(&mut self) {
        peripheral::book_action(self.addr, "lock", &msgpack::array(&[]));
    }
    pub fn read_last_lock(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "lock")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// モーターのロックを解除する。
    pub fn book_next_unlock(&mut self) {
        peripheral::book_action(self.addr, "unlock", &msgpack::array(&[]));
    }
    pub fn read_last_unlock(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "unlock")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}

impl DynamicMotor {
    pub async fn async_get_target_value(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_target_value();
        crate::wait_for_next_tick().await;
        self.read_last_get_target_value()
    }

    pub async fn async_get_physics(&mut self) -> Result<crate::msgpack::Value, PeripheralError> {
        self.book_next_get_physics();
        crate::wait_for_next_tick().await;
        self.read_last_get_physics()
    }

    pub async fn async_get_angle(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_angle();
        crate::wait_for_next_tick().await;
        self.read_last_get_angle()
    }

    pub async fn async_get_angular_velocity(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_angular_velocity();
        crate::wait_for_next_tick().await;
        self.read_last_get_angular_velocity()
    }

    pub async fn async_get_current_value(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_current_value();
        crate::wait_for_next_tick().await;
        self.read_last_get_current_value()
    }

    pub async fn async_get_relative(&mut self) -> Result<[[f64; 3]; 3], PeripheralError> {
        self.book_next_get_relative();
        crate::wait_for_next_tick().await;
        self.read_last_get_relative()
    }

    pub async fn async_is_locked(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_locked();
        crate::wait_for_next_tick().await;
        self.read_last_is_locked()
    }

    pub async fn async_set_pid(&mut self, p: f64, i: f64, d: f64) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_pid(p, i, d);
        crate::wait_for_next_tick().await;
        self.read_last_set_pid()
    }

    pub async fn async_set_target_value(&mut self, value: f64) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_target_value(value);
        crate::wait_for_next_tick().await;
        self.read_last_set_target_value()
    }

    pub async fn async_set_output_torque(&mut self, scale: f64) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_output_torque(scale);
        crate::wait_for_next_tick().await;
        self.read_last_set_output_torque()
    }

    pub async fn async_set_is_adjusting_angle(&mut self, enabled: bool) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_is_adjusting_angle(enabled);
        crate::wait_for_next_tick().await;
        self.read_last_set_is_adjusting_angle()
    }

    pub async fn async_lock(&mut self) -> Vec<Result<(), PeripheralError>> {
        self.book_next_lock();
        crate::wait_for_next_tick().await;
        self.read_last_lock()
    }

    pub async fn async_unlock(&mut self) -> Vec<Result<(), PeripheralError>> {
        self.book_next_unlock();
        crate::wait_for_next_tick().await;
        self.read_last_unlock()
    }
}
