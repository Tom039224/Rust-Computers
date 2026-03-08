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

// -----------------------------------------------------------------------
// entityToMapRadar が返す実際の Kotlin マップ構造:
//   is_entity: bool                (常に true)
//   pos: [x, y, z]                 (設定で有効時。個別 x/y/z フィールドはない)
//   entity_type: String            (設定で有効時。"type" や "id" キーではない)
//   health, max_health: f64        (LivingEntity かつ設定で有効時)
//   is_player: bool                (プレイヤーのみ)
//   nickname: String               (プレイヤーかつ設定で有効時)
//
// Actual Kotlin map structure returned by entityToMapRadar:
//   is_entity: bool                (always true)
//   pos: [x, y, z]                 (when config enabled; NO individual x/y/z keys)
//   entity_type: String            (when config enabled; NOT "type" or "id")
//   health, max_health: f64        (LivingEntity when config enabled)
//   is_player: bool                (players only)
//   nickname: String               (players when config enabled)
// -----------------------------------------------------------------------

/// 3次元座標 / 3D position.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// msgpack デコード用の生構造体。
/// Raw helper struct for msgpack deserialization.
#[derive(Deserialize)]
struct SPEntityInfoRaw {
    #[serde(default)]
    pub is_entity: bool,
    /// 座標リスト [x, y, z]
    pub pos: Option<[f64; 3]>,
    /// エンティティタイプ文字列 (例: "minecraft:skeleton")
    pub entity_type: Option<String>,
    pub is_player: Option<bool>,
    /// プレイヤー名
    pub nickname: Option<String>,
    pub health: Option<f64>,
    pub max_health: Option<f64>,
    pub armor_value: Option<i32>,
    pub is_baby: Option<bool>,
    pub is_blocking: Option<bool>,
    pub is_sleeping: Option<bool>,
    pub is_fall_flying: Option<bool>,
    pub speed: Option<f64>,
}

/// エンティティ情報。
/// `name` にはプレイヤーなら `nickname`、それ以外なら `entity_type` が入る。
#[derive(Debug, Clone, Serialize)]
pub struct SPEntityInfo {
    /// 3次元座標
    pub pos: SPPosition,
    /// エンティティ表示名: プレイヤーは nickname、それ以外は entity_type
    /// Display name: nickname for players, entity_type for others
    pub name: String,
    /// エンティティタイプ文字列 (例: "minecraft:skeleton")
    pub entity_type: String,
    pub is_entity: bool,
    pub is_player: bool,
    pub nickname: Option<String>,
    pub health: Option<f64>,
    pub max_health: Option<f64>,
    pub armor_value: Option<i32>,
    pub is_baby: Option<bool>,
    pub is_blocking: Option<bool>,
    pub is_sleeping: Option<bool>,
    pub is_fall_flying: Option<bool>,
    pub speed: Option<f64>,
}

impl<'de> Deserialize<'de> for SPEntityInfo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = SPEntityInfoRaw::deserialize(deserializer)?;
        let [x, y, z] = raw.pos.unwrap_or([0.0, 0.0, 0.0]);
        let pos = SPPosition { x, y, z };
        let entity_type = raw.entity_type.unwrap_or_default();
        let is_player = raw.is_player.unwrap_or(false);
        let name = if is_player {
            raw.nickname.clone().unwrap_or_else(|| entity_type.clone())
        } else {
            entity_type.clone()
        };
        Ok(SPEntityInfo {
            pos,
            name,
            entity_type,
            is_entity: raw.is_entity,
            is_player,
            nickname: raw.nickname,
            health: raw.health,
            max_health: raw.max_health,
            armor_value: raw.armor_value,
            is_baby: raw.is_baby,
            is_blocking: raw.is_blocking,
            is_sleeping: raw.is_sleeping,
            is_fall_flying: raw.is_fall_flying,
            speed: raw.speed,
        })
    }
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
