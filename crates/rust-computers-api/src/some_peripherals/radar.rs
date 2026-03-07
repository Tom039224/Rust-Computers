//! Some-Peripherals Radar。

use alloc::string::String;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::cc_vs::ship::VSQuaternion;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

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
    dir: Direction,
}

impl Peripheral for Radar {
    const NAME: &'static str = "sp_radar";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl Radar {
    /// エンティティをスキャンする。
    pub async fn scan_for_entities(
        &self,
        radius: f64,
    ) -> Result<Vec<SPEntityInfo>, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data =
            peripheral::request_info(self.dir, "scanForEntities", &args).await?;
        peripheral::decode(&data)
    }

    /// VS シップをスキャンする。
    pub async fn scan_for_ships(
        &self,
        radius: f64,
    ) -> Result<Vec<SPShipInfo>, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data =
            peripheral::request_info(self.dir, "scanForShips", &args).await?;
        peripheral::decode(&data)
    }

    /// プレイヤーをスキャンする。
    pub async fn scan_for_players(
        &self,
        radius: f64,
    ) -> Result<Vec<SPEntityInfo>, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data =
            peripheral::request_info(self.dir, "scanForPlayers", &args).await?;
        peripheral::decode(&data)
    }

    /// 設定情報を取得する (imm 対応)。
    pub async fn get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getConfigInfo",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getConfigInfo",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
