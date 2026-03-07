//! Create Rotation Speed Controller ペリフェラル。
//! Create Rotation Speed Controller peripheral.

use crate::error::PeripheralError;
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
    pub async fn set_target_speed(&self, speed: i32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(speed)]);
        peripheral::do_action(self.addr, "setTargetSpeed", &args).await?;
        Ok(())
    }

    /// 現在のターゲット速度を取得する。
    pub async fn get_target_speed(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getTargetSpeed",
            &msgpack::array(&[]),
        )
        .await?;
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
