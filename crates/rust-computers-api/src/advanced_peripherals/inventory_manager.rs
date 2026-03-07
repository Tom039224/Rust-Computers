//! AdvancedPeripherals InventoryManager。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// アイテムエントリ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ADItemEntry {
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub count: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "maxStackSize")]
    pub max_stack_size: u32,
    #[serde(default)]
    pub components: crate::msgpack::Value,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub slot: Option<u32>,
}

/// InventoryManager ペリフェラル。
pub struct InventoryManager {
    addr: PeriphAddr,
}

impl Peripheral for InventoryManager {
    const NAME: &'static str = "advancedPeripherals:inventory_manager";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl InventoryManager {
    /// オーナーを取得する (imm 対応)。
    pub async fn get_owner(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getOwner",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_owner_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getOwner",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// プレイヤーにアイテムを追加する。
    pub async fn add_item_to_player(
        &self,
        slot: u32,
        count: Option<u32>,
    ) -> Result<u32, PeripheralError> {
        let mut args = alloc::vec![msgpack::int(slot as i32)];
        if let Some(c) = count {
            args.push(msgpack::int(c as i32));
        }
        let data =
            peripheral::do_action(self.addr, "addItemToPlayer", &msgpack::array(&args))
                .await?;
        peripheral::decode(&data)
    }

    /// プレイヤーからアイテムを除去する。
    pub async fn remove_item_from_player(
        &self,
        slot: u32,
        count: Option<u32>,
    ) -> Result<u32, PeripheralError> {
        let mut args = alloc::vec![msgpack::int(slot as i32)];
        if let Some(c) = count {
            args.push(msgpack::int(c as i32));
        }
        let data =
            peripheral::do_action(self.addr, "removeItemFromPlayer", &msgpack::array(&args))
                .await?;
        peripheral::decode(&data)
    }

    /// インベントリ一覧を取得する。
    pub async fn list(&self) -> Result<Vec<ADItemEntry>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "list",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 防具一覧を取得する。
    pub async fn get_armor(&self) -> Result<Vec<ADItemEntry>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getArmor",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// プレイヤーが装備しているかどうか。
    pub async fn is_player_equipped(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "isPlayerEquipped",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 指定スロットに装備しているかどうか。
    pub async fn is_wearing(&self, slot: u32) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        let data =
            peripheral::request_info(self.addr, "isWearing", &args).await?;
        peripheral::decode(&data)
    }

    /// メインハンドのアイテムを取得する。
    pub async fn get_item_in_hand(&self) -> Result<ADItemEntry, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getItemInHand",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// オフハンドのアイテムを取得する。
    pub async fn get_item_in_off_hand(&self) -> Result<ADItemEntry, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getItemInOffHand",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 空きスペース数を取得する。
    pub async fn get_empty_space(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getEmptySpace",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 空きスペースがあるかどうか。
    pub async fn is_space_available(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "isSpaceAvailable",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 最初の空きスロットを取得する (なければ -1)。
    pub async fn get_free_slot(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getFreeSlot",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// チェストインベントリ一覧を取得する。
    pub async fn list_chest(&self) -> Result<Vec<ADItemEntry>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "listChest",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }
}
