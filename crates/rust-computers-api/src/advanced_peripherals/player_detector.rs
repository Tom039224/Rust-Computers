//! AdvancedPeripherals PlayerDetector。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

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
    dir: Direction,
}

impl Peripheral for PlayerDetector {
    const NAME: &'static str = "advancedPeripherals:player_detector";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl PlayerDetector {
    /// オンラインプレイヤー一覧を取得する。
    pub async fn get_online_players(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getOnlinePlayers",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 範囲内のプレイヤーを取得する。
    pub async fn get_players_in_range(
        &self,
        radius: f64,
    ) -> Result<Vec<String>, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data =
            peripheral::request_info(self.dir, "getPlayersInRange", &args).await?;
        peripheral::decode(&data)
    }

    /// 座標範囲内のプレイヤーを取得する。
    #[allow(clippy::too_many_arguments)]
    pub async fn get_players_in_coords(
        &self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) -> Result<Vec<String>, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(x1),
            msgpack::float64(y1),
            msgpack::float64(z1),
            msgpack::float64(x2),
            msgpack::float64(y2),
            msgpack::float64(z2),
        ]);
        let data =
            peripheral::request_info(self.dir, "getPlayersInCoords", &args).await?;
        peripheral::decode(&data)
    }

    /// 立方体範囲内のプレイヤーを取得する。
    pub async fn get_players_in_cubic(
        &self,
        dx: f64,
        dy: f64,
        dz: f64,
    ) -> Result<Vec<String>, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(dx),
            msgpack::float64(dy),
            msgpack::float64(dz),
        ]);
        let data =
            peripheral::request_info(self.dir, "getPlayersInCubic", &args).await?;
        peripheral::decode(&data)
    }

    /// 範囲内にプレイヤーがいるかどうか。
    pub async fn is_players_in_range(
        &self,
        radius: f64,
    ) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data =
            peripheral::request_info(self.dir, "isPlayersInRange", &args).await?;
        peripheral::decode(&data)
    }

    /// 座標範囲内にプレイヤーがいるかどうか。
    #[allow(clippy::too_many_arguments)]
    pub async fn is_players_in_coords(
        &self,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(x1),
            msgpack::float64(y1),
            msgpack::float64(z1),
            msgpack::float64(x2),
            msgpack::float64(y2),
            msgpack::float64(z2),
        ]);
        let data =
            peripheral::request_info(self.dir, "isPlayersInCoords", &args).await?;
        peripheral::decode(&data)
    }

    /// 立方体範囲内にプレイヤーがいるかどうか。
    pub async fn is_players_in_cubic(
        &self,
        dx: f64,
        dy: f64,
        dz: f64,
    ) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(dx),
            msgpack::float64(dy),
            msgpack::float64(dz),
        ]);
        let data =
            peripheral::request_info(self.dir, "isPlayersInCubic", &args).await?;
        peripheral::decode(&data)
    }

    /// 指定プレイヤーが範囲内にいるかどうか。
    pub async fn is_player_in_range(
        &self,
        player: &str,
        radius: f64,
    ) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(player), msgpack::float64(radius)]);
        let data =
            peripheral::request_info(self.dir, "isPlayerInRange", &args).await?;
        peripheral::decode(&data)
    }

    /// 指定プレイヤーが座標範囲内にいるかどうか。
    #[allow(clippy::too_many_arguments)]
    pub async fn is_player_in_coords(
        &self,
        player: &str,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::str(player),
            msgpack::float64(x1),
            msgpack::float64(y1),
            msgpack::float64(z1),
            msgpack::float64(x2),
            msgpack::float64(y2),
            msgpack::float64(z2),
        ]);
        let data =
            peripheral::request_info(self.dir, "isPlayerInCoords", &args).await?;
        peripheral::decode(&data)
    }

    /// 指定プレイヤーが立方体範囲内にいるかどうか。
    pub async fn is_player_in_cubic(
        &self,
        player: &str,
        dx: f64,
        dy: f64,
        dz: f64,
    ) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::str(player),
            msgpack::float64(dx),
            msgpack::float64(dy),
            msgpack::float64(dz),
        ]);
        let data =
            peripheral::request_info(self.dir, "isPlayerInCubic", &args).await?;
        peripheral::decode(&data)
    }

    /// プレイヤー位置を取得する。
    pub async fn get_player_pos(
        &self,
        player: &str,
        decimals: Option<u32>,
    ) -> Result<ADPlayerInfo, PeripheralError> {
        let mut args = alloc::vec![msgpack::str(player)];
        if let Some(d) = decimals {
            args.push(msgpack::int(d as i32));
        }
        let data =
            peripheral::request_info(self.dir, "getPlayerPos", &msgpack::array(&args))
                .await?;
        peripheral::decode(&data)
    }

    /// プレイヤー詳細情報を取得する。
    pub async fn get_player(
        &self,
        player: &str,
        decimals: Option<u32>,
    ) -> Result<ADPlayerInfo, PeripheralError> {
        let mut args = alloc::vec![msgpack::str(player)];
        if let Some(d) = decimals {
            args.push(msgpack::int(d as i32));
        }
        let data =
            peripheral::request_info(self.dir, "getPlayer", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }
}
