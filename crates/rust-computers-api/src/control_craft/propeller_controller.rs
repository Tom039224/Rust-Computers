//! Control-Craft PropellerController ペリフェラル。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// プロペラコントローラー ペリフェラル。
pub struct PropellerController {
    addr: PeriphAddr,
}

impl Peripheral for PropellerController {
    const NAME: &'static str = "PropellerController";

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

impl PropellerController {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(
        book_next_get_target_speed,
        read_last_get_target_speed,
        get_target_speed_imm,
        "getTargetSpeed",
        f64
    );

    // ====== 状態変更系 ======

    /// プロペラの目標速度 (RPM) を設定する。
    pub fn book_next_set_target_speed(&mut self, speed: f64) {
        let args = msgpack::array(&[msgpack::float64(speed)]);
        peripheral::book_action(self.addr, "setTargetSpeed", &args);
    }
    pub fn read_last_set_target_speed(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTargetSpeed")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
