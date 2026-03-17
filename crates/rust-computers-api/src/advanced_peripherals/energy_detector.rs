//! AdvancedPeripherals EnergyDetector。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// EnergyDetector ペリフェラル。
pub struct EnergyDetector {
    addr: PeriphAddr,
}

impl Peripheral for EnergyDetector {
    const NAME: &'static str = "energy_detector";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl EnergyDetector {
    /// 現在の転送量 (FE/t) を取得する。
    pub fn book_next_get_transfer_rate(&mut self) {
        peripheral::book_request(self.addr, "getTransferRate", &msgpack::array(&[]));
    }
    pub fn read_last_get_transfer_rate(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTransferRate")?;
        peripheral::decode(&data)
    }

    /// 転送レート制限 (FE/t) を取得する。
    pub fn book_next_get_transfer_rate_limit(&mut self) {
        peripheral::book_request(self.addr, "getTransferRateLimit", &msgpack::array(&[]));
    }
    pub fn read_last_get_transfer_rate_limit(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTransferRateLimit")?;
        peripheral::decode(&data)
    }

    /// 転送レート上限 (FE/t) を設定する。
    pub fn book_next_set_transfer_rate_limit(&mut self, rate: f64) {
        let args = msgpack::array(&[msgpack::float64(rate)]);
        peripheral::book_action(self.addr, "setTransferRateLimit", &args);
    }
    pub fn read_last_set_transfer_rate_limit(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTransferRateLimit")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}

impl EnergyDetector {
    // ─── async_* バリアント ──────────────────────────────────

    pub async fn async_get_transfer_rate(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_transfer_rate();
        crate::wait_for_next_tick().await;
        self.read_last_get_transfer_rate()
    }

    pub async fn async_get_transfer_rate_limit(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_transfer_rate_limit();
        crate::wait_for_next_tick().await;
        self.read_last_get_transfer_rate_limit()
    }

    pub async fn async_set_transfer_rate_limit(&mut self, rate: f64) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_transfer_rate_limit(rate);
        crate::wait_for_next_tick().await;
        self.read_last_set_transfer_rate_limit()
    }
}
