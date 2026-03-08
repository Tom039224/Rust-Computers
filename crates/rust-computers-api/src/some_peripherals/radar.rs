//! Some-Peripherals Radar。

use alloc::string::String;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::cc_vs::ship::VSQuaternion;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::ballistic_accelerator::SPCoordinate;

/// エンティティ情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPEntityInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// エンティティ登録 ID (例: "minecraft:skeleton")
    pub id: String,
    /// エンティティタイプ文字列 (Lua キー: "type")
    #[serde(rename = "type")]
    pub entity_type: String,
    pub name: String,
}

/// シップ情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPShipInfo {
    pub is_ship: bool,
    /// シップ ID (Lua キー: "id")
    #[serde(rename = "id")]
    pub ship_id: i64,
    pub pos: SPCoordinate,
    pub mass: f64,
    pub rotation: VSQuaternion,
    pub velocity: SPCoordinate,
    pub size: SPCoordinate,
    pub scale: SPCoordinate,
    pub moment_of_inertia_tensor: [[f64; 3]; 3],
    /// シップ座標系での重心位置 (Lua キー: "center_of_mass_in_a_ship")
    #[serde(rename = "center_of_mass_in_a_ship")]
    pub center_of_mass_in_ship: SPCoordinate,
}

/// Radar ペリフェラル。
pub struct Radar {
    addr: PeriphAddr,
}

impl Peripheral for Radar {
    const NAME: &'static str = "sp_radar";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Radar {
    /// エンティティをスキャンする。
    pub fn book_next_scan_for_entities(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "scanForEntities", &args);
    }

    pub fn read_last_scan_for_entities(&self) -> Result<Vec<SPEntityInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "scanForEntities")?;
        peripheral::decode(&data)
    }

    /// VS シップをスキャンする。
    pub fn book_next_scan_for_ships(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "scanForShips", &args);
    }

    pub fn read_last_scan_for_ships(&self) -> Result<Vec<SPShipInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "scanForShips")?;
        peripheral::decode(&data)
    }

    /// プレイヤーをスキャンする。
    pub fn book_next_scan_for_players(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "scanForPlayers", &args);
    }

    pub fn read_last_scan_for_players(&self) -> Result<Vec<SPEntityInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "scanForPlayers")?;
        peripheral::decode(&data)
    }

    /// 設定情報を取得する (imm 対応)。
    pub fn book_next_get_config_info(&mut self) {
        peripheral::book_request(self.addr, "getConfigInfo", &msgpack::array(&[]));
    }

    pub fn read_last_get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getConfigInfo")?;
        peripheral::decode(&data)
    }

    pub fn get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getConfigInfo",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
