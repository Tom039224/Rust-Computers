//! CC-VS Aerodynamics API。
//! CC-VS Aerodynamics global API.

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// 大気パラメータ。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VSAtmosphericParameters {
    pub max_y: f64,
    pub sea_level: f64,
    pub gravity: f64,
}

/// Aerodynamics グローバル API。
/// Global aerodynamics API (not direction-specific, uses a fixed direction).
pub struct Aerodynamics {
    dir: Direction,
}

impl Peripheral for Aerodynamics {
    const NAME: &'static str = "vs_aerodynamics";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

macro_rules! imm_only {
    ($fn_name:ident, $method:literal, $ret:ty) => {
        pub fn $fn_name(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.dir, $method, &msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl Aerodynamics {
    // ====== プロパティ系 (imm のみ) ======

    imm_only!(default_max_imm, "defaultMax", f64);
    imm_only!(default_sea_level_imm, "defaultSeaLevel", f64);
    imm_only!(drag_coefficient_imm, "dragCoefficient", f64);
    imm_only!(gravitational_acceleration_imm, "gravitationalAcceleration", f64);
    imm_only!(universal_gas_constant_imm, "universalGasConstant", f64);
    imm_only!(air_molar_mass_imm, "airMolarMass", f64);

    // ====== 関数メソッド (imm 対応) ======

    /// 大気パラメータを取得する (imm)。
    pub async fn get_atmospheric_parameters(
        &self,
    ) -> Result<Option<VSAtmosphericParameters>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getAtmosphericParameters",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_atmospheric_parameters_imm(
        &self,
    ) -> Result<Option<VSAtmosphericParameters>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getAtmosphericParameters",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 指定 Y 座標の空気密度 (imm)。
    pub async fn get_air_density(
        &self,
        y: Option<f64>,
    ) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data =
            peripheral::request_info(self.dir, "getAirDensity", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_air_density_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data = peripheral::request_info_imm(self.dir, "getAirDensity", &args)?;
        peripheral::decode(&data)
    }

    /// 指定 Y 座標の大気圧 (imm)。
    pub async fn get_air_pressure(
        &self,
        y: Option<f64>,
    ) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data =
            peripheral::request_info(self.dir, "getAirPressure", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_air_pressure_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data = peripheral::request_info_imm(self.dir, "getAirPressure", &args)?;
        peripheral::decode(&data)
    }

    /// 指定 Y 座標の気温 (imm)。
    pub async fn get_air_temperature(
        &self,
        y: Option<f64>,
    ) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data =
            peripheral::request_info(self.dir, "getAirTemperature", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_air_temperature_imm(
        &self,
        y: Option<f64>,
    ) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data = peripheral::request_info_imm(self.dir, "getAirTemperature", &args)?;
        peripheral::decode(&data)
    }
}
