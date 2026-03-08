//! Create Tablecloth Shop ペリフェラル。
//! Create Tablecloth Shop peripheral.

use alloc::string::String;
use alloc::vec::Vec;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::common::CRItemDetail;

/// Tablecloth Shop ペリフェラル。
pub struct TableclothShop {
    addr: PeriphAddr,
}

impl Peripheral for TableclothShop {
    const NAME: &'static str = "create:tablecloth_shop";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl TableclothShop {
    /// ショップとして機能しているかどうかを取得する。
    pub fn book_next_is_shop(&mut self) {
        peripheral::book_request(
            self.addr,
            "isShop",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_shop(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isShop")?;
        peripheral::decode(&data)
    }

    /// アドレスを取得する。
    pub fn book_next_get_address(&mut self) {
        peripheral::book_request(
            self.addr,
            "getAddress",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_address(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAddress")?;
        peripheral::decode(&data)
    }

    /// アドレスを設定する。
    pub fn book_next_set_address(&mut self, address: &str) {
        let args = msgpack::array(&[msgpack::str(address)]);
        peripheral::book_action(self.addr, "setAddress", &args);
    }

    pub fn read_last_set_address(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setAddress")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 価格タグアイテムを取得する。
    pub fn book_next_get_price_tag_item(&mut self) {
        peripheral::book_request(
            self.addr,
            "getPriceTagItem",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_price_tag_item(&self) -> Result<Option<CRItemDetail>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPriceTagItem")?;
        peripheral::decode(&data)
    }

    /// 価格タグアイテムを設定する。
    pub fn book_next_set_price_tag_item(&mut self, item_name: &str) {
        let args = msgpack::array(&[msgpack::str(item_name)]);
        peripheral::book_action(self.addr, "setPriceTagItem", &args);
    }

    pub fn read_last_set_price_tag_item(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setPriceTagItem")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 価格タグの数量を取得する。
    pub fn book_next_get_price_tag_count(&mut self) {
        peripheral::book_request(
            self.addr,
            "getPriceTagCount",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_price_tag_count(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPriceTagCount")?;
        peripheral::decode(&data)
    }

    /// 価格タグの数量を設定する。
    pub fn book_next_set_price_tag_count(&mut self, count: u32) {
        let args = msgpack::array(&[msgpack::int(count as i32)]);
        peripheral::book_action(self.addr, "setPriceTagCount", &args);
    }

    pub fn read_last_set_price_tag_count(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setPriceTagCount")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 商品情報を取得する。
    pub fn book_next_get_wares(&mut self) {
        peripheral::book_request(
            self.addr,
            "getWares",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_wares(&self) -> Result<Option<CRItemDetail>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getWares")?;
        peripheral::decode(&data)
    }

    /// 商品情報を設定する。
    pub fn book_next_set_wares(&mut self, item_name: &str) {
        let args = msgpack::array(&[msgpack::str(item_name)]);
        peripheral::book_action(self.addr, "setWares", &args);
    }

    pub fn read_last_set_wares(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setWares")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
