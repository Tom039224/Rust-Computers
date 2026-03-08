//! Create Rotation Speed Controller ペリフェラル。
//! Create Rotation Speed Controller peripheral.

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Rotation Speed Controller ペリフェラル。
pub struct RotationSpeedController {
    addr: PeriphAddr,
}

impl Peripheral for RotationSpeedController {
    const NAME: &'static str = "create:rotation_speed_controller";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl RotationSpeedController {
    /// ターゲット速度を設定する。
    pub fn book_next_set_target_speed(&mut self, speed: i32) {
        let args = msgpack::array(&[msgpack::int(speed)]);
        peripheral::book_action(self.addr, "setTargetSpeed", &args);
    }

    pub fn read_last_set_target_speed(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTargetSpeed")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 現在のターゲット速度を取得する。
    pub fn book_next_get_target_speed(&mut self) {
        peripheral::book_request(
            self.addr,
            "getTargetSpeed",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_target_speed(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTargetSpeed")?;
        peripheral::decode(&data)
    }

    /// 現在のターゲット速度を即時取得する (imm 対応)。
    pub fn get_target_speed_imm(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getTargetSpeed",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
