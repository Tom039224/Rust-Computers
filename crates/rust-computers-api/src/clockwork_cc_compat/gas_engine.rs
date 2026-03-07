//! Clockwork CC Compat GasEngine。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// GasEngine ペリフェラル。
pub struct GasEngine {
    dir: Direction,
}

impl Peripheral for GasEngine {
    const NAME: &'static str = "clockwork:gas_engine";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl GasEngine {
    /// 接続エンジン数を取得する (imm 対応)。
    pub async fn get_attached_engines(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getAttachedEngines",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_attached_engines_imm(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getAttachedEngines",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 全体効率を取得する (imm 対応)。
    pub async fn get_total_efficiency(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getTotalEfficiency",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_total_efficiency_imm(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getTotalEfficiency",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
