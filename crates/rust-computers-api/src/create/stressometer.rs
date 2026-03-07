//! Create Stressometer ペリフェラル。
//! Create Stressometer peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Stressometer ペリフェラル。
pub struct Stressometer {
    addr: PeriphAddr,
}

impl Peripheral for Stressometer {
    const NAME: &'static str = "create:stressometer";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Stressometer {
    /// 現在のストレス値を取得する。
    pub async fn get_stress(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getStress",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 現在のストレス値を即時取得する (imm 対応)。
    pub fn get_stress_imm(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getStress",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ストレス容量を取得する。
    pub async fn get_stress_capacity(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getStressCapacity",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// ストレス容量を即時取得する (imm 対応)。
    pub fn get_stress_capacity_imm(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getStressCapacity",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 過負荷イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_overstressed(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "try_pull_overstressed",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 過負荷イベントを受信するまで待機する。
    pub async fn pull_overstressed(&self) -> Result<(), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_overstressed().await? {
                return Ok(v);
            }
        }
    }

    /// ストレス変化イベントを 1tick 待機して取得する。来なければ None。
    /// 戻り値は (stress, capacity) のタプル。
    pub async fn try_pull_stress_change(
        &self,
    ) -> Result<Option<(f32, f32)>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "try_pull_stress_change",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// ストレス変化イベントを受信するまで待機する。
    /// 戻り値は (stress, capacity) のタプル。
    pub async fn pull_stress_change(&self) -> Result<(f32, f32), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_stress_change().await? {
                return Ok(v);
            }
        }
    }
}
