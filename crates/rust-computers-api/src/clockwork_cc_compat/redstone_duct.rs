//! Clockwork CC Compat RedstoneDuct。

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// レッドストーンダクト条件情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalInfo {
    #[serde(rename = "moreThan")]
    pub more_than: bool,
    #[serde(rename = "comparisonValue")]
    pub comparison_value: f64,
    #[serde(rename = "filterBlacklist")]
    pub filter_blacklist: bool,
}

/// RedstoneDuct ペリフェラル。
pub struct RedstoneDuct {
    addr: PeriphAddr,
}

impl Peripheral for RedstoneDuct {
    const NAME: &'static str = "clockwork:redstone_duct";

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

impl RedstoneDuct {
    // ---- 固有メソッド ----

    imm_getter!(book_next_get_power, read_last_get_power, get_power_imm, "getPower", f64);
    imm_getter!(book_next_get_conditional, read_last_get_conditional, get_conditional_imm, "getConditional", ConditionalInfo);

    // ---- GasNetwork 共通メソッド ----

    super::gas_network::gas_network_methods!();
}
