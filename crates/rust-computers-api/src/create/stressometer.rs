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
    pub fn book_next_get_stress(&mut self) {
        peripheral::book_request(
            self.addr,
            "getStress",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_stress(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getStress")?;
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
    pub fn book_next_get_stress_capacity(&mut self) {
        peripheral::book_request(
            self.addr,
            "getStressCapacity",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_stress_capacity(&self) -> Result<f32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getStressCapacity")?;
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
    pub fn book_next_try_pull_overstressed(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_overstressed",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_overstressed(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_overstressed")?;
        peripheral::decode(&data)
    }

    /// 過負荷イベントを受信するまで待機する。
    pub async fn pull_overstressed(&self) -> Result<(), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_overstressed", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_overstressed")?;
            let result: Option<()> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }

    /// ストレス変化イベントを 1tick 待機して取得する。来なければ None。
    /// 戻り値は (stress, capacity) のタプル。
    pub fn book_next_try_pull_stress_change(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_stress_change",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_stress_change(&self) -> Result<Option<(f32, f32)>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_stress_change")?;
        peripheral::decode(&data)
    }

    /// ストレス変化イベントを受信するまで待機する。
    /// 戻り値は (stress, capacity) のタプル。
    pub async fn pull_stress_change(&self) -> Result<(f32, f32), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_stress_change", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_stress_change")?;
            let result: Option<(f32, f32)> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }
}
