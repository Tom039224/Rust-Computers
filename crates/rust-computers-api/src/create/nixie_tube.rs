//! Create Nixie Tube ペリフェラル。
//! Create Nixie Tube peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::common::CRSignalParams;

/// Nixie Tube ペリフェラル。
pub struct NixieTube {
    addr: PeriphAddr,
}

impl Peripheral for NixieTube {
    const NAME: &'static str = "create:nixie_tube";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl NixieTube {
    /// テキストを設定する。オプションでカラーを指定可能。
    pub async fn set_text(
        &self,
        text: &str,
        colour: Option<&str>,
    ) -> Result<(), PeripheralError> {
        let mut args_vec = alloc::vec![msgpack::str(text)];
        if let Some(c) = colour {
            args_vec.push(msgpack::str(c));
        }
        let args = msgpack::array(&args_vec);
        peripheral::do_action(self.addr, "setText", &args).await?;
        Ok(())
    }

    /// テキストカラーを設定する。
    pub async fn set_text_colour(&self, colour: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(colour)]);
        peripheral::do_action(self.addr, "setTextColour", &args).await?;
        Ok(())
    }

    /// シグナル表示を設定する。front は必須、back はオプション。
    pub async fn set_signal(
        &self,
        front: &CRSignalParams,
        back: Option<&CRSignalParams>,
    ) -> Result<(), PeripheralError> {
        let front_encoded = peripheral::encode(front)?;
        let mut args_vec = alloc::vec![front_encoded];
        if let Some(b) = back {
            args_vec.push(peripheral::encode(b)?);
        }
        let args = msgpack::array(&args_vec);
        peripheral::do_action(self.addr, "setSignal", &args).await?;
        Ok(())
    }
}
