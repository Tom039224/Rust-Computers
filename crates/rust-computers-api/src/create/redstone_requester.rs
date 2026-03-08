//! Create Redstone Requester ペリフェラル。
//! Create Redstone Requester peripheral.

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;

use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::common::{CRItemFilter, CROrderItem, CRPackage};

/// Redstone Requester ペリフェラル。
pub struct RedstoneRequester {
    addr: PeriphAddr,
}

impl Peripheral for RedstoneRequester {
    const NAME: &'static str = "create:redstone_requester";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl RedstoneRequester {
    /// リクエストを送信する。
    pub fn book_next_request(
        &mut self,
        items: &[CROrderItem],
    ) -> Result<(), PeripheralError> {
        let items_vec: Vec<_> = items.iter().cloned().collect();
        let encoded = peripheral::encode(&items_vec)?;
        let args = msgpack::array(&[encoded]);
        peripheral::book_action(self.addr, "request", &args);
        Ok(())
    }

    pub fn read_last_request(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "request")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// リクエストを設定する。
    pub fn book_next_set_request(
        &mut self,
        slot: u32,
        item: &CROrderItem,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(item)?;
        let args = msgpack::array(&[msgpack::int(slot as i32), encoded]);
        peripheral::book_action(self.addr, "setRequest", &args);
        Ok(())
    }

    pub fn read_last_set_request(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setRequest")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// クラフティングリクエストを設定する。
    pub fn book_next_set_crafting_request(
        &mut self,
        slot: u32,
        item: &CROrderItem,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(item)?;
        let args = msgpack::array(&[msgpack::int(slot as i32), encoded]);
        peripheral::book_action(self.addr, "setCraftingRequest", &args);
        Ok(())
    }

    pub fn read_last_set_crafting_request(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setCraftingRequest")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// リクエスト情報を取得する。
    pub fn book_next_get_request(&mut self, slot: u32) {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        peripheral::book_request(self.addr, "getRequest", &args);
    }

    pub fn read_last_get_request(&self) -> Result<Option<CRItemFilter>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getRequest")?;
        peripheral::decode(&data)
    }

    /// 構成情報を取得する。
    pub fn book_next_get_configuration(&mut self) {
        peripheral::book_request(
            self.addr,
            "getConfiguration",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_configuration(&self) -> Result<CRPackage, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getConfiguration")?;
        peripheral::decode(&data)
    }

    /// 構成情報を設定する。
    pub fn book_next_set_configuration(
        &mut self,
        config: &CRPackage,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(config)?;
        let args = msgpack::array(&[encoded]);
        peripheral::book_action(self.addr, "setConfiguration", &args);
        Ok(())
    }

    pub fn read_last_set_configuration(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setConfiguration")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
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
}
