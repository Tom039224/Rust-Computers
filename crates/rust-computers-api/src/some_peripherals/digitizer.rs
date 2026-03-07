//! Some-Peripherals Digitizer。

use alloc::string::String;

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
    pub async fn digitize_amount(
        &self,
        amount: Option<u32>,
    ) -> Result<String, PeripheralError> {
        let args = match amount {
            Some(a) => msgpack::array(&[msgpack::int(a as i32)]),
            None => msgpack::array(&[]),
        };
        let data = peripheral::do_action(self.addr, "digitizeAmount", &args).await?;
        peripheral::decode(&data)
    }

    /// デジタルアイテムを物理化してスロット0に戻す。
    pub async fn rematerialize_amount(
        &self,
        uuid: &str,
        amount: Option<u32>,
    ) -> Result<bool, PeripheralError> {
        let mut args = alloc::vec![msgpack::str(uuid)];
        if let Some(a) = amount {
            args.push(msgpack::int(a as i32));
        }
        let data =
            peripheral::do_action(self.addr, "rematerializeAmount", &msgpack::array(&args))
                .await?;
        peripheral::decode(&data)
    }

    /// 2つのデジタルアイテムを合成する。
    pub async fn merge_digital_items(
        &self,
        into_uuid: &str,
        from_uuid: &str,
        amount: Option<u32>,
    ) -> Result<bool, PeripheralError> {
        let mut args = alloc::vec![msgpack::str(into_uuid), msgpack::str(from_uuid)];
        if let Some(a) = amount {
            args.push(msgpack::int(a as i32));
        }
        let data =
            peripheral::do_action(self.addr, "mergeDigitalItems", &msgpack::array(&args))
                .await?;
        peripheral::decode(&data)
    }

    /// デジタルアイテムスタックを分割する。
    pub async fn separate_digital_item(
        &self,
        from_uuid: &str,
        amount: u32,
    ) -> Result<String, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(from_uuid), msgpack::int(amount as i32)]);
        let data =
            peripheral::do_action(self.addr, "separateDigitalItem", &args).await?;
        peripheral::decode(&data)
    }

    /// UUID が存在するか確認する。
    pub async fn check_id(&self, uuid: &str) -> Result<SPItemData, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(uuid)]);
        let data = peripheral::request_info(self.addr, "checkId", &args).await?;
        peripheral::decode(&data)
    }

    /// スロット0 のアイテム情報を返す。
    pub async fn get_item_in_slot(&self) -> Result<SPItemData, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getItemInSlot",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// スロット0 のアイテム上限数を返す。
    pub async fn get_item_limit_in_slot(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getItemLimitInSlot",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }
}
