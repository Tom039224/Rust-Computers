//! Control-Craft CannonMount ペリフェラル。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// キャノンマウント ペリフェラル。
pub struct CannonMount {
    addr: PeriphAddr,
}

impl Peripheral for CannonMount {
    const NAME: &'static str = "controlcraft$cannon_mount";

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

impl CannonMount {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(book_next_get_pitch, read_last_get_pitch, get_pitch_imm, "getPitch", f64);
    book_read_imm!(book_next_get_yaw, read_last_get_yaw, get_yaw_imm, "getYaw", f64);

    // ====== 状態変更系 ======

    /// ピッチ角度 (deg) を設定する。
    pub fn book_next_set_pitch(&mut self, pitch: f64) {
        let args = msgpack::array(&[msgpack::float64(pitch)]);
        peripheral::book_action(self.addr, "setPitch", &args);
    }
    pub fn read_last_set_pitch(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setPitch")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ヨー角度 (deg) を設定する。
    pub fn book_next_set_yaw(&mut self, yaw: f64) {
        let args = msgpack::array(&[msgpack::float64(yaw)]);
        peripheral::book_action(self.addr, "setYaw", &args);
    }
    pub fn read_last_set_yaw(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setYaw")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// コントラプションを組み立てる (mainThread)。
    pub fn book_next_assemble(&mut self) {
        peripheral::book_action(self.addr, "assemble", &msgpack::array(&[]));
    }
    pub fn read_last_assemble(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "assemble")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// コントラプションを分解する (mainThread)。
    pub fn book_next_disassemble(&mut self) {
        peripheral::book_action(self.addr, "disassemble", &msgpack::array(&[]));
    }
    pub fn read_last_disassemble(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "disassemble")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
