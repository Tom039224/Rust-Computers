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
    pub async fn is_extended(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "isExtended",
            &msgpack::array(&[]),
        )
        .await?;
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
    pub async fn is_attached_to_block(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "isAttachedToBlock",
            &msgpack::array(&[]),
        )
        .await?;
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
    pub async fn extend(&self) -> Result<bool, PeripheralError> {
        let data =
            peripheral::do_action(self.addr, "extend", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// スティッカーを収縮させる。成功したかどうかを返す。
    pub async fn retract(&self) -> Result<bool, PeripheralError> {
        let data =
            peripheral::do_action(self.addr, "retract", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// スティッカーの伸展/収縮を切り替える。成功したかどうかを返す。
    pub async fn toggle(&self) -> Result<bool, PeripheralError> {
        let data =
            peripheral::do_action(self.addr, "toggle", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }
}
