//! Create Additions PortableEnergyInterface。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// PortableEnergyInterface ペリフェラル。
pub struct PortableEnergyInterface {
    addr: PeriphAddr,
}

impl Peripheral for PortableEnergyInterface {
    const NAME: &'static str = "createaddition:portable_energy_interface";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl PortableEnergyInterface {
    /// バッファ内エネルギー (FE) を返す。
    pub fn book_next_get_energy(&mut self) {
        peripheral::book_request(self.addr, "getEnergy", &msgpack::array(&[]));
    }
    pub fn read_last_get_energy(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getEnergy")?;
        peripheral::decode(&data)
    }

    /// バッファの最大容量 (FE) を返す。
    pub fn book_next_get_capacity(&mut self) {
        peripheral::book_request(self.addr, "getCapacity", &msgpack::array(&[]));
    }
    pub fn read_last_get_capacity(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getCapacity")?;
        peripheral::decode(&data)
    }

    /// コントラプションに接続されているかを返す。
    pub fn book_next_is_connected(&mut self) {
        peripheral::book_request(self.addr, "isConnected", &msgpack::array(&[]));
    }
    pub fn read_last_is_connected(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isConnected")?;
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
}
