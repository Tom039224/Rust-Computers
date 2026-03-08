//! Create Creative Motor ペリフェラル。
//! Create Creative Motor peripheral.

use crate::error::PeripheralError;
use alloc::vec::Vec;
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
    pub fn book_next_set_generated_speed(&mut self, speed: i32) {
        let args = msgpack::array(&[msgpack::int(speed)]);
        peripheral::book_action(self.addr, "setGeneratedSpeed", &args);
    }

    pub fn read_last_set_generated_speed(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setGeneratedSpeed")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 現在の生成回転速度を取得する。
    pub fn book_next_get_generated_speed(&mut self) {
        peripheral::book_request(
            self.addr,
            "getGeneratedSpeed",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_generated_speed(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getGeneratedSpeed")?;
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
