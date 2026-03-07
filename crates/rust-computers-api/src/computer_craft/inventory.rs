//! CC:Tweaked Inventory ペリフェラル。
//! CC:Tweaked Inventory peripheral.

use alloc::collections::BTreeMap;
use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// アイテム詳細情報。
/// Detailed item information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDetail {
    pub name: String,
    pub count: u32,
    #[serde(rename = "maxCount")]
    pub max_count: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default)]
    pub damage: Option<u32>,
    #[serde(rename = "maxDamage", default)]
    pub max_damage: Option<u32>,
    #[serde(default)]
    pub tags: BTreeMap<String, bool>,
}

/// スロット簡易情報。
/// Slot summary information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfo {
    pub name: String,
    pub count: u32,
}

/// インベントリペリフェラル。
/// Inventory peripheral.
pub struct Inventory {
    addr: PeriphAddr,
}

impl Peripheral for Inventory {
    const NAME: &'static str = "inventory";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Inventory {
    /// インベントリサイズを取得する。
    /// Get the inventory size.
    pub async fn size(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info(self.addr, "size", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// 全スロットの一覧を取得する。
    /// List all slots.
    pub async fn list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError> {
        let data = peripheral::request_info(self.addr, "list", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// 指定スロットのアイテム詳細を取得する。
    /// Get item detail for the specified slot.
    pub async fn get_item_detail(&self, slot: u32) -> Result<Option<ItemDetail>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getItemDetail",
            &msgpack::array(&[msgpack::int(slot as i32)]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// アイテムを別のインベントリに転送する。
    /// Push items to another inventory.
    pub async fn push_items(
        &self,
        to: &Inventory,
        from_slot: u32,
        limit: Option<u32>,
        to_slot: Option<u32>,
    ) -> Result<u32, PeripheralError> {
        let mut args = alloc::vec![
            msgpack::int(to.addr.raw() as i32),
            msgpack::int(from_slot as i32),
        ];
        if let Some(l) = limit {
            args.push(msgpack::int(l as i32));
            if let Some(ts) = to_slot {
                args.push(msgpack::int(ts as i32));
            }
        } else if let Some(ts) = to_slot {
            args.push(msgpack::nil());
            args.push(msgpack::int(ts as i32));
        }
        let data =
            peripheral::do_action(self.addr, "pushItems", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }

    /// 別のインベントリからアイテムを引き出す。
    /// Pull items from another inventory.
    pub async fn pull_items(
        &self,
        from: &Inventory,
        from_slot: u32,
        limit: Option<u32>,
        to_slot: Option<u32>,
    ) -> Result<u32, PeripheralError> {
        let mut args = alloc::vec![
            msgpack::int(from.addr.raw() as i32),
            msgpack::int(from_slot as i32),
        ];
        if let Some(l) = limit {
            args.push(msgpack::int(l as i32));
            if let Some(ts) = to_slot {
                args.push(msgpack::int(ts as i32));
            }
        } else if let Some(ts) = to_slot {
            args.push(msgpack::nil());
            args.push(msgpack::int(ts as i32));
        }
        let data =
            peripheral::do_action(self.addr, "pullItems", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }
}
