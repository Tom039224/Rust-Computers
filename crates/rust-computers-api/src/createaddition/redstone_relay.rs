//! Create Additions RedstoneRelay。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// RedstoneRelay ペリフェラル。
pub struct RedstoneRelay {
    addr: PeriphAddr,
}

impl Peripheral for RedstoneRelay {
    const NAME: &'static str = "redstone_relay";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl RedstoneRelay {
    /// 1 tick あたりの最大入力 (FE/t) を返す。
    pub fn book_next_get_max_insert(&mut self) {
        peripheral::book_request(self.addr, "getMaxInsert", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_insert(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxInsert")?;
        peripheral::decode(&data)
    }

    /// 1 tick あたりの最大出力 (FE/t) を返す。
    pub fn book_next_get_max_extract(&mut self) {
        peripheral::book_request(self.addr, "getMaxExtract", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_extract(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxExtract")?;
        peripheral::decode(&data)
    }

    /// 現在の通過エネルギー量 (FE/t) を返す。
    pub fn book_next_get_throughput(&mut self) {
        peripheral::book_request(self.addr, "getThroughput", &msgpack::array(&[]));
    }
    pub fn read_last_get_throughput(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getThroughput")?;
        peripheral::decode(&data)
    }

    /// レッドストーン信号を受信しているかを返す。
    pub fn book_next_is_powered(&mut self) {
        peripheral::book_request(self.addr, "isPowered", &msgpack::array(&[]));
    }
    pub fn read_last_is_powered(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPowered")?;
        peripheral::decode(&data)
    }
}
