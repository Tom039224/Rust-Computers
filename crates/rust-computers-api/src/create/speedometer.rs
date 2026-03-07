//! Create Speedometer ペリフェラル。
//! Create Speedometer peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// Speedometer ペリフェラル。
pub struct Speedometer {
    dir: Direction,
}

impl Peripheral for Speedometer {
    const NAME: &'static str = "create:speedometer";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl Speedometer {
    /// 現在の回転速度を取得する。
    pub async fn get_speed(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getSpeed",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 現在の回転速度を即時取得する (imm 対応)。
    pub fn get_speed_imm(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getSpeed",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 速度変化イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_speed_change(&self) -> Result<Option<f32>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "try_pull_speed_change",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 速度変化イベントを受信するまで待機する。
    pub async fn pull_speed_change(&self) -> Result<f32, PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_speed_change().await? {
                return Ok(v);
            }
        }
    }
}
