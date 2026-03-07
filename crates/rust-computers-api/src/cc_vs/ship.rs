//! CC-VS Ship API。
//! CC-VS Ship peripheral for Valkyrien Skies integration.

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// 3D ベクトル。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VSVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// クォータニオン。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VSQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

/// 4x4 変換行列。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VSTransformMatrix {
    pub matrix: [[f64; 4]; 4],
}

/// ジョイント情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSJoint {
    pub id: u64,
    pub name: String,
}

/// 慣性情報。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VSInertiaInfo {
    pub moment_of_inertia: VSVector3,
    pub mass: f64,
}

/// 位置/速度情報。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VSPoseVelInfo {
    pub vel: VSVector3,
    pub omega: VSVector3,
    pub pos: VSVector3,
    pub rot: VSQuaternion,
}

/// 物理ティックデータ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSPhysicsTickData {
    pub buoyant_factor: f64,
    pub is_static: bool,
    pub do_fluid_drag: bool,
    pub inertia: VSInertiaInfo,
    pub pose_vel: VSPoseVelInfo,
    pub forces_inducers: Vec<String>,
}

/// テレポートデータ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSTeleportData {
    #[serde(default)]
    pub pos: Option<VSVector3>,
    #[serde(default)]
    pub rot: Option<VSQuaternion>,
    #[serde(default)]
    pub vel: Option<VSVector3>,
    #[serde(default)]
    pub omega: Option<VSVector3>,
    #[serde(default)]
    pub dimension: Option<String>,
    #[serde(default)]
    pub scale: Option<f64>,
}

/// Ship ペリフェラル。
/// Ship peripheral for controlling Valkyrien Skies ships.
pub struct Ship {
    dir: Direction,
}

impl Peripheral for Ship {
    const NAME: &'static str = "ship";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

// ============================================================
// Helper macros for repetitive imm/async getter patterns
// ============================================================

macro_rules! imm_getter {
    ($fn_async:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub async fn $fn_async(&self) -> Result<$ret, PeripheralError> {
            let data = peripheral::request_info(self.dir, $method, &msgpack::array(&[]))
                .await?;
            peripheral::decode(&data)
        }

        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.dir, $method, &msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl Ship {
    // ====== 読み取り系 (imm 対応) ======

    imm_getter!(get_id, get_id_imm, "getId", i64);
    imm_getter!(get_mass, get_mass_imm, "getMass", f64);
    imm_getter!(
        get_moment_of_inertia_tensor,
        get_moment_of_inertia_tensor_imm,
        "getMomentOfInertiaTensor",
        [[f64; 3]; 3]
    );
    imm_getter!(get_slug, get_slug_imm, "getSlug", String);
    imm_getter!(
        get_angular_velocity,
        get_angular_velocity_imm,
        "getAngularVelocity",
        VSVector3
    );
    imm_getter!(get_quaternion, get_quaternion_imm, "getQuaternion", VSQuaternion);
    imm_getter!(get_scale, get_scale_imm, "getScale", VSVector3);
    imm_getter!(
        get_shipyard_position,
        get_shipyard_position_imm,
        "getShipyardPosition",
        VSVector3
    );
    imm_getter!(get_size, get_size_imm, "getSize", VSVector3);
    imm_getter!(get_velocity, get_velocity_imm, "getVelocity", VSVector3);
    imm_getter!(
        get_worldspace_position,
        get_worldspace_position_imm,
        "getWorldspacePosition",
        VSVector3
    );
    imm_getter!(is_static, is_static_imm, "isStatic", bool);
    imm_getter!(
        get_transformation_matrix,
        get_transformation_matrix_imm,
        "getTransformationMatrix",
        VSTransformMatrix
    );
    imm_getter!(get_joints, get_joints_imm, "getJoints", Vec<VSJoint>);

    /// ローカル座標をワールド座標に変換する (imm 対応)。
    pub async fn transform_position_to_world(
        &self,
        pos: VSVector3,
    ) -> Result<VSVector3, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(pos.x),
            msgpack::float64(pos.y),
            msgpack::float64(pos.z),
        ]);
        let data = peripheral::request_info(self.dir, "transformPositionToWorld", &args).await?;
        peripheral::decode(&data)
    }

    pub fn transform_position_to_world_imm(
        &self,
        pos: VSVector3,
    ) -> Result<VSVector3, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(pos.x),
            msgpack::float64(pos.y),
            msgpack::float64(pos.z),
        ]);
        let data = peripheral::request_info_imm(self.dir, "transformPositionToWorld", &args)?;
        peripheral::decode(&data)
    }

    // ====== 状態変更系 (allow_op) ======

    /// スラグ名を設定する。
    pub async fn set_slug(&self, name: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(name)]);
        peripheral::do_action(self.dir, "setSlug", &args).await?;
        Ok(())
    }

    /// 静的状態を設定する。
    pub async fn set_static(&self, is_static: bool) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::bool_val(is_static)]);
        peripheral::do_action(self.dir, "setStatic", &args).await?;
        Ok(())
    }

    /// スケールを設定する。
    pub async fn set_scale_value(&self, scale: f64) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(scale)]);
        peripheral::do_action(self.dir, "setScale", &args).await?;
        Ok(())
    }

    /// テレポートする。
    pub async fn teleport(&self, data: &VSTeleportData) -> Result<(), PeripheralError> {
        let args = peripheral::encode(data)?;
        peripheral::do_action(self.dir, "teleport", &args).await?;
        Ok(())
    }

    // ====== 力の印加系 (allow_op) ======

    /// ワールド座標系で力を印加する。
    pub async fn apply_world_force(
        &self,
        fx: f64,
        fy: f64,
        fz: f64,
        pos: Option<VSVector3>,
    ) -> Result<(), PeripheralError> {
        let mut args = alloc::vec![
            msgpack::float64(fx),
            msgpack::float64(fy),
            msgpack::float64(fz),
        ];
        if let Some(p) = pos {
            args.push(msgpack::float64(p.x));
            args.push(msgpack::float64(p.y));
            args.push(msgpack::float64(p.z));
        }
        peripheral::do_action(self.dir, "applyWorldForce", &msgpack::array(&args)).await?;
        Ok(())
    }

    /// ワールド座標系でトルクを印加する。
    pub async fn apply_world_torque(
        &self,
        tx: f64,
        ty: f64,
        tz: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(tx),
            msgpack::float64(ty),
            msgpack::float64(tz),
        ]);
        peripheral::do_action(self.dir, "applyWorldTorque", &args).await?;
        Ok(())
    }

    /// モデル座標系で力を印加する。
    pub async fn apply_model_force(
        &self,
        fx: f64,
        fy: f64,
        fz: f64,
        pos: Option<VSVector3>,
    ) -> Result<(), PeripheralError> {
        let mut args = alloc::vec![
            msgpack::float64(fx),
            msgpack::float64(fy),
            msgpack::float64(fz),
        ];
        if let Some(p) = pos {
            args.push(msgpack::float64(p.x));
            args.push(msgpack::float64(p.y));
            args.push(msgpack::float64(p.z));
        }
        peripheral::do_action(self.dir, "applyModelForce", &msgpack::array(&args)).await?;
        Ok(())
    }

    /// モデル座標系でトルクを印加する。
    pub async fn apply_model_torque(
        &self,
        tx: f64,
        ty: f64,
        tz: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(tx),
            msgpack::float64(ty),
            msgpack::float64(tz),
        ]);
        peripheral::do_action(self.dir, "applyModelTorque", &args).await?;
        Ok(())
    }

    /// ワールド力をモデル座標系位置に対して印加する。
    pub async fn apply_world_force_to_model_pos(
        &self,
        fx: f64,
        fy: f64,
        fz: f64,
        px: f64,
        py: f64,
        pz: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(fx),
            msgpack::float64(fy),
            msgpack::float64(fz),
            msgpack::float64(px),
            msgpack::float64(py),
            msgpack::float64(pz),
        ]);
        peripheral::do_action(self.dir, "applyWorldForceToModelPos", &args).await?;
        Ok(())
    }

    /// ボディ座標系で力を印加する。
    pub async fn apply_body_force(
        &self,
        fx: f64,
        fy: f64,
        fz: f64,
        pos: Option<VSVector3>,
    ) -> Result<(), PeripheralError> {
        let mut args = alloc::vec![
            msgpack::float64(fx),
            msgpack::float64(fy),
            msgpack::float64(fz),
        ];
        if let Some(p) = pos {
            args.push(msgpack::float64(p.x));
            args.push(msgpack::float64(p.y));
            args.push(msgpack::float64(p.z));
        }
        peripheral::do_action(self.dir, "applyBodyForce", &msgpack::array(&args)).await?;
        Ok(())
    }

    /// ボディ座標系でトルクを印加する。
    pub async fn apply_body_torque(
        &self,
        tx: f64,
        ty: f64,
        tz: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(tx),
            msgpack::float64(ty),
            msgpack::float64(tz),
        ]);
        peripheral::do_action(self.dir, "applyBodyTorque", &args).await?;
        Ok(())
    }

    /// ワールド力をボディ座標系位置に対して印加する。
    pub async fn apply_world_force_to_body_pos(
        &self,
        fx: f64,
        fy: f64,
        fz: f64,
        px: f64,
        py: f64,
        pz: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(fx),
            msgpack::float64(fy),
            msgpack::float64(fz),
            msgpack::float64(px),
            msgpack::float64(py),
            msgpack::float64(pz),
        ]);
        peripheral::do_action(self.dir, "applyWorldForceToBodyPos", &args).await?;
        Ok(())
    }

    // ====== イベント系 ======

    /// 1tick 待機して物理ティックイベントを受信する。
    pub async fn try_pull_physics_ticks(
        &self,
    ) -> Result<Option<VSPhysicsTickData>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "try_pull_physics_ticks",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 物理ティックイベントを受信するまで待機する。
    pub async fn pull_physics_ticks(&self) -> Result<VSPhysicsTickData, PeripheralError> {
        loop {
            if let Some(data) = self.try_pull_physics_ticks().await? {
                return Ok(data);
            }
        }
    }
}
