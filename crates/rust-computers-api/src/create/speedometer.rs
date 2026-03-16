//! Create Speedometer ペリフェラル。
//! Create Speedometer peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Speedometer ペリフェラル。
pub struct Speedometer {
    addr: PeriphAddr,
}

impl Peripheral for Speedometer {
    const NAME: &'static str = "Create_Speedometer";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Speedometer {
    /// 現在の回転速度を取得する。
    pub fn book_next_get_speed(&mut self) {
        peripheral::book_request(
            self.addr,
            "getSpeed",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_speed(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSpeed")?;
        peripheral::decode(&data)
    }

    /// 現在の回転速度を即時取得する (imm 対応)。
    pub fn get_speed_imm(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getSpeed",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 速度変化イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_speed_change(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_speed_change",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_speed_change(&self) -> Result<Option<f32>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_speed_change")?;
        peripheral::decode(&data)
    }

    /// 速度変化イベントを受信するまで待機する。
    pub async fn pull_speed_change(&self) -> Result<f32, PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_speed_change", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_speed_change")?;
            let result: Option<f32> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }
}
