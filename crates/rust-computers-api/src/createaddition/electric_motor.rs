//! Create Additions ElectricMotor。

use alloc::string::String;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// ElectricMotor ペリフェラル。
pub struct ElectricMotor {
    dir: Direction,
}

impl Peripheral for ElectricMotor {
    const NAME: &'static str = "createaddition:electric_motor";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl ElectricMotor {
    /// ペリフェラルタイプを取得する (imm 対応)。
    pub async fn get_type(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getType",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_type_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getType",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// RPM を設定する (符号で方向制御)。
    pub async fn set_speed(&self, speed: f64) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(speed)]);
        peripheral::do_action(self.dir, "setSpeed", &args).await?;
        Ok(())
    }

    /// 停止する。
    pub async fn stop(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "stop", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// 現在の速度を取得する。
    pub async fn get_speed(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getSpeed",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// ストレス容量を取得する。
    pub async fn get_stress_capacity(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getStressCapacity",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// エネルギー消費量を取得する。
    pub async fn get_energy_consumption(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getEnergyConsumption",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 指定角度回転する。所要秒数を返す。
    pub async fn rotate(
        &self,
        degrees: f64,
        rpm: Option<f64>,
    ) -> Result<f64, PeripheralError> {
        let mut args = alloc::vec![msgpack::float64(degrees)];
        if let Some(r) = rpm {
            args.push(msgpack::float64(r));
        }
        let data =
            peripheral::do_action(self.dir, "rotate", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }

    /// 指定距離移動する。所要秒数を返す。
    pub async fn translate(
        &self,
        distance: f64,
        rpm: Option<f64>,
    ) -> Result<f64, PeripheralError> {
        let mut args = alloc::vec![msgpack::float64(distance)];
        if let Some(r) = rpm {
            args.push(msgpack::float64(r));
        }
        let data =
            peripheral::do_action(self.dir, "translate", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }

    /// 最大挿入エネルギーを取得する。
    pub async fn get_max_insert(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getMaxInsert",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 最大抽出エネルギーを取得する。
    pub async fn get_max_extract(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getMaxExtract",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }
}
