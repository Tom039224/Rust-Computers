//! Create Tablecloth Shop ペリフェラル。
//! Create Tablecloth Shop peripheral.

use alloc::string::String;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

use super::common::CRItemDetail;

/// Tablecloth Shop ペリフェラル。
pub struct TableclothShop {
    dir: Direction,
}

impl Peripheral for TableclothShop {
    const NAME: &'static str = "create:tablecloth_shop";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl TableclothShop {
    /// ショップとして機能しているかどうかを取得する。
    pub async fn is_shop(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "isShop",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
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

    /// アドレスを設定する。
    pub async fn set_address(&self, address: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(address)]);
        peripheral::do_action(self.dir, "setAddress", &args).await?;
        Ok(())
    }

    /// 価格タグアイテムを取得する。
    pub async fn get_price_tag_item(&self) -> Result<Option<CRItemDetail>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getPriceTagItem",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 価格タグアイテムを設定する。
    pub async fn set_price_tag_item(&self, item_name: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(item_name)]);
        peripheral::do_action(self.dir, "setPriceTagItem", &args).await?;
        Ok(())
    }

    /// 価格タグの数量を取得する。
    pub async fn get_price_tag_count(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getPriceTagCount",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 価格タグの数量を設定する。
    pub async fn set_price_tag_count(&self, count: u32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(count as i32)]);
        peripheral::do_action(self.dir, "setPriceTagCount", &args).await?;
        Ok(())
    }

    /// 商品情報を取得する。
    pub async fn get_wares(&self) -> Result<Option<CRItemDetail>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getWares",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 商品情報を設定する。
    pub async fn set_wares(&self, item_name: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(item_name)]);
        peripheral::do_action(self.dir, "setWares", &args).await?;
        Ok(())
    }
}
