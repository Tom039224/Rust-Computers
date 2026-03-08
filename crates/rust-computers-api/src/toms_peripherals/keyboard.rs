//! Toms-Peripherals Keyboard。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Keyboard ペリフェラル。
pub struct Keyboard {
    addr: PeriphAddr,
}

impl Peripheral for Keyboard {
    const NAME: &'static str = "tm:keyboard";

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

    pub fn read_last_set_fire_native_events(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setFireNativeEvents")?;
        Ok(())
    }
}
