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
    const NAME: &'static str = "clockwork:radiator";

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
