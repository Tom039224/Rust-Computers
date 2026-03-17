//! Clockwork CC Compat GasPump。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// GasPump ペリフェラル。
pub struct GasPump {
    addr: PeriphAddr,
}

impl Peripheral for GasPump {
    const NAME: &'static str = "cw_gas_pump";

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

impl GasPump {
    // ---- 固有メソッド ----

    imm_getter!(book_next_get_pump_pressure, read_last_get_pump_pressure, get_pump_pressure_imm, "getPumpPressure", f64);
    imm_getter!(book_next_get_speed, read_last_get_speed, get_speed_imm, "getSpeed", f64);
    imm_getter!(book_next_get_facing, read_last_get_facing, get_facing_imm, "getFacing", alloc::string::String);

    // ---- GasNetwork 共通メソッド ----

    super::gas_network::gas_network_methods!();
}

impl GasPump {
    pub async fn async_get_pump_pressure(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_pump_pressure();
        crate::wait_for_next_tick().await;
        self.read_last_get_pump_pressure()
    }

    pub async fn async_get_speed(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_speed();
        crate::wait_for_next_tick().await;
        self.read_last_get_speed()
    }

    pub async fn async_get_facing(&mut self) -> Result<alloc::string::String, PeripheralError> {
        self.book_next_get_facing();
        crate::wait_for_next_tick().await;
        self.read_last_get_facing()
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
