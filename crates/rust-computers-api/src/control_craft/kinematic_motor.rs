//! Control-Craft KinematicMotor ペリフェラル。

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// キネマティックモーター ペリフェラル。
pub struct KinematicMotor {
    addr: PeriphAddr,
}

impl Peripheral for KinematicMotor {
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

impl KinematicMotor {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(
        book_next_get_target_angle,
        read_last_get_target_angle,
        get_target_angle_imm,
        "getTargetAngle",
        f64
    );
    book_read_imm!(
        book_next_get_control_target,
        read_last_get_control_target,
        get_control_target_imm,
        "getControlTarget",
        String
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
        book_next_get_relative,
        read_last_get_relative,
        get_relative_imm,
        "getRelative",
        [[f64; 3]; 3]
    );

    // ====== 状態変更系 ======

    /// 目標角度 (deg) を設定する。
    pub fn book_next_set_target_angle(&mut self, value: f64) {
        let args = msgpack::array(&[msgpack::float64(value)]);
        peripheral::book_action(self.addr, "setTargetAngle", &args);
    }
    pub fn read_last_set_target_angle(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTargetAngle")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 制御ターゲットを設定する。
    pub fn book_next_set_control_target(&mut self, target: &str) {
        let args = msgpack::array(&[msgpack::str(target)]);
        peripheral::book_action(self.addr, "setControlTarget", &args);
    }
    pub fn read_last_set_control_target(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setControlTarget")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 強制角度モードのオン/オフを切り替える。
    pub fn book_next_set_is_forcing_angle(&mut self, enabled: bool) {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::book_action(self.addr, "setIsForcingAngle", &args);
    }
    pub fn read_last_set_is_forcing_angle(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setIsForcingAngle")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}

impl KinematicMotor {
    pub async fn async_get_target_angle(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_target_angle();
        crate::wait_for_next_tick().await;
        self.read_last_get_target_angle()
    }

    pub async fn async_get_control_target(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_control_target();
        crate::wait_for_next_tick().await;
        self.read_last_get_control_target()
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

    pub async fn async_get_relative(&mut self) -> Result<[[f64; 3]; 3], PeripheralError> {
        self.book_next_get_relative();
        crate::wait_for_next_tick().await;
        self.read_last_get_relative()
    }

    pub async fn async_set_target_angle(&mut self, value: f64) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_target_angle(value);
        crate::wait_for_next_tick().await;
        self.read_last_set_target_angle()
    }

    pub async fn async_set_control_target(&mut self, target: &str) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_control_target(target);
        crate::wait_for_next_tick().await;
        self.read_last_set_control_target()
    }

    pub async fn async_set_is_forcing_angle(&mut self, enabled: bool) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_is_forcing_angle(enabled);
        crate::wait_for_next_tick().await;
        self.read_last_set_is_forcing_angle()
    }
}
