//! Clockwork CC Compat CoalBurner。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// CoalBurner ペリフェラル。
pub struct CoalBurner {
    addr: PeriphAddr,
}

impl Peripheral for CoalBurner {
    const NAME: &'static str = "clockwork:coal_burner";

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

impl CoalBurner {
    // ---- 固有メソッド ----

    imm_getter!(book_next_get_fuel_ticks, read_last_get_fuel_ticks, get_fuel_ticks_imm, "getFuelTicks", f64);
    imm_getter!(book_next_get_max_burn_time, read_last_get_max_burn_time, get_max_burn_time_imm, "getMaxBurnTime", f64);
    imm_getter!(book_next_is_burning, read_last_is_burning, is_burning_imm, "isBurning", bool);

    // ---- GasNetwork 共通メソッド ----

    super::gas_network::gas_network_methods!();
}
