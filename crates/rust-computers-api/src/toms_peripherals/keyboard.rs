//! Toms-Peripherals Keyboard。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Keyboard ペリフェラル。
pub struct Keyboard {
    addr: PeriphAddr,
}

impl Peripheral for Keyboard {
    const NAME: &'static str = "tm_keyboard";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Keyboard {
    /// ネイティブイベント発火の有効/無効を設定する。
    pub fn book_next_set_fire_native_events(&mut self, enabled: bool) {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::book_action(self.addr, "setFireNativeEvents", &args);
    }

    pub fn read_last_set_fire_native_events(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setFireNativeEvents")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
