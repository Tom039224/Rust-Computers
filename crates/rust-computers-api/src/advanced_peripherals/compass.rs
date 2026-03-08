//! AdvancedPeripherals Compass。

use alloc::string::String;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Compass ペリフェラル（タートル専用）。
pub struct Compass {
    addr: PeriphAddr,
}

impl Peripheral for Compass {
    const NAME: &'static str = "advancedPeripherals:compass";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Compass {
    /// タートルの向いている方向を返す ("north", "south", "east", "west")。
    pub fn book_next_get_facing(&mut self) {
        peripheral::book_request(self.addr, "getFacing", &msgpack::array(&[]));
    }
    pub fn read_last_get_facing(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getFacing")?;
        peripheral::decode(&data)
    }
}
