//! Create Additions ElectricMotor。

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ElectricMotor ペリフェラル。
pub struct ElectricMotor {
    addr: PeriphAddr,
}

impl Peripheral for ElectricMotor {
    const NAME: &'static str = "createaddition:electric_motor";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl ElectricMotor {
    /// ペリフェラルタイプを取得する (imm 対応)。
    pub fn book_next_get_type(&mut self) {
        peripheral::book_request(self.addr, "getType", &msgpack::array(&[]));
    }
    pub fn read_last_get_type(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getType")?;
        peripheral::decode(&data)
    }

    pub fn get_type_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getType",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// RPM を設定する (符号で方向制御)。
    pub fn book_next_set_speed(&mut self, speed: f64) {
        let args = msgpack::array(&[msgpack::float64(speed)]);
        peripheral::book_action(self.addr, "setSpeed", &args);
    }
    pub fn read_last_set_speed(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setSpeed")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 停止する。
    pub fn book_next_stop(&mut self) {
        peripheral::book_action(self.addr, "stop", &msgpack::array(&[]));
    }
    pub fn read_last_stop(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "stop")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 現在の速度を取得する。
    pub fn book_next_get_speed(&mut self) {
        peripheral::book_request(self.addr, "getSpeed", &msgpack::array(&[]));
    }
    pub fn read_last_get_speed(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSpeed")?;
        peripheral::decode(&data)
    }

    /// ストレス容量を取得する。
    pub fn book_next_get_stress_capacity(&mut self) {
        peripheral::book_request(self.addr, "getStressCapacity", &msgpack::array(&[]));
    }
    pub fn read_last_get_stress_capacity(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getStressCapacity")?;
        peripheral::decode(&data)
    }

    /// エネルギー消費量を取得する。
    pub fn book_next_get_energy_consumption(&mut self) {
        peripheral::book_request(self.addr, "getEnergyConsumption", &msgpack::array(&[]));
    }
    pub fn read_last_get_energy_consumption(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getEnergyConsumption")?;
        peripheral::decode(&data)
    }

    /// 指定角度回転する。所要秒数を返す。
    pub fn book_next_rotate(&mut self, degrees: f64, rpm: Option<f64>) {
        let mut args = alloc::vec![msgpack::float64(degrees)];
        if let Some(r) = rpm {
            args.push(msgpack::float64(r));
        }
        peripheral::book_action(self.addr, "rotate", &msgpack::array(&args));
    }
    pub fn read_last_rotate(&self) -> Vec<Result<f64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "rotate")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定距離移動する。所要秒数を返す。
    pub fn book_next_translate(&mut self, distance: f64, rpm: Option<f64>) {
        let mut args = alloc::vec![msgpack::float64(distance)];
        if let Some(r) = rpm {
            args.push(msgpack::float64(r));
        }
        peripheral::book_action(self.addr, "translate", &msgpack::array(&args));
    }
    pub fn read_last_translate(&self) -> Vec<Result<f64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "translate")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 最大挿入エネルギーを取得する。
    pub fn book_next_get_max_insert(&mut self) {
        peripheral::book_request(self.addr, "getMaxInsert", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_insert(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxInsert")?;
        peripheral::decode(&data)
    }

    /// 最大抽出エネルギーを取得する。
    pub fn book_next_get_max_extract(&mut self) {
        peripheral::book_request(self.addr, "getMaxExtract", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_extract(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxExtract")?;
        peripheral::decode(&data)
    }
}
