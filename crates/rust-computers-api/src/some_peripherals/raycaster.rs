//! Some-Peripherals Raycaster。

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
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
    pub fn book_next_raycast(
        &mut self,
        distance: f64,
        variables: Option<(f64, f64, Option<f64>)>,
        euler_mode: Option<bool>,
        im_execute: Option<bool>,
        check_for_blocks: Option<bool>,
        only_distance: Option<bool>,
    ) {
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
        peripheral::book_request(self.addr, "raycast", &msgpack::array(&args));
    }

    pub fn read_last_raycast(&self) -> Result<SPRaycastResult, PeripheralError> {
        let data = peripheral::read_result(self.addr, "raycast")?;
        peripheral::decode(&data)
    }

    /// ステッカーの powered 状態を設定する。
    pub fn book_next_add_stickers(&mut self, state: bool) {
        let args = msgpack::array(&[msgpack::bool_val(state)]);
        peripheral::book_action(self.addr, "addStickers", &args);
    }

    pub fn read_last_add_stickers(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "addStickers")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
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

    /// ブロックの向きを取得する (imm 対応)。
    pub fn book_next_get_facing_direction(&mut self) {
        peripheral::book_request(self.addr, "getFacingDirection", &msgpack::array(&[]));
    }

    pub fn read_last_get_facing_direction(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getFacingDirection")?;
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

impl Raycaster {
    pub async fn async_raycast(
        &mut self,
        distance: f64,
        variables: Option<(f64, f64, Option<f64>)>,
        euler_mode: Option<bool>,
        im_execute: Option<bool>,
        check_for_blocks: Option<bool>,
        only_distance: Option<bool>,
    ) -> Result<SPRaycastResult, PeripheralError> {
        self.book_next_raycast(distance, variables, euler_mode, im_execute, check_for_blocks, only_distance);
        crate::wait_for_next_tick().await;
        self.read_last_raycast()
    }

    pub async fn async_add_stickers(&mut self, state: bool) -> Vec<Result<(), PeripheralError>> {
        self.book_next_add_stickers(state);
        crate::wait_for_next_tick().await;
        self.read_last_add_stickers()
    }

    pub async fn async_get_config_info(&mut self) -> Result<BTreeMap<String, String>, PeripheralError> {
        self.book_next_get_config_info();
        crate::wait_for_next_tick().await;
        self.read_last_get_config_info()
    }

    pub async fn async_get_facing_direction(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_facing_direction();
        crate::wait_for_next_tick().await;
        self.read_last_get_facing_direction()
    }
}
