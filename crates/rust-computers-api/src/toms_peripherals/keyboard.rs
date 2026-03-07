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
    pub async fn set_fire_native_events(
        &self,
        enabled: bool,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::do_action(self.addr, "setFireNativeEvents", &args).await?;
        Ok(())
    }
}
