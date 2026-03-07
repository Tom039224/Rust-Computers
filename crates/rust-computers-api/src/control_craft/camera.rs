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
    const NAME: &'static str = "controlcraft:camera";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

macro_rules! imm_getter {
    ($fn_async:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub async fn $fn_async(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info(self.addr, $method, &msgpack::array(&[])).await?;
            peripheral::decode(&data)
        }

        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.addr, $method, &msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl Camera {
    // ====== imm 対応読み取り系 ======

    imm_getter!(
        get_abs_view_transform,
        get_abs_view_transform_imm,
        "getAbsViewTransform",
        CTLTransform
    );
    imm_getter!(get_pitch, get_pitch_imm, "getPitch", f64);
    imm_getter!(get_yaw, get_yaw_imm, "getYaw", f64);
    imm_getter!(
        get_transformed_pitch,
        get_transformed_pitch_imm,
        "getTransformedPitch",
        f64
    );
    imm_getter!(
        get_transformed_yaw,
        get_transformed_yaw_imm,
        "getTransformedYaw",
        f64
    );
    imm_getter!(
        get_clip_distance,
        get_clip_distance_imm,
        "getClipDistance",
        f64
    );
    imm_getter!(
        latest_ship,
        latest_ship_imm,
        "latestShip",
        Option<crate::msgpack::Value>
    );
    imm_getter!(
        latest_player,
        latest_player_imm,
        "latestPlayer",
        Option<crate::msgpack::Value>
    );
    imm_getter!(
        latest_entity,
        latest_entity_imm,
        "latestEntity",
        Option<crate::msgpack::Value>
    );
    imm_getter!(
        latest_block,
        latest_block_imm,
        "latestBlock",
        Option<crate::msgpack::Value>
    );
    imm_getter!(
        get_camera_position,
        get_camera_position_imm,
        "getCameraPosition",
        (f64, f64, f64)
    );
    imm_getter!(
        get_abs_view_forward,
        get_abs_view_forward_imm,
        "getAbsViewForward",
        (f64, f64, f64)
    );
    imm_getter!(is_being_used, is_being_used_imm, "isBeingUsed", bool);
    imm_getter!(
        get_direction,
        get_direction_imm,
        "getDirection",
        String
    );

    // ====== クリップ系 ======

    /// clip (全体)。
    pub async fn clip(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data =
            peripheral::request_info(self.addr, "clip", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// clip (エンティティ)。
    pub async fn clip_entity(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data =
            peripheral::request_info(self.addr, "clipEntity", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// clip (ブロック)。
    pub async fn clip_block(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data =
            peripheral::request_info(self.addr, "clipBlock", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// clip (全エンティティ)。
    pub async fn clip_all_entity(&self) -> Result<Vec<CTLRaycastResult>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "clipAllEntity",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// clip (シップ)。
    pub async fn clip_ship(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data =
            peripheral::request_info(self.addr, "clipShip", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    /// clip (プレイヤー)。
    pub async fn clip_player(&self) -> Result<CTLRaycastResult, PeripheralError> {
        let data =
            peripheral::request_info(self.addr, "clipPlayer", &msgpack::array(&[])).await?;
        peripheral::decode(&data)
    }

    // ====== 状態変更系 ======

    /// ピッチを設定する。
    pub async fn set_pitch(&self, degrees: f64) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(degrees)]);
        peripheral::do_action(self.addr, "setPitch", &args).await?;
        Ok(())
    }

    /// ヨーを設定する。
    pub async fn set_yaw(&self, degrees: f64) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(degrees)]);
        peripheral::do_action(self.addr, "setYaw", &args).await?;
        Ok(())
    }

    /// アウトラインをユーザーに表示する。
    pub async fn outline_to_user(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.addr, "outlineToUser", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// ピッチとヨーを強制設定する。
    pub async fn force_pitch_yaw(
        &self,
        pitch: f64,
        yaw: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(pitch), msgpack::float64(yaw)]);
        peripheral::do_action(self.addr, "forcePitchYaw", &args).await?;
        Ok(())
    }

    /// クリップ範囲を設定する。
    pub async fn set_clip_range(&self, range: f64) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(range)]);
        peripheral::do_action(self.addr, "setClipRange", &args).await?;
        Ok(())
    }

    /// コーン角度を設定する。
    pub async fn set_cone_angle(&self, angle: f64) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(angle)]);
        peripheral::do_action(self.addr, "setConeAngle", &args).await?;
        Ok(())
    }

    /// レイキャスト (指定座標)。
    pub async fn raycast(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<CTLRaycastResult, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        let data = peripheral::request_info(self.addr, "raycast", &args).await?;
        peripheral::decode(&data)
    }

    /// 範囲内のエンティティを取得する。
    pub async fn get_entities(
        &self,
        radius: f64,
    ) -> Result<Vec<crate::msgpack::Value>, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data =
            peripheral::request_info(self.addr, "getEntities", &args).await?;
        peripheral::decode(&data)
    }

    /// 範囲内のモブを取得する。
    pub async fn get_mobs(
        &self,
        radius: f64,
    ) -> Result<Vec<crate::msgpack::Value>, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data = peripheral::request_info(self.addr, "getMobs", &args).await?;
        peripheral::decode(&data)
    }

    /// リセットする。
    pub async fn reset(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.addr, "reset", &msgpack::array(&[])).await?;
        Ok(())
    }
}
