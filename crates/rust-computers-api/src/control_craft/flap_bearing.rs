//! Control-Craft FlapBearing ペリフェラル。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// フラップベアリング（翼制御） ペリフェラル。
pub struct FlapBearing {
    addr: PeriphAddr,
}

impl Peripheral for FlapBearing {
    const NAME: &'static str = "controlcraft:flap_bearing_peripheral";

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

impl FlapBearing {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(book_next_get_angle, read_last_get_angle, get_angle_imm, "getAngle", f64);

    // ====== 状態変更系 ======

    /// 翼角度 (deg) を設定する。
    pub fn book_next_set_angle(&mut self, angle: f64) {
        let args = msgpack::array(&[msgpack::float64(angle)]);
        peripheral::book_action(self.addr, "setAngle", &args);
    }
    pub fn read_last_set_angle(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setAngle")?;
        Ok(())
    }

    /// 次のティックにコントラプションを組み立てる。
    pub fn book_next_assemble_next_tick(&mut self) {
        peripheral::book_action(self.addr, "assembleNextTick", &msgpack::array(&[]));
    }
    pub fn read_last_assemble_next_tick(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "assembleNextTick")?;
        Ok(())
    }

    /// 次のティックにコントラプションを分解する。
    pub fn book_next_disassemble_next_tick(&mut self) {
        peripheral::book_action(self.addr, "disassembleNextTick", &msgpack::array(&[]));
    }
    pub fn read_last_disassemble_next_tick(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "disassembleNextTick")?;
        Ok(())
    }
}
