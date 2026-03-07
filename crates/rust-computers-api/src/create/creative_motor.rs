//! Create Creative Motor ペリフェラル。
//! Create Creative Motor peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Creative Motor ペリフェラル。
pub struct CreativeMotor {
    addr: PeriphAddr,
}

impl Peripheral for CreativeMotor {
    const NAME: &'static str = "create:creative_motor";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl CreativeMotor {
    /// 生成する回転速度を設定する。
    pub async fn set_generated_speed(&self, speed: i32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(speed)]);
        peripheral::do_action(self.addr, "setGeneratedSpeed", &args).await?;
        Ok(())
    }

    /// 現在の生成回転速度を取得する。
    pub async fn get_generated_speed(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getGeneratedSpeed",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 現在の生成回転速度を即時取得する (imm 対応)。
    pub fn get_generated_speed_imm(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getGeneratedSpeed",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
