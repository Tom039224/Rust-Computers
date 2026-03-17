//! Clockwork CC Compat DuctTank。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// DuctTank ペリフェラル。
pub struct DuctTank {
    addr: PeriphAddr,
}

impl Peripheral for DuctTank {
    const NAME: &'static str = "cw_duct_tank";

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

impl DuctTank {
    // ---- 固有メソッド ----

    imm_getter!(book_next_get_height, read_last_get_height, get_height_imm, "getHeight", f64);
    imm_getter!(book_next_get_width, read_last_get_width, get_width_imm, "getWidth", f64);

    // ---- GasNetwork 共通メソッド ----

    super::gas_network::gas_network_methods!();
}

impl DuctTank {
    pub async fn async_get_height(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_height();
        crate::wait_for_next_tick().await;
        self.read_last_get_height()
    }

    pub async fn async_get_width(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_width();
        crate::wait_for_next_tick().await;
        self.read_last_get_width()
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
