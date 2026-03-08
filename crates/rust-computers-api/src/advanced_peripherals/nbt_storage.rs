//! AdvancedPeripherals NBTStorage。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// NBTStorage ペリフェラル。
pub struct NbtStorage {
    addr: PeriphAddr,
}

impl Peripheral for NbtStorage {
    const NAME: &'static str = "advancedPeripherals:nbt_storage";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl NbtStorage {
    /// 保存されている NBT データを読み取る。
    pub fn book_next_read(&mut self) {
        peripheral::book_request(self.addr, "read", &msgpack::array(&[]));
    }
    pub fn read_last_read(&self) -> Result<msgpack::Value, PeripheralError> {
        let data = peripheral::read_result(self.addr, "read")?;
        peripheral::decode(&data)
    }

    /// SNBT 文字列を解析して保存する。
    pub fn book_next_write_json(&mut self, snbt: &str) {
        let args = msgpack::array(&[msgpack::str(snbt)]);
        peripheral::book_action(self.addr, "writeJson", &args);
    }
    pub fn read_last_write_json(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "writeJson")?;
        peripheral::decode(&data)
    }

    /// Lua テーブル（msgpack エンコード済み）を NBT に変換して保存する。
    pub fn book_next_write_table(&mut self, table_data: &[u8]) {
        let args = msgpack::array(&[table_data.to_vec()]);
        peripheral::book_action(self.addr, "writeTable", &args);
    }
    pub fn read_last_write_table(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "writeTable")?;
        peripheral::decode(&data)
    }
}
