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
    pub fn book_next_get_owner(&mut self) {
        peripheral::book_request(self.addr, "getOwner", &msgpack::array(&[]));
    }
    pub fn read_last_get_owner(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getOwner")?;
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
    pub fn book_next_add_item_to_player(
        &mut self,
        slot: u32,
        count: Option<u32>,
    ) {
        let mut args = alloc::vec![msgpack::int(slot as i32)];
        if let Some(c) = count {
            args.push(msgpack::int(c as i32));
        }
        peripheral::book_action(self.addr, "addItemToPlayer", &msgpack::array(&args));
    }
    pub fn read_last_add_item_to_player(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "addItemToPlayer")?;
        peripheral::decode(&data)
    }

    /// プレイヤーからアイテムを除去する。
    pub fn book_next_remove_item_from_player(
        &mut self,
        slot: u32,
        count: Option<u32>,
    ) {
        let mut args = alloc::vec![msgpack::int(slot as i32)];
        if let Some(c) = count {
            args.push(msgpack::int(c as i32));
        }
        peripheral::book_action(self.addr, "removeItemFromPlayer", &msgpack::array(&args));
    }
    pub fn read_last_remove_item_from_player(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "removeItemFromPlayer")?;
        peripheral::decode(&data)
    }

    /// インベントリ一覧を取得する。
    pub fn book_next_list(&mut self) {
        peripheral::book_request(self.addr, "list", &msgpack::array(&[]));
    }
    pub fn read_last_list(&self) -> Result<Vec<ADItemEntry>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "list")?;
        peripheral::decode(&data)
    }

    /// 防具一覧を取得する。
    pub fn book_next_get_armor(&mut self) {
        peripheral::book_request(self.addr, "getArmor", &msgpack::array(&[]));
    }
    pub fn read_last_get_armor(&self) -> Result<Vec<ADItemEntry>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getArmor")?;
        peripheral::decode(&data)
    }

    /// プレイヤーが装備しているかどうか。
    pub fn book_next_is_player_equipped(&mut self) {
        peripheral::book_request(self.addr, "isPlayerEquipped", &msgpack::array(&[]));
    }
    pub fn read_last_is_player_equipped(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPlayerEquipped")?;
        peripheral::decode(&data)
    }

    /// 指定スロットに装備しているかどうか。
    pub fn book_next_is_wearing(&mut self, slot: u32) {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        peripheral::book_request(self.addr, "isWearing", &args);
    }
    pub fn read_last_is_wearing(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isWearing")?;
        peripheral::decode(&data)
    }

    /// メインハンドのアイテムを取得する。
    pub fn book_next_get_item_in_hand(&mut self) {
        peripheral::book_request(self.addr, "getItemInHand", &msgpack::array(&[]));
    }
    pub fn read_last_get_item_in_hand(&self) -> Result<ADItemEntry, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItemInHand")?;
        peripheral::decode(&data)
    }

    /// オフハンドのアイテムを取得する。
    pub fn book_next_get_item_in_off_hand(&mut self) {
        peripheral::book_request(self.addr, "getItemInOffHand", &msgpack::array(&[]));
    }
    pub fn read_last_get_item_in_off_hand(&self) -> Result<ADItemEntry, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItemInOffHand")?;
        peripheral::decode(&data)
    }

    /// 空きスペース数を取得する。
    pub fn book_next_get_empty_space(&mut self) {
        peripheral::book_request(self.addr, "getEmptySpace", &msgpack::array(&[]));
    }
    pub fn read_last_get_empty_space(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getEmptySpace")?;
        peripheral::decode(&data)
    }

    /// 空きスペースがあるかどうか。
    pub fn book_next_is_space_available(&mut self) {
        peripheral::book_request(self.addr, "isSpaceAvailable", &msgpack::array(&[]));
    }
    pub fn read_last_is_space_available(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isSpaceAvailable")?;
        peripheral::decode(&data)
    }

    /// 最初の空きスロットを取得する (なければ -1)。
    pub fn book_next_get_free_slot(&mut self) {
        peripheral::book_request(self.addr, "getFreeSlot", &msgpack::array(&[]));
    }
    pub fn read_last_get_free_slot(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getFreeSlot")?;
        peripheral::decode(&data)
    }

    /// チェストインベントリ一覧を取得する。
    pub fn book_next_list_chest(&mut self) {
        peripheral::book_request(self.addr, "listChest", &msgpack::array(&[]));
    }
    pub fn read_last_list_chest(&self) -> Result<Vec<ADItemEntry>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "listChest")?;
        peripheral::decode(&data)
    }
}
