//! Clockwork CC Compat GasEngine。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// GasEngine ペリフェラル。
pub struct GasEngine {
    addr: PeriphAddr,
}

impl Peripheral for GasEngine {
    const NAME: &'static str = "cw_gas_engine";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl GasEngine {
    /// 接続エンジン数を取得する (imm 対応)。
    pub fn book_next_get_attached_engines(&mut self) {
        peripheral::book_request(
            self.addr,
            "getAttachedEngines",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_attached_engines(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAttachedEngines")?;
        peripheral::decode(&data)
    }

    pub fn get_attached_engines_imm(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getAttachedEngines",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 全体効率を取得する (imm 対応)。
    pub fn book_next_get_total_efficiency(&mut self) {
        peripheral::book_request(
            self.addr,
            "getTotalEfficiency",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_total_efficiency(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTotalEfficiency")?;
        peripheral::decode(&data)
    }

    pub fn get_total_efficiency_imm(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getTotalEfficiency",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
