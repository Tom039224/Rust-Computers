//! CC-VS Aerodynamics API。
//! CC-VS Aerodynamics global API.

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

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
    addr: PeriphAddr,
}

impl Peripheral for Aerodynamics {
    const NAME: &'static str = "vs_aerodynamics";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

macro_rules! imm_only {
    ($fn_name:ident, $method:literal, $ret:ty) => {
        pub fn $fn_name(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.addr, $method, &msgpack::array(&[]))?;
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

    // ====== 関数メソッド (book/read + imm 対応) ======

    /// 大気パラメータを取得する (book/read)。
    pub fn book_next_get_atmospheric_parameters(&mut self) {
        peripheral::book_request(
            self.addr,
            "getAtmosphericParameters",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_atmospheric_parameters(
        &self,
    ) -> Result<Option<VSAtmosphericParameters>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAtmosphericParameters")?;
        peripheral::decode(&data)
    }

    pub fn get_atmospheric_parameters_imm(
        &self,
    ) -> Result<Option<VSAtmosphericParameters>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getAtmosphericParameters",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 指定 Y 座標の空気密度 (book/read)。
    pub fn book_next_get_air_density(&mut self, y: Option<f64>) {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        peripheral::book_request(self.addr, "getAirDensity", &args);
    }

    pub fn read_last_get_air_density(&self) -> Result<Option<f64>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAirDensity")?;
        peripheral::decode(&data)
    }

    pub fn get_air_density_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data = peripheral::request_info_imm(self.addr, "getAirDensity", &args)?;
        peripheral::decode(&data)
    }

    /// 指定 Y 座標の大気圧 (book/read)。
    pub fn book_next_get_air_pressure(&mut self, y: Option<f64>) {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        peripheral::book_request(self.addr, "getAirPressure", &args);
    }

    pub fn read_last_get_air_pressure(&self) -> Result<Option<f64>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAirPressure")?;
        peripheral::decode(&data)
    }

    pub fn get_air_pressure_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        let data = peripheral::request_info_imm(self.addr, "getAirPressure", &args)?;
        peripheral::decode(&data)
    }

    /// 指定 Y 座標の気温 (book/read)。
    pub fn book_next_get_air_temperature(&mut self, y: Option<f64>) {
        let args = match y {
            Some(v) => msgpack::array(&[msgpack::float64(v)]),
            None => msgpack::array(&[]),
        };
        peripheral::book_request(self.addr, "getAirTemperature", &args);
    }

    pub fn read_last_get_air_temperature(&self) -> Result<Option<f64>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAirTemperature")?;
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
        let data = peripheral::request_info_imm(self.addr, "getAirTemperature", &args)?;
        peripheral::decode(&data)
    }
}

impl Aerodynamics {
    pub async fn async_get_atmospheric_parameters(&mut self) -> Result<Option<VSAtmosphericParameters>, PeripheralError> {
        self.book_next_get_atmospheric_parameters();
        crate::wait_for_next_tick().await;
        self.read_last_get_atmospheric_parameters()
    }

    pub async fn async_get_air_density(&mut self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> {
        self.book_next_get_air_density(y);
        crate::wait_for_next_tick().await;
        self.read_last_get_air_density()
    }

    pub async fn async_get_air_pressure(&mut self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> {
        self.book_next_get_air_pressure(y);
        crate::wait_for_next_tick().await;
        self.read_last_get_air_pressure()
    }

    pub async fn async_get_air_temperature(&mut self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> {
        self.book_next_get_air_temperature(y);
        crate::wait_for_next_tick().await;
        self.read_last_get_air_temperature()
    }
}
