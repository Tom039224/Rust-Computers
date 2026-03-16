//! Clockwork CC Compat GasThruster。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// GasThruster ペリフェラル。
pub struct GasThruster {
    addr: PeriphAddr,
}

impl Peripheral for GasThruster {
    const NAME: &'static str = "cw_gas_thruster";

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

impl GasThruster {
    // ---- 固有メソッド ----

    imm_getter!(book_next_get_thrust, read_last_get_thrust, get_thrust_imm, "getThrust", f64);
    imm_getter!(book_next_get_flow_rate, read_last_get_flow_rate, get_flow_rate_imm, "getFlowRate", f64);
    imm_getter!(book_next_get_gas_mass_flow, read_last_get_gas_mass_flow, get_gas_mass_flow_imm, "getGasMassFlow", alloc::collections::BTreeMap<alloc::string::String, f64>);
    imm_getter!(book_next_get_facing, read_last_get_facing, get_facing_imm, "getFacing", alloc::string::String);

    // ---- GasNetwork 共通メソッド ----

    super::gas_network::gas_network_methods!();
}
