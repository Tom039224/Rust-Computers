//! Some-Peripherals Raycaster。

use alloc::collections::BTreeMap;
use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::ballistic_accelerator::SPCoordinate;

/// レイキャスト結果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPRaycastResult {
    #[serde(default)]
    pub is_block: Option<bool>,
    #[serde(default)]
    pub is_entity: Option<bool>,
    #[serde(default)]
    pub abs_pos: Option<SPCoordinate>,
    #[serde(default)]
    pub hit_pos: Option<SPCoordinate>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub block_type: Option<String>,
    #[serde(default)]
    pub rel_hit_pos: Option<SPCoordinate>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub ship_id: Option<i64>,
    #[serde(default)]
    pub hit_pos_ship: Option<SPCoordinate>,
    #[serde(default)]
    pub error: Option<String>,
}

/// Raycaster ペリフェラル。
pub struct Raycaster {
    addr: PeriphAddr,
}

impl Peripheral for Raycaster {
    const NAME: &'static str = "sp:raycaster";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Raycaster {
    /// レイキャストを実行する。
    #[allow(clippy::too_many_arguments)]
    pub async fn raycast(
        &self,
        distance: f64,
        variables: Option<(f64, f64, Option<f64>)>,
        euler_mode: Option<bool>,
        im_execute: Option<bool>,
        check_for_blocks: Option<bool>,
        only_distance: Option<bool>,
    ) -> Result<SPRaycastResult, PeripheralError> {
        let mut args = alloc::vec![msgpack::float64(distance)];
        if let Some((a, b, c)) = variables {
            args.push(msgpack::float64(a));
            args.push(msgpack::float64(b));
            args.push(c.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        }
        args.push(euler_mode.map_or_else(|| msgpack::nil(), |v| msgpack::bool_val(v)));
        args.push(im_execute.map_or_else(|| msgpack::nil(), |v| msgpack::bool_val(v)));
        args.push(
            check_for_blocks.map_or_else(|| msgpack::nil(), |v| msgpack::bool_val(v)),
        );
        args.push(only_distance.map_or_else(|| msgpack::nil(), |v| msgpack::bool_val(v)));
        let data =
            peripheral::request_info(self.addr, "raycast", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }

    /// ステッカーの powered 状態を設定する。
    pub async fn add_stickers(&self, state: bool) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::bool_val(state)]);
        peripheral::do_action(self.addr, "addStickers", &args).await?;
        Ok(())
    }

    /// 設定情報を取得する (imm 対応)。
    pub async fn get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getConfigInfo",
            &msgpack::array(&[]),
        )
        .await?;
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

    /// ブロックの向きを取得する (imm 対応)。
    pub async fn get_facing_direction(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getFacingDirection",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_facing_direction_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getFacingDirection",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
