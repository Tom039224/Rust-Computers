//! AdvancedPeripherals BlockReader。

use alloc::string::String;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// BlockReader ペリフェラル。
pub struct BlockReader {
    addr: PeriphAddr,
}

impl Peripheral for BlockReader {
    const NAME: &'static str = "advancedPeripherals:block_reader";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl BlockReader {
    /// ブロック名（リソース ID）を取得する。
    pub fn book_next_get_block_name(&mut self) {
        peripheral::book_request(self.addr, "getBlockName", &msgpack::array(&[]));
    }
    pub fn read_last_get_block_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBlockName")?;
        peripheral::decode(&data)
    }

    /// ブロックの NBT データをテーブルで取得する。
    pub fn book_next_get_block_data(&mut self) {
        peripheral::book_request(self.addr, "getBlockData", &msgpack::array(&[]));
    }
    pub fn read_last_get_block_data(&self) -> Result<msgpack::Value, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBlockData")?;
        peripheral::decode(&data)
    }

    /// ブロックステートプロパティを取得する。
    pub fn book_next_get_block_states(&mut self) {
        peripheral::book_request(self.addr, "getBlockStates", &msgpack::array(&[]));
    }
    pub fn read_last_get_block_states(&self) -> Result<msgpack::Value, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBlockStates")?;
        peripheral::decode(&data)
    }

    /// タイルエンティティかどうかを取得する。
    pub fn book_next_is_tile_entity(&mut self) {
        peripheral::book_request(self.addr, "isTileEntity", &msgpack::array(&[]));
    }
    pub fn read_last_is_tile_entity(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isTileEntity")?;
        peripheral::decode(&data)
    }
}
