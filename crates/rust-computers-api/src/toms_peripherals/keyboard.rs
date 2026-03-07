//! Toms-Peripherals Keyboard。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// Keyboard ペリフェラル。
pub struct Keyboard {
    dir: Direction,
}

impl Peripheral for Keyboard {
    const NAME: &'static str = "tm:keyboard";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl Keyboard {
    /// ネイティブイベント発火の有効/無効を設定する。
    pub async fn set_fire_native_events(
        &self,
        enabled: bool,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::do_action(self.dir, "setFireNativeEvents", &args).await?;
        Ok(())
    }
}
