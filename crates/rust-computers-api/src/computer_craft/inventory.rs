//! CC:Tweaked Inventory ペリフェラル。
//! CC:Tweaked Inventory peripheral.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
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
    // 1. size メソッド
    /// インベントリサイズを取得する（book-read パターン）。
    /// Get the inventory size (book-read pattern).
    pub fn book_next_size(&mut self) {
        peripheral::book_request(self.addr, "size", &[]);
    }
    
    pub fn read_last_size(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "size")?;
        peripheral::decode(&data)
    }
    
    pub async fn async_size(&mut self) -> Result<u32, PeripheralError> {
        self.book_next_size();
        crate::wait_for_next_tick().await;
        self.read_last_size()
    }
    
    // 2. list メソッド
    /// 全スロットの一覧を取得する（book-read パターン）。
    /// List all slots (book-read pattern).
    pub fn book_next_list(&mut self) {
        peripheral::book_request(self.addr, "list", &[]);
    }
    
    pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "list")?;
        peripheral::decode(&data)
    }
    
    pub async fn async_list(&mut self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError> {
        self.book_next_list();
        crate::wait_for_next_tick().await;
        self.read_last_list()
    }
    
    // 3. getItemDetail メソッド
    /// 指定スロットのアイテム詳細を取得する（book-read パターン）。
    /// Get item detail for the specified slot (book-read pattern).
    pub fn book_next_get_item_detail(&mut self, slot: u32) {
        let args = peripheral::encode(&slot).unwrap_or_default();
        peripheral::book_request(self.addr, "getItemDetail", &args);
    }
    
    pub fn read_last_get_item_detail(&self) -> Result<Option<ItemDetail>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItemDetail")?;
        peripheral::decode(&data)
    }
    
    pub async fn async_get_item_detail(&mut self, slot: u32) -> Result<Option<ItemDetail>, PeripheralError> {
        self.book_next_get_item_detail(slot);
        crate::wait_for_next_tick().await;
        self.read_last_get_item_detail()
    }
    
    // 4. pushItems メソッド（アクション系）
    /// アイテムを別のインベントリに転送する（アクション系）。
    /// Push items to another inventory (action pattern).
    pub fn book_next_push_items(
        &mut self,
        to: &Inventory,
        from_slot: u32,
        limit: Option<u32>,
        to_slot: Option<u32>,
    ) {
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
        peripheral::book_action(self.addr, "pushItems", &msgpack::array(&args));
    }
    
    pub fn read_last_push_items(&self) -> Vec<Result<u32, PeripheralError>> {
        peripheral::read_action_results(self.addr, "pushItems")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }
    
    pub async fn async_push_items(
        &mut self,
        to: &Inventory,
        from_slot: u32,
        limit: Option<u32>,
        to_slot: Option<u32>,
    ) -> Result<u32, PeripheralError> {
        self.book_next_push_items(to, from_slot, limit, to_slot);
        crate::wait_for_next_tick().await;
        self.read_last_push_items()
            .into_iter()
            .next()
            .unwrap_or(Ok(0))
    }
    
    // 5. pullItems メソッド（アクション系）
    /// 別のインベントリからアイテムを引き出す（アクション系）。
    /// Pull items from another inventory (action pattern).
    pub fn book_next_pull_items(
        &mut self,
        from: &Inventory,
        from_slot: u32,
        limit: Option<u32>,
        to_slot: Option<u32>,
    ) {
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
        peripheral::book_action(self.addr, "pullItems", &msgpack::array(&args));
    }
    
    pub fn read_last_pull_items(&self) -> Vec<Result<u32, PeripheralError>> {
        peripheral::read_action_results(self.addr, "pullItems")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }
    
    pub async fn async_pull_items(
        &mut self,
        from: &Inventory,
        from_slot: u32,
        limit: Option<u32>,
        to_slot: Option<u32>,
    ) -> Result<u32, PeripheralError> {
        self.book_next_pull_items(from, from_slot, limit, to_slot);
        crate::wait_for_next_tick().await;
        self.read_last_pull_items()
            .into_iter()
            .next()
            .unwrap_or(Ok(0))
    }

    // 6. getItemLimit メソッド
    /// 指定スロットの最大アイテム数を取得する（book-read パターン）。
    /// Get the maximum item count for the specified slot (book-read pattern).
    pub fn book_next_get_item_limit(&mut self, slot: u32) {
        let args = peripheral::encode(&slot).unwrap_or_default();
        peripheral::book_request(self.addr, "getItemLimit", &args);
    }
    
    pub fn read_last_get_item_limit(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItemLimit")?;
        peripheral::decode(&data)
    }
    
    pub async fn async_get_item_limit(&mut self, slot: u32) -> Result<u32, PeripheralError> {
        self.book_next_get_item_limit(slot);
        crate::wait_for_next_tick().await;
        self.read_last_get_item_limit()
    }
}
