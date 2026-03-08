//! Control-Craft CompactFlap ペリフェラル。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// コンパクトフラップ ペリフェラル。
pub struct CompactFlap {
    addr: PeriphAddr,
}

impl Peripheral for CompactFlap {
    const NAME: &'static str = "controlcraft:compact_flap_peripheral";

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

impl CompactFlap {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(book_next_get_angle, read_last_get_angle, get_angle_imm, "getAngle", f64);
    book_read_imm!(book_next_get_tilt, read_last_get_tilt, get_tilt_imm, "getTilt", f64);

    // ====== 状態変更系 ======

    /// フラップ角度 (deg) を設定する。
    pub fn book_next_set_angle(&mut self, angle: f64) {
        let args = msgpack::array(&[msgpack::float64(angle)]);
        peripheral::book_action(self.addr, "setAngle", &args);
    }
    pub fn read_last_set_angle(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setAngle")?;
        Ok(())
    }

    /// チルト角度 (deg) を設定する。
    pub fn book_next_set_tilt(&mut self, tilt: f64) {
        let args = msgpack::array(&[msgpack::float64(tilt)]);
        peripheral::book_action(self.addr, "setTilt", &args);
    }
    pub fn read_last_set_tilt(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setTilt")?;
        Ok(())
    }
}
