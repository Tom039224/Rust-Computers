//! Control-Craft Slider ペリフェラル。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// スライダー ペリフェラル。
pub struct Slider {
    addr: PeriphAddr,
}

impl Peripheral for Slider {
    const NAME: &'static str = "slider";

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

impl Slider {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(
        book_next_get_distance,
        read_last_get_distance,
        get_distance_imm,
        "getDistance",
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
    book_read_imm!(book_next_is_locked, read_last_is_locked, is_locked_imm, "isLocked", bool);

    // ====== 状態変更系 ======

    /// 出力力スケールを設定する。
    pub fn book_next_set_output_force(&mut self, scale: f64) {
        let args = msgpack::array(&[msgpack::float64(scale)]);
        peripheral::book_action(self.addr, "setOutputForce", &args);
    }
    pub fn read_last_set_output_force(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setOutputForce")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

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

    /// 目標値を設定する。
    pub fn book_next_set_target_value(&mut self, target: f64) {
        let args = msgpack::array(&[msgpack::float64(target)]);
        peripheral::book_action(self.addr, "setTargetValue", &args);
    }
    pub fn read_last_set_target_value(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTargetValue")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// スライダーをロック（固定）する。
    pub fn book_next_lock(&mut self) {
        peripheral::book_action(self.addr, "lock", &msgpack::array(&[]));
    }
    pub fn read_last_lock(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "lock")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// スライダーのロックを解除する。
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
