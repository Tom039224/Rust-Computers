//! Control-Craft Camera ペリフェラル。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// 4x4 変換行列。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CTLTransform {
    pub matrix: [[f64; 4]; 4],
}

/// レイキャスト結果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTLRaycastResult {
    #[serde(default)]
    pub hit_type: Option<String>,
    #[serde(default)]
    pub pos: Option<(f64, f64, f64)>,
    #[serde(default)]
    pub block_pos: Option<(i32, i32, i32)>,
    #[serde(default)]
    pub entity_id: Option<String>,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub ship_id: Option<i64>,
    #[serde(default)]
    pub player_name: Option<String>,
    #[serde(default)]
    pub distance: Option<f64>,
}

/// Camera ペリフェラル。
pub struct Camera {
    addr: PeriphAddr,
}

impl Peripheral for Camera {
    const NAME: &'static str = "camera";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

macro_rules! book_read_imm {
    ($book:ident, $read:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub fn $book(&mut self) {
            peripheral::book_request(self.addr, $method, &crate::msgpack::array(&[]));
        }
        pub fn $read(&self) -> Result<$ret, PeripheralError> {
            let data = peripheral::read_result(self.addr, $method)?;
            peripheral::decode(&data)
        }
        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.addr, $method, &crate::msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl Camera {
    // ====== imm 対応読み取り系 ======

    book_read_imm!(
        book_next_get_abs_view_transform,
        read_last_get_abs_view_transform,
        get_abs_view_transform_imm,
        "getAbsViewTransform",
        CTLTransform
    );
    book_read_imm!(book_next_get_pitch, read_last_get_pitch, get_pitch_imm, "getPitch", f64);
    book_read_imm!(book_next_get_yaw, read_last_get_yaw, get_yaw_imm, "getYaw", f64);
    book_read_imm!(
        book_next_get_transformed_pitch,
        read_last_get_transformed_pitch,
        get_transformed_pitch_imm,
        "getTransformedPitch",
        f64
    );
    book_read_imm!(
        book_next_get_transformed_yaw,
        read_last_get_transformed_yaw,
        get_transformed_yaw_imm,
        "getTransformedYaw",
        f64
    );
    book_read_imm!(
        book_next_get_clip_distance,
        read_last_get_clip_distance,
        get_clip_distance_imm,
        "getClipDistance",
        f64
    );
    book_read_imm!(
        book_next_latest_ship,
        read_last_latest_ship,
        latest_ship_imm,
        "latestShip",
        Option<crate::msgpack::Value>
    );
    book_read_imm!(
        book_next_latest_player,
        read_last_latest_player,
        latest_player_imm,
        "latestPlayer",
        Option<crate::msgpack::Value>
    );
    book_read_imm!(
        book_next_latest_entity,
        read_last_latest_entity,
        latest_entity_imm,
        "latestEntity",
        Option<crate::msgpack::Value>
    );
    book_read_imm!(
        book_next_latest_block,
        read_last_latest_block,
        latest_block_imm,
        "latestBlock",
        Option<crate::msgpack::Value>
    );
    book_read_imm!(
        book_next_get_camera_position,
        read_last_get_camera_position,
        get_camera_position_imm,
        "getCameraPosition",
        (f64, f64, f64)
    );
    book_read_imm!(
        book_next_get_abs_view_forward,
        read_last_get_abs_view_forward,
        get_abs_view_forward_imm,
        "getAbsViewForward",
        (f64, f64, f64)
    );
    book_read_imm!(book_next_is_being_used, read_last_is_being_used, is_being_used_imm, "isBeingUsed", bool);
    book_read_imm!(
        book_next_get_direction,
        read_last_get_direction,
        get_direction_imm,
        "getDirection",
        String
    );

    // ====== クリップ系 ======

    /// clip (全体)。
    pub fn book_next_clip(&mut self) {
        peripheral::book_request(self.addr, "clip", &msgpack::array(&[]));
    }
    pub fn read_last_clip(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data = peripheral::read_result(self.addr, "clip")?;
        peripheral::decode(&data)
    }

    /// clip (エンティティ)。
    pub fn book_next_clip_entity(&mut self) {
        peripheral::book_request(self.addr, "clipEntity", &msgpack::array(&[]));
    }
    pub fn read_last_clip_entity(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data = peripheral::read_result(self.addr, "clipEntity")?;
        peripheral::decode(&data)
    }

    /// clip (ブロック)。
    pub fn book_next_clip_block(&mut self) {
        peripheral::book_request(self.addr, "clipBlock", &msgpack::array(&[]));
    }
    pub fn read_last_clip_block(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data = peripheral::read_result(self.addr, "clipBlock")?;
        peripheral::decode(&data)
    }

    /// clip (全エンティティ)。
    pub fn book_next_clip_all_entity(&mut self) {
        peripheral::book_request(self.addr, "clipAllEntity", &msgpack::array(&[]));
    }
    pub fn read_last_clip_all_entity(&self) -> Result<Vec<CTLRaycastResult>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "clipAllEntity")?;
        peripheral::decode(&data)
    }

    /// clip (シップ)。
    pub fn book_next_clip_ship(&mut self) {
        peripheral::book_request(self.addr, "clipShip", &msgpack::array(&[]));
    }
    pub fn read_last_clip_ship(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data = peripheral::read_result(self.addr, "clipShip")?;
        peripheral::decode(&data)
    }

    /// clip (プレイヤー)。
    pub fn book_next_clip_player(&mut self) {
        peripheral::book_request(self.addr, "clipPlayer", &msgpack::array(&[]));
    }
    pub fn read_last_clip_player(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data = peripheral::read_result(self.addr, "clipPlayer")?;
        peripheral::decode(&data)
    }

    // ====== 状態変更系 ======

    /// ピッチを設定する。
    pub fn book_next_set_pitch(&mut self, degrees: f64) {
        let args = msgpack::array(&[msgpack::float64(degrees)]);
        peripheral::book_action(self.addr, "setPitch", &args);
    }
    pub fn read_last_set_pitch(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setPitch")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ヨーを設定する。
    pub fn book_next_set_yaw(&mut self, degrees: f64) {
        let args = msgpack::array(&[msgpack::float64(degrees)]);
        peripheral::book_action(self.addr, "setYaw", &args);
    }
    pub fn read_last_set_yaw(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setYaw")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// アウトラインをユーザーに表示する。
    pub fn book_next_outline_to_user(&mut self) {
        peripheral::book_action(self.addr, "outlineToUser", &msgpack::array(&[]));
    }
    pub fn read_last_outline_to_user(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "outlineToUser")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ピッチとヨーを強制設定する。
    pub fn book_next_force_pitch_yaw(&mut self, pitch: f64, yaw: f64) {
        let args = msgpack::array(&[msgpack::float64(pitch), msgpack::float64(yaw)]);
        peripheral::book_action(self.addr, "forcePitchYaw", &args);
    }
    pub fn read_last_force_pitch_yaw(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "forcePitchYaw")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// クリップ範囲を設定する。
    pub fn book_next_set_clip_range(&mut self, range: f64) {
        let args = msgpack::array(&[msgpack::float64(range)]);
        peripheral::book_action(self.addr, "setClipRange", &args);
    }
    pub fn read_last_set_clip_range(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setClipRange")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// コーン角度を設定する。
    pub fn book_next_set_cone_angle(&mut self, angle: f64) {
        let args = msgpack::array(&[msgpack::float64(angle)]);
        peripheral::book_action(self.addr, "setConeAngle", &args);
    }
    pub fn read_last_set_cone_angle(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setConeAngle")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// レイキャスト (指定座標)。
    pub fn book_next_raycast(&mut self, x: f64, y: f64, z: f64) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_request(self.addr, "raycast", &args);
    }
    pub fn read_last_raycast(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data = peripheral::read_result(self.addr, "raycast")?;
        peripheral::decode(&data)
    }

    /// 範囲内のエンティティを取得する。
    pub fn book_next_get_entities(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "getEntities", &args);
    }
    pub fn read_last_get_entities(&self) -> Result<Vec<crate::msgpack::Value>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getEntities")?;
        peripheral::decode(&data)
    }

    /// 範囲内のモブを取得する。
    pub fn book_next_get_mobs(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "getMobs", &args);
    }
    pub fn read_last_get_mobs(&self) -> Result<Vec<crate::msgpack::Value>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMobs")?;
        peripheral::decode(&data)
    }

    /// リセットする。
    pub fn book_next_reset(&mut self) {
        peripheral::book_action(self.addr, "reset", &msgpack::array(&[]));
    }
    pub fn read_last_reset(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "reset")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
