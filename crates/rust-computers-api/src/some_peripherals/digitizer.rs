//! Some-Peripherals Digitizer。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// デジタイズアイテムデータ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPItemData {
    pub id: String,
    pub count: u32,
    #[serde(default)]
    pub tag: crate::msgpack::Value,
}

/// デジタイズ済みアイテム。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPDigitizedItem {
    pub uuid: String,
}

/// Digitizer ペリフェラル。
pub struct Digitizer {
    addr: PeriphAddr,
}

impl Peripheral for Digitizer {
    const NAME: &'static str = "sp:digitizer";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Digitizer {
    /// スロット0 のアイテムをデジタル化してUUIDを返す。
    pub fn book_next_digitize_amount(&mut self, amount: Option<u32>) {
        let args = match amount {
            Some(a) => msgpack::array(&[msgpack::int(a as i32)]),
            None => msgpack::array(&[]),
        };
        peripheral::book_action(self.addr, "digitizeAmount", &args);
    }

    pub fn read_last_digitize_amount(&self) -> Vec<Result<String, PeripheralError>> {
        peripheral::read_action_results(self.addr, "digitizeAmount")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// デジタルアイテムを物理化してスロット0に戻す。
    pub fn book_next_rematerialize_amount(&mut self, uuid: &str, amount: Option<u32>) {
        let mut args = alloc::vec![msgpack::str(uuid)];
        if let Some(a) = amount {
            args.push(msgpack::int(a as i32));
        }
        peripheral::book_action(self.addr, "rematerializeAmount", &msgpack::array(&args));
    }

    pub fn read_last_rematerialize_amount(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "rematerializeAmount")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 2つのデジタルアイテムを合成する。
    pub fn book_next_merge_digital_items(
        &mut self,
        into_uuid: &str,
        from_uuid: &str,
        amount: Option<u32>,
    ) {
        let mut args = alloc::vec![msgpack::str(into_uuid), msgpack::str(from_uuid)];
        if let Some(a) = amount {
            args.push(msgpack::int(a as i32));
        }
        peripheral::book_action(self.addr, "mergeDigitalItems", &msgpack::array(&args));
    }

    pub fn read_last_merge_digital_items(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "mergeDigitalItems")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// デジタルアイテムスタックを分割する。
    pub fn book_next_separate_digital_item(&mut self, from_uuid: &str, amount: u32) {
        let args = msgpack::array(&[msgpack::str(from_uuid), msgpack::int(amount as i32)]);
        peripheral::book_action(self.addr, "separateDigitalItem", &args);
    }

    pub fn read_last_separate_digital_item(&self) -> Vec<Result<String, PeripheralError>> {
        peripheral::read_action_results(self.addr, "separateDigitalItem")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// UUID が存在するか確認する。
    pub fn book_next_check_id(&mut self, uuid: &str) {
        let args = msgpack::array(&[msgpack::str(uuid)]);
        peripheral::book_request(self.addr, "checkId", &args);
    }

    pub fn read_last_check_id(&self) -> Result<SPItemData, PeripheralError> {
        let data = peripheral::read_result(self.addr, "checkId")?;
        peripheral::decode(&data)
    }

    /// スロット0 のアイテム情報を返す。
    pub fn book_next_get_item_in_slot(&mut self) {
        peripheral::book_request(self.addr, "getItemInSlot", &msgpack::array(&[]));
    }

    pub fn read_last_get_item_in_slot(&self) -> Result<SPItemData, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItemInSlot")?;
        peripheral::decode(&data)
    }

    /// スロット0 のアイテム上限数を返す。
    pub fn book_next_get_item_limit_in_slot(&mut self) {
        peripheral::book_request(self.addr, "getItemLimitInSlot", &msgpack::array(&[]));
    }

    pub fn read_last_get_item_limit_in_slot(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItemLimitInSlot")?;
        peripheral::decode(&data)
    }
}

impl Digitizer {
    pub async fn async_digitize_amount(&mut self, amount: Option<u32>) -> Vec<Result<String, PeripheralError>> {
        self.book_next_digitize_amount(amount);
        crate::wait_for_next_tick().await;
        self.read_last_digitize_amount()
    }

    pub async fn async_rematerialize_amount(&mut self, uuid: &str, amount: Option<u32>) -> Vec<Result<bool, PeripheralError>> {
        self.book_next_rematerialize_amount(uuid, amount);
        crate::wait_for_next_tick().await;
        self.read_last_rematerialize_amount()
    }

    pub async fn async_merge_digital_items(&mut self, into_uuid: &str, from_uuid: &str, amount: Option<u32>) -> Vec<Result<bool, PeripheralError>> {
        self.book_next_merge_digital_items(into_uuid, from_uuid, amount);
        crate::wait_for_next_tick().await;
        self.read_last_merge_digital_items()
    }

    pub async fn async_separate_digital_item(&mut self, from_uuid: &str, amount: u32) -> Vec<Result<String, PeripheralError>> {
        self.book_next_separate_digital_item(from_uuid, amount);
        crate::wait_for_next_tick().await;
        self.read_last_separate_digital_item()
    }

    pub async fn async_check_id(&mut self, uuid: &str) -> Result<SPItemData, PeripheralError> {
        self.book_next_check_id(uuid);
        crate::wait_for_next_tick().await;
        self.read_last_check_id()
    }

    pub async fn async_get_item_in_slot(&mut self) -> Result<SPItemData, PeripheralError> {
        self.book_next_get_item_in_slot();
        crate::wait_for_next_tick().await;
        self.read_last_get_item_in_slot()
    }

    pub async fn async_get_item_limit_in_slot(&mut self) -> Result<u32, PeripheralError> {
        self.book_next_get_item_limit_in_slot();
        crate::wait_for_next_tick().await;
        self.read_last_get_item_limit_in_slot()
    }
}
