//! Control-Craft Spinalyzer ペリフェラル。

use serde::{Deserialize, Serialize};
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// クォータニオン。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CTLQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

/// 3次元ベクトル。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CTLVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// スパイナライザー ペリフェラル。
pub struct Spinalyzer {
    addr: PeriphAddr,
}

impl Peripheral for Spinalyzer {
    const NAME: &'static str = "controlcraft:spinalyzer_peripheral";

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

impl Spinalyzer {
    // ====== 状態取得 (imm 対応) ======

    book_read_imm!(
        book_next_get_quaternion,
        read_last_get_quaternion,
        get_quaternion_imm,
        "getQuaternion",
        CTLQuaternion
    );
    book_read_imm!(
        book_next_get_quaternion_j,
        read_last_get_quaternion_j,
        get_quaternion_j_imm,
        "getQuaternionJ",
        CTLQuaternion
    );
    book_read_imm!(
        book_next_get_rotation_matrix,
        read_last_get_rotation_matrix,
        get_rotation_matrix_imm,
        "getRotationMatrix",
        [[f64; 3]; 3]
    );
    book_read_imm!(
        book_next_get_rotation_matrix_t,
        read_last_get_rotation_matrix_t,
        get_rotation_matrix_t_imm,
        "getRotationMatrixT",
        [[f64; 3]; 3]
    );
    book_read_imm!(
        book_next_get_velocity,
        read_last_get_velocity,
        get_velocity_imm,
        "getVelocity",
        CTLVec3
    );
    book_read_imm!(
        book_next_get_angular_velocity,
        read_last_get_angular_velocity,
        get_angular_velocity_imm,
        "getAngularVelocity",
        CTLVec3
    );
    book_read_imm!(
        book_next_get_position,
        read_last_get_position,
        get_position_imm,
        "getPosition",
        CTLVec3
    );
    book_read_imm!(
        book_next_get_spinalyzer_position,
        read_last_get_spinalyzer_position,
        get_spinalyzer_position_imm,
        "getSpinalyzerPosition",
        CTLVec3
    );
    book_read_imm!(
        book_next_get_spinalyzer_velocity,
        read_last_get_spinalyzer_velocity,
        get_spinalyzer_velocity_imm,
        "getSpinalyzerVelocity",
        CTLVec3
    );
    book_read_imm!(
        book_next_get_physics,
        read_last_get_physics,
        get_physics_imm,
        "getPhysics",
        crate::msgpack::Value
    );

    // ====== 力・トルク印加 ======

    /// ワールド空間固定方向の力を船の重心に印加する。
    pub fn book_next_apply_invariant_force(&mut self, x: f64, y: f64, z: f64) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_action(self.addr, "applyInvariantForce", &args);
    }
    pub fn read_last_apply_invariant_force(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyInvariantForce")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ワールド空間固定方向のトルクを印加する。
    pub fn book_next_apply_invariant_torque(&mut self, x: f64, y: f64, z: f64) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_action(self.addr, "applyInvariantTorque", &args);
    }
    pub fn read_last_apply_invariant_torque(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyInvariantTorque")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 船ローカル座標系の力を印加する（回転に追従）。
    pub fn book_next_apply_rot_dependent_force(&mut self, x: f64, y: f64, z: f64) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_action(self.addr, "applyRotDependentForce", &args);
    }
    pub fn read_last_apply_rot_dependent_force(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyRotDependentForce")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 船ローカル座標系のトルクを印加する（回転に追従）。
    pub fn book_next_apply_rot_dependent_torque(&mut self, x: f64, y: f64, z: f64) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_action(self.addr, "applyRotDependentTorque", &args);
    }
    pub fn read_last_apply_rot_dependent_torque(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyRotDependentTorque")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
