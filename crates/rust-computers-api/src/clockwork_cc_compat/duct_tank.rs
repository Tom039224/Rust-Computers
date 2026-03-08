//! Clockwork CC Compat DuctTank。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// DuctTank ペリフェラル。
pub struct DuctTank {
    addr: PeriphAddr,
}

impl Peripheral for DuctTank {
    const NAME: &'static str = "clockwork:duct_tank";

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
