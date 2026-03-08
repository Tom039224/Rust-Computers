//! Create Sticker ペリフェラル。
//! Create Sticker peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Sticker ペリフェラル。
pub struct Sticker {
    addr: PeriphAddr,
}

impl Peripheral for Sticker {
    const NAME: &'static str = "create:sticker";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Sticker {
    /// 伸展状態かどうかを取得する。
    pub fn book_next_is_extended(&mut self) {
        peripheral::book_request(
            self.addr,
            "isExtended",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_extended(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isExtended")?;
        peripheral::decode(&data)
    }

    /// 伸展状態かどうかを即時取得する (imm 対応)。
    pub fn is_extended_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isExtended",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ブロックに接続されているかどうかを取得する。
    pub fn book_next_is_attached_to_block(&mut self) {
        peripheral::book_request(
            self.addr,
            "isAttachedToBlock",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_attached_to_block(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isAttachedToBlock")?;
        peripheral::decode(&data)
    }

    /// ブロックに接続されているかどうかを即時取得する (imm 対応)。
    pub fn is_attached_to_block_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isAttachedToBlock",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// スティッカーを伸展させる。成功したかどうかを返す。
    pub fn book_next_extend(&mut self) {
        peripheral::book_action(self.addr, "extend", &msgpack::array(&[]));
    }

    pub fn read_last_extend(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "extend")?;
        peripheral::decode(&data)
    }

    /// スティッカーを収縮させる。成功したかどうかを返す。
    pub fn book_next_retract(&mut self) {
        peripheral::book_action(self.addr, "retract", &msgpack::array(&[]));
    }

    pub fn read_last_retract(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "retract")?;
        peripheral::decode(&data)
    }

    /// スティッカーの伸展/収縮を切り替える。成功したかどうかを返す。
    pub fn book_next_toggle(&mut self) {
        peripheral::book_action(self.addr, "toggle", &msgpack::array(&[]));
    }

    pub fn read_last_toggle(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "toggle")?;
        peripheral::decode(&data)
    }
}
