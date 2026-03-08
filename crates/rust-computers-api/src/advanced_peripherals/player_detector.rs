//! AdvancedPeripherals PlayerDetector。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// プレイヤー情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ADPlayerInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub health: Option<f64>,
    #[serde(default)]
    pub max_health: Option<f64>,
    #[serde(default)]
    pub is_flying: Option<bool>,
    #[serde(default)]
    pub is_sprinting: Option<bool>,
    #[serde(default)]
    pub is_sneaking: Option<bool>,
    #[serde(default)]
    pub game_mode: Option<String>,
    #[serde(default)]
    pub experience: Option<u32>,
    #[serde(default)]
    pub level: Option<u32>,
    #[serde(default)]
    pub pitch: Option<f64>,
}

/// PlayerDetector ペリフェラル。
pub struct PlayerDetector {
    addr: PeriphAddr,
}

impl Peripheral for PlayerDetector {
    const NAME: &'static str = "advancedPeripherals:player_detector";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl PlayerDetector {
    /// オンラインプレイヤー一覧を取得する。
    pub fn book_next_get_online_players(&mut self) {
        peripheral::book_request(self.addr, "getOnlinePlayers", &msgpack::array(&[]));
    }
    pub fn read_last_get_online_players(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getOnlinePlayers")?;
        peripheral::decode(&data)
    }

    /// 範囲内のプレイヤーを取得する。
    pub fn book_next_get_players_in_range(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "getPlayersInRange", &args);
    }
    pub fn read_last_get_players_in_range(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPlayersInRange")?;
        peripheral::decode(&data)
    }

    /// 座標範囲内のプレイヤーを取得する。
    #[allow(clippy::too_many_arguments)]
    pub fn book_next_get_players_in_coords(
        &mut self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(x1),
            msgpack::float64(y1),
            msgpack::float64(z1),
            msgpack::float64(x2),
            msgpack::float64(y2),
            msgpack::float64(z2),
        ]);
        peripheral::book_request(self.addr, "getPlayersInCoords", &args);
    }
    pub fn read_last_get_players_in_coords(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPlayersInCoords")?;
        peripheral::decode(&data)
    }

    /// 立方体範囲内のプレイヤーを取得する。
    pub fn book_next_get_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64) {
        let args = msgpack::array(&[
            msgpack::float64(dx),
            msgpack::float64(dy),
            msgpack::float64(dz),
        ]);
        peripheral::book_request(self.addr, "getPlayersInCubic", &args);
    }
    pub fn read_last_get_players_in_cubic(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPlayersInCubic")?;
        peripheral::decode(&data)
    }

    /// 範囲内にプレイヤーがいるかどうか。
    pub fn book_next_is_players_in_range(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "isPlayersInRange", &args);
    }
    pub fn read_last_is_players_in_range(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPlayersInRange")?;
        peripheral::decode(&data)
    }

    /// 座標範囲内にプレイヤーがいるかどうか。
    #[allow(clippy::too_many_arguments)]
    pub fn book_next_is_players_in_coords(
        &mut self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(x1),
            msgpack::float64(y1),
            msgpack::float64(z1),
            msgpack::float64(x2),
            msgpack::float64(y2),
            msgpack::float64(z2),
        ]);
        peripheral::book_request(self.addr, "isPlayersInCoords", &args);
    }
    pub fn read_last_is_players_in_coords(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPlayersInCoords")?;
        peripheral::decode(&data)
    }

    /// 立方体範囲内にプレイヤーがいるかどうか。
    pub fn book_next_is_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64) {
        let args = msgpack::array(&[
            msgpack::float64(dx),
            msgpack::float64(dy),
            msgpack::float64(dz),
        ]);
        peripheral::book_request(self.addr, "isPlayersInCubic", &args);
    }
    pub fn read_last_is_players_in_cubic(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPlayersInCubic")?;
        peripheral::decode(&data)
    }

    /// 指定プレイヤーが範囲内にいるかどうか。
    pub fn book_next_is_player_in_range(&mut self, player: &str, radius: f64) {
        let args = msgpack::array(&[msgpack::str(player), msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "isPlayerInRange", &args);
    }
    pub fn read_last_is_player_in_range(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPlayerInRange")?;
        peripheral::decode(&data)
    }

    /// 指定プレイヤーが座標範囲内にいるかどうか。
    #[allow(clippy::too_many_arguments)]
    pub fn book_next_is_player_in_coords(
        &mut self,
        player: &str,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::str(player),
            msgpack::float64(x1),
            msgpack::float64(y1),
            msgpack::float64(z1),
            msgpack::float64(x2),
            msgpack::float64(y2),
            msgpack::float64(z2),
        ]);
        peripheral::book_request(self.addr, "isPlayerInCoords", &args);
    }
    pub fn read_last_is_player_in_coords(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPlayerInCoords")?;
        peripheral::decode(&data)
    }

    /// 指定プレイヤーが立方体範囲内にいるかどうか。
    pub fn book_next_is_player_in_cubic(
        &mut self,
        player: &str,
        dx: f64,
        dy: f64,
        dz: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::str(player),
            msgpack::float64(dx),
            msgpack::float64(dy),
            msgpack::float64(dz),
        ]);
        peripheral::book_request(self.addr, "isPlayerInCubic", &args);
    }
    pub fn read_last_is_player_in_cubic(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isPlayerInCubic")?;
        peripheral::decode(&data)
    }

    /// プレイヤー位置を取得する。
    pub fn book_next_get_player_pos(
        &mut self,
        player: &str,
        decimals: Option<u32>,
    ) {
        let mut args = alloc::vec![msgpack::str(player)];
        if let Some(d) = decimals {
            args.push(msgpack::int(d as i32));
        }
        peripheral::book_request(self.addr, "getPlayerPos", &msgpack::array(&args));
    }
    pub fn read_last_get_player_pos(&self) -> Result<ADPlayerInfo, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPlayerPos")?;
        peripheral::decode(&data)
    }

    /// プレイヤー詳細情報を取得する。
    pub fn book_next_get_player(
        &mut self,
        player: &str,
        decimals: Option<u32>,
    ) {
        let mut args = alloc::vec![msgpack::str(player)];
        if let Some(d) = decimals {
            args.push(msgpack::int(d as i32));
        }
        peripheral::book_request(self.addr, "getPlayer", &msgpack::array(&args));
    }
    pub fn read_last_get_player(&self) -> Result<ADPlayerInfo, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPlayer")?;
        peripheral::decode(&data)
    }
}
