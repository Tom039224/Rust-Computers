//! CC-VS Ship API。
//! CC-VS Ship peripheral for Valkyrien Skies integration.

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

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
    addr: PeriphAddr,
}

impl Peripheral for Ship {
    const NAME: &'static str = "ship";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

// ============================================================
// Helper macros for repetitive book/read/imm getter patterns
// ============================================================

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

impl Ship {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(book_next_get_id, read_last_get_id, get_id_imm, "getId", i64);
    book_read_imm!(book_next_get_mass, read_last_get_mass, get_mass_imm, "getMass", f64);
    book_read_imm!(
        book_next_get_moment_of_inertia_tensor,
        read_last_get_moment_of_inertia_tensor,
        get_moment_of_inertia_tensor_imm,
        "getMomentOfInertiaTensor",
        [[f64; 3]; 3]
    );
    book_read_imm!(book_next_get_slug, read_last_get_slug, get_slug_imm, "getSlug", String);
    book_read_imm!(
        book_next_get_angular_velocity,
        read_last_get_angular_velocity,
        get_angular_velocity_imm,
        "getAngularVelocity",
        VSVector3
    );
    book_read_imm!(book_next_get_quaternion, read_last_get_quaternion, get_quaternion_imm, "getQuaternion", VSQuaternion);
    book_read_imm!(book_next_get_scale, read_last_get_scale, get_scale_imm, "getScale", VSVector3);
    book_read_imm!(
        book_next_get_shipyard_position,
        read_last_get_shipyard_position,
        get_shipyard_position_imm,
        "getShipyardPosition",
        VSVector3
    );
    book_read_imm!(book_next_get_size, read_last_get_size, get_size_imm, "getSize", VSVector3);
    book_read_imm!(book_next_get_velocity, read_last_get_velocity, get_velocity_imm, "getVelocity", VSVector3);
    book_read_imm!(
        book_next_get_worldspace_position,
        read_last_get_worldspace_position,
        get_worldspace_position_imm,
        "getWorldspacePosition",
        VSVector3
    );
    book_read_imm!(book_next_is_static, read_last_is_static, is_static_imm, "isStatic", bool);
    book_read_imm!(
        book_next_get_transformation_matrix,
        read_last_get_transformation_matrix,
        get_transformation_matrix_imm,
        "getTransformationMatrix",
        VSTransformMatrix
    );
    book_read_imm!(book_next_get_joints, read_last_get_joints, get_joints_imm, "getJoints", Vec<VSJoint>);

    /// ローカル座標をワールド座標に変換する (book/read)。
    pub fn book_next_transform_position_to_world(&mut self, pos: VSVector3) {
        let args = msgpack::array(&[
            msgpack::float64(pos.x),
            msgpack::float64(pos.y),
            msgpack::float64(pos.z),
        ]);
        peripheral::book_request(self.addr, "transformPositionToWorld", &args);
    }

    pub fn read_last_transform_position_to_world(&self) -> Result<VSVector3, PeripheralError> {
        let data = peripheral::read_result(self.addr, "transformPositionToWorld")?;
        peripheral::decode(&data)
    }

    /// ローカル座標をワールド座標に変換する (imm)。
    pub fn transform_position_to_world_imm(
        &self,
        pos: VSVector3,
    ) -> Result<VSVector3, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(pos.x),
            msgpack::float64(pos.y),
            msgpack::float64(pos.z),
        ]);
        let data = peripheral::request_info_imm(self.addr, "transformPositionToWorld", &args)?;
        peripheral::decode(&data)
    }

    // ====== 状態変更系 (allow_op) ======

    /// スラグ名を設定する。
    pub fn book_next_set_slug(&mut self, name: &str) {
        let args = msgpack::array(&[msgpack::str(name)]);
        peripheral::book_action(self.addr, "setSlug", &args);
    }

    pub fn read_last_set_slug(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setSlug")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 静的状態を設定する。
    pub fn book_next_set_static(&mut self, is_static: bool) {
        let args = msgpack::array(&[msgpack::bool_val(is_static)]);
        peripheral::book_action(self.addr, "setStatic", &args);
    }

    pub fn read_last_set_static(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setStatic")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// スケールを設定する。
    pub fn book_next_set_scale_value(&mut self, scale: f64) {
        let args = msgpack::array(&[msgpack::float64(scale)]);
        peripheral::book_action(self.addr, "setScale", &args);
    }

    pub fn read_last_set_scale_value(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setScale")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// テレポートする。
    pub fn book_next_teleport(&mut self, data: &VSTeleportData) -> Result<(), PeripheralError> {
        let args = peripheral::encode(data)?;
        peripheral::book_action(self.addr, "teleport", &args);
        Ok(())
    }

    pub fn read_last_teleport(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "teleport")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    // ====== 力の印加系 (allow_op) ======

    /// ワールド座標系で力を印加する。
    pub fn book_next_apply_world_force(
        &mut self,
        fx: f64,
        fy: f64,
        fz: f64,
        pos: Option<VSVector3>,
    ) {
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
        peripheral::book_action(self.addr, "applyWorldForce", &msgpack::array(&args));
    }

    pub fn read_last_apply_world_force(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyWorldForce")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ワールド座標系でトルクを印加する。
    pub fn book_next_apply_world_torque(
        &mut self,
        tx: f64,
        ty: f64,
        tz: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(tx),
            msgpack::float64(ty),
            msgpack::float64(tz),
        ]);
        peripheral::book_action(self.addr, "applyWorldTorque", &args);
    }

    pub fn read_last_apply_world_torque(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyWorldTorque")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// モデル座標系で力を印加する。
    pub fn book_next_apply_model_force(
        &mut self,
        fx: f64,
        fy: f64,
        fz: f64,
        pos: Option<VSVector3>,
    ) {
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
        peripheral::book_action(self.addr, "applyModelForce", &msgpack::array(&args));
    }

    pub fn read_last_apply_model_force(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyModelForce")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// モデル座標系でトルクを印加する。
    pub fn book_next_apply_model_torque(
        &mut self,
        tx: f64,
        ty: f64,
        tz: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(tx),
            msgpack::float64(ty),
            msgpack::float64(tz),
        ]);
        peripheral::book_action(self.addr, "applyModelTorque", &args);
    }

    pub fn read_last_apply_model_torque(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyModelTorque")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ワールド力をモデル座標系位置に対して印加する。
    pub fn book_next_apply_world_force_to_model_pos(
        &mut self,
        fx: f64,
        fy: f64,
        fz: f64,
        px: f64,
        py: f64,
        pz: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(fx),
            msgpack::float64(fy),
            msgpack::float64(fz),
            msgpack::float64(px),
            msgpack::float64(py),
            msgpack::float64(pz),
        ]);
        peripheral::book_action(self.addr, "applyWorldForceToModelPos", &args);
    }

    pub fn read_last_apply_world_force_to_model_pos(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyWorldForceToModelPos")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ボディ座標系で力を印加する。
    pub fn book_next_apply_body_force(
        &mut self,
        fx: f64,
        fy: f64,
        fz: f64,
        pos: Option<VSVector3>,
    ) {
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
        peripheral::book_action(self.addr, "applyBodyForce", &msgpack::array(&args));
    }

    pub fn read_last_apply_body_force(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyBodyForce")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ボディ座標系でトルクを印加する。
    pub fn book_next_apply_body_torque(
        &mut self,
        tx: f64,
        ty: f64,
        tz: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(tx),
            msgpack::float64(ty),
            msgpack::float64(tz),
        ]);
        peripheral::book_action(self.addr, "applyBodyTorque", &args);
    }

    pub fn read_last_apply_body_torque(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyBodyTorque")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ワールド力をボディ座標系位置に対して印加する。
    pub fn book_next_apply_world_force_to_body_pos(
        &mut self,
        fx: f64,
        fy: f64,
        fz: f64,
        px: f64,
        py: f64,
        pz: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(fx),
            msgpack::float64(fy),
            msgpack::float64(fz),
            msgpack::float64(px),
            msgpack::float64(py),
            msgpack::float64(pz),
        ]);
        peripheral::book_action(self.addr, "applyWorldForceToBodyPos", &args);
    }

    pub fn read_last_apply_world_force_to_body_pos(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyWorldForceToBodyPos")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    // ====== イベント系 ======

    /// 1tick 待機して物理ティックイベントを受信する (book/read)。
    pub fn book_next_try_pull_physics_ticks(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_physics_ticks",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_physics_ticks(
        &self,
    ) -> Result<Option<VSPhysicsTickData>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_physics_ticks")?;
        peripheral::decode(&data)
    }

    /// 物理ティックイベントを受信するまで待機する。
    pub async fn pull_physics_ticks(&self) -> Result<VSPhysicsTickData, PeripheralError> {
        loop {
            peripheral::book_request(
                self.addr,
                "try_pull_physics_ticks",
                &crate::msgpack::array(&[]),
            );
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_physics_ticks")?;
            let result: Option<VSPhysicsTickData> = peripheral::decode(&data)?;
            if let Some(val) = result {
                return Ok(val);
            }
        }
    }
}
