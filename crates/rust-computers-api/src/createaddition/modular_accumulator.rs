//! Create Additions ModularAccumulator。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ModularAccumulator ペリフェラル。
pub struct ModularAccumulator {
    addr: PeriphAddr,
}

impl Peripheral for ModularAccumulator {
    const NAME: &'static str = "modular_accumulator";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl ModularAccumulator {
    /// 現在の蓄積エネルギー (FE) を返す。
    pub fn book_next_get_energy(&mut self) {
        peripheral::book_request(self.addr, "getEnergy", &msgpack::array(&[]));
    }
    pub fn read_last_get_energy(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getEnergy")?;
        peripheral::decode(&data)
    }

    /// 最大蓄積容量 (FE) を返す。
    pub fn book_next_get_capacity(&mut self) {
        peripheral::book_request(self.addr, "getCapacity", &msgpack::array(&[]));
    }
    pub fn read_last_get_capacity(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getCapacity")?;
        peripheral::decode(&data)
    }

    /// 充電率 (0.0〜100.0 %) を返す。
    pub fn book_next_get_percent(&mut self) {
        peripheral::book_request(self.addr, "getPercent", &msgpack::array(&[]));
    }
    pub fn read_last_get_percent(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPercent")?;
        peripheral::decode(&data)
    }

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

    /// マルチブロックの高さ（ブロック数）を返す。
    pub fn book_next_get_height(&mut self) {
        peripheral::book_request(self.addr, "getHeight", &msgpack::array(&[]));
    }
    pub fn read_last_get_height(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getHeight")?;
        peripheral::decode(&data)
    }

    /// マルチブロックの幅（ブロック数）を返す。
    pub fn book_next_get_width(&mut self) {
        peripheral::book_request(self.addr, "getWidth", &msgpack::array(&[]));
    }
    pub fn read_last_get_width(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getWidth")?;
        peripheral::decode(&data)
    }
}
