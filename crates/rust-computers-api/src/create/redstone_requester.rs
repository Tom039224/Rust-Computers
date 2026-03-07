//! Create Redstone Requester ペリフェラル。
//! Create Redstone Requester peripheral.

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;

use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

use super::common::{CRItemFilter, CROrderItem, CRPackage};

/// Redstone Requester ペリフェラル。
pub struct RedstoneRequester {
    dir: Direction,
}

impl Peripheral for RedstoneRequester {
    const NAME: &'static str = "create:redstone_requester";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl RedstoneRequester {
    /// リクエストを送信する。
    pub async fn request(
        &self,
        items: &[CROrderItem],
    ) -> Result<(), PeripheralError> {
        let items_vec: Vec<_> = items.iter().cloned().collect();
        let encoded = peripheral::encode(&items_vec)?;
        let args = msgpack::array(&[encoded]);
        peripheral::do_action(self.dir, "request", &args).await?;
        Ok(())
    }

    /// リクエストを設定する。
    pub async fn set_request(
        &self,
        slot: u32,
        item: &CROrderItem,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(item)?;
        let args = msgpack::array(&[msgpack::int(slot as i32), encoded]);
        peripheral::do_action(self.dir, "setRequest", &args).await?;
        Ok(())
    }

    /// クラフティングリクエストを設定する。
    pub async fn set_crafting_request(
        &self,
        slot: u32,
        item: &CROrderItem,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(item)?;
        let args = msgpack::array(&[msgpack::int(slot as i32), encoded]);
        peripheral::do_action(self.dir, "setCraftingRequest", &args).await?;
        Ok(())
    }

    /// リクエスト情報を取得する。
    pub async fn get_request(
        &self,
        slot: u32,
    ) -> Result<Option<CRItemFilter>, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        let data =
            peripheral::request_info(self.dir, "getRequest", &args).await?;
        peripheral::decode(&data)
    }

    /// 構成情報を取得する。
    pub async fn get_configuration(&self) -> Result<CRPackage, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getConfiguration",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 構成情報を設定する。
    pub async fn set_configuration(
        &self,
        config: &CRPackage,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(config)?;
        let args = msgpack::array(&[encoded]);
        peripheral::do_action(self.dir, "setConfiguration", &args).await?;
        Ok(())
    }

    /// アドレスを設定する。
    pub async fn set_address(&self, address: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(address)]);
        peripheral::do_action(self.dir, "setAddress", &args).await?;
        Ok(())
    }

    /// アドレスを取得する。
    pub async fn get_address(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getAddress",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }
}
