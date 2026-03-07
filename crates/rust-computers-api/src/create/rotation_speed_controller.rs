//! Create Rotation Speed Controller ペリフェラル。
//! Create Rotation Speed Controller peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// Rotation Speed Controller ペリフェラル。
pub struct RotationSpeedController {
    dir: Direction,
}

impl Peripheral for RotationSpeedController {
    const NAME: &'static str = "create:rotation_speed_controller";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl RotationSpeedController {
    /// ターゲット速度を設定する。
    pub async fn set_target_speed(&self, speed: i32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(speed)]);
        peripheral::do_action(self.dir, "setTargetSpeed", &args).await?;
        Ok(())
    }

    /// 現在のターゲット速度を取得する。
    pub async fn get_target_speed(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getTargetSpeed",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 現在のターゲット速度を即時取得する (imm 対応)。
    pub fn get_target_speed_imm(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getTargetSpeed",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
