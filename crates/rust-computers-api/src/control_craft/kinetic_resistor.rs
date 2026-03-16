//! Control-Craft KineticResistor ペリフェラル。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// キネティックレジスター ペリフェラル。
pub struct KineticResistor {
    addr: PeriphAddr,
}

impl Peripheral for KineticResistor {
    const NAME: &'static str = "resistor";

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

impl KineticResistor {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(book_next_get_ratio, read_last_get_ratio, get_ratio_imm, "getRatio", f64);

    // ====== 状態変更系 ======

    /// 抵抗比率を設定する (mainThread)。
    pub fn book_next_set_ratio(&mut self, ratio: f64) {
        let args = msgpack::array(&[msgpack::float64(ratio)]);
        peripheral::book_action(self.addr, "setRatio", &args);
    }
    pub fn read_last_set_ratio(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setRatio")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
