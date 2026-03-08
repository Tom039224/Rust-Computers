//! Clockwork CC Compat GasPump。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// GasPump ペリフェラル。
pub struct GasPump {
    addr: PeriphAddr,
}

impl Peripheral for GasPump {
    const NAME: &'static str = "clockwork:gas_pump";

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
