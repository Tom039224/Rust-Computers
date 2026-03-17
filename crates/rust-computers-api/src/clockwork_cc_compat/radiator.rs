//! Clockwork CC Compat Radiator。

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ファン情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanInfo {
    pub r#type: alloc::string::String,
    pub rpm: f64,
    pub dir: alloc::string::String,
    pub dist: f64,
}

/// ガス変換情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionInfo {
    pub from: alloc::string::String,
    pub to: alloc::string::String,
    pub amount: f64,
}

/// Radiator ペリフェラル。
pub struct Radiator {
    addr: PeriphAddr,
}

impl Peripheral for Radiator {
    const NAME: &'static str = "cw_radiator";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

macro_rules! imm_getter {
    ($fn_book:ident, $fn_read:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub fn $fn_book(&mut self) {
            peripheral::book_request(self.addr, $method, &msgpack::array(&[]));
        }

        pub fn $fn_read(&self) -> Result<$ret, PeripheralError> {
            let data = peripheral::read_result(self.addr, $method)?;
            peripheral::decode(&data)
        }

        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.addr, $method, &msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl Radiator {
    // ---- 固有メソッド ----

    imm_getter!(book_next_get_fan_type, read_last_get_fan_type, get_fan_type_imm, "getFanType", alloc::string::String);
    imm_getter!(book_next_get_fan_rpm, read_last_get_fan_rpm, get_fan_rpm_imm, "getFanRPM", f64);
    imm_getter!(book_next_get_fan_count, read_last_get_fan_count, get_fan_count_imm, "getFanCount", f64);
    imm_getter!(book_next_get_fans, read_last_get_fans, get_fans_imm, "getFans", alloc::vec::Vec<FanInfo>);
    imm_getter!(book_next_is_active, read_last_is_active, is_active_imm, "isActive", bool);
    imm_getter!(book_next_is_cooling, read_last_is_cooling, is_cooling_imm, "isCooling", bool);
    imm_getter!(book_next_is_heating, read_last_is_heating, is_heating_imm, "isHeating", bool);
    imm_getter!(book_next_get_target_temp, read_last_get_target_temp, get_target_temp_imm, "getTargetTemp", f64);
    imm_getter!(book_next_get_input_temperature, read_last_get_input_temperature, get_input_temperature_imm, "getInputTemperature", f64);
    imm_getter!(book_next_get_output_temperature, read_last_get_output_temperature, get_output_temperature_imm, "getOutputTemperature", f64);
    imm_getter!(book_next_get_thermal_factor, read_last_get_thermal_factor, get_thermal_factor_imm, "getThermalFactor", f64);
    imm_getter!(book_next_get_atmospheric_pressure, read_last_get_atmospheric_pressure, get_atmospheric_pressure_imm, "getAtmosphericPressure", f64);
    imm_getter!(book_next_get_pressure_scale, read_last_get_pressure_scale, get_pressure_scale_imm, "getPressureScale", f64);
    imm_getter!(book_next_get_thermal_power, read_last_get_thermal_power, get_thermal_power_imm, "getThermalPower", f64);
    imm_getter!(book_next_get_status, read_last_get_status, get_status_imm, "getStatus", alloc::string::String);
    imm_getter!(book_next_get_conversion_rate, read_last_get_conversion_rate, get_conversion_rate_imm, "getConversionRate", f64);
    imm_getter!(book_next_get_conversions, read_last_get_conversions, get_conversions_imm, "getConversions", alloc::vec::Vec<ConversionInfo>);

    // ---- GasNetwork 共通メソッド ----

    super::gas_network::gas_network_methods!();
}

impl Radiator {
    pub async fn async_get_fan_type(&mut self) -> Result<alloc::string::String, PeripheralError> {
        self.book_next_get_fan_type();
        crate::wait_for_next_tick().await;
        self.read_last_get_fan_type()
    }

    pub async fn async_get_fan_rpm(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_fan_rpm();
        crate::wait_for_next_tick().await;
        self.read_last_get_fan_rpm()
    }

    pub async fn async_get_fan_count(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_fan_count();
        crate::wait_for_next_tick().await;
        self.read_last_get_fan_count()
    }

    pub async fn async_get_fans(&mut self) -> Result<alloc::vec::Vec<FanInfo>, PeripheralError> {
        self.book_next_get_fans();
        crate::wait_for_next_tick().await;
        self.read_last_get_fans()
    }

    pub async fn async_is_active(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_active();
        crate::wait_for_next_tick().await;
        self.read_last_is_active()
    }

    pub async fn async_is_cooling(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_cooling();
        crate::wait_for_next_tick().await;
        self.read_last_is_cooling()
    }

    pub async fn async_is_heating(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_heating();
        crate::wait_for_next_tick().await;
        self.read_last_is_heating()
    }

    pub async fn async_get_target_temp(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_target_temp();
        crate::wait_for_next_tick().await;
        self.read_last_get_target_temp()
    }

    pub async fn async_get_input_temperature(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_input_temperature();
        crate::wait_for_next_tick().await;
        self.read_last_get_input_temperature()
    }

    pub async fn async_get_output_temperature(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_output_temperature();
        crate::wait_for_next_tick().await;
        self.read_last_get_output_temperature()
    }

    pub async fn async_get_thermal_factor(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_thermal_factor();
        crate::wait_for_next_tick().await;
        self.read_last_get_thermal_factor()
    }

    pub async fn async_get_atmospheric_pressure(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_atmospheric_pressure();
        crate::wait_for_next_tick().await;
        self.read_last_get_atmospheric_pressure()
    }

    pub async fn async_get_pressure_scale(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_pressure_scale();
        crate::wait_for_next_tick().await;
        self.read_last_get_pressure_scale()
    }

    pub async fn async_get_thermal_power(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_thermal_power();
        crate::wait_for_next_tick().await;
        self.read_last_get_thermal_power()
    }

    pub async fn async_get_status(&mut self) -> Result<alloc::string::String, PeripheralError> {
        self.book_next_get_status();
        crate::wait_for_next_tick().await;
        self.read_last_get_status()
    }

    pub async fn async_get_conversion_rate(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_conversion_rate();
        crate::wait_for_next_tick().await;
        self.read_last_get_conversion_rate()
    }

    pub async fn async_get_conversions(&mut self) -> Result<alloc::vec::Vec<ConversionInfo>, PeripheralError> {
        self.book_next_get_conversions();
        crate::wait_for_next_tick().await;
        self.read_last_get_conversions()
    }

    pub async fn async_get_temperature(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_temperature();
        crate::wait_for_next_tick().await;
        self.read_last_get_temperature()
    }

    pub async fn async_get_pressure(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_pressure();
        crate::wait_for_next_tick().await;
        self.read_last_get_pressure()
    }

    pub async fn async_get_heat_energy(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_heat_energy();
        crate::wait_for_next_tick().await;
        self.read_last_get_heat_energy()
    }

    pub async fn async_get_gas_mass(&mut self) -> Result<alloc::collections::BTreeMap<alloc::string::String, f64>, PeripheralError> {
        self.book_next_get_gas_mass();
        crate::wait_for_next_tick().await;
        self.read_last_get_gas_mass()
    }

    pub async fn async_get_position(&mut self) -> Result<super::CLPosition, PeripheralError> {
        self.book_next_get_position();
        crate::wait_for_next_tick().await;
        self.read_last_get_position()
    }

    pub async fn async_get_network_info(&mut self) -> Result<crate::msgpack::Value, PeripheralError> {
        self.book_next_get_network_info();
        crate::wait_for_next_tick().await;
        self.read_last_get_network_info()
    }
}
