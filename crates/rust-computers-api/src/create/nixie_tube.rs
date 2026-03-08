//! Create Nixie Tube ペリフェラル。
//! Create Nixie Tube peripheral.

use crate::error::PeripheralError;
use alloc::vec::Vec;
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
    pub fn book_next_set_text(
        &mut self,
        text: &str,
        colour: Option<&str>,
    ) {
        let mut args_vec = alloc::vec![msgpack::str(text)];
        if let Some(c) = colour {
            args_vec.push(msgpack::str(c));
        }
        let args = msgpack::array(&args_vec);
        peripheral::book_action(self.addr, "setText", &args);
    }

    pub fn read_last_set_text(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setText")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// テキストカラーを設定する。
    pub fn book_next_set_text_colour(&mut self, colour: &str) {
        let args = msgpack::array(&[msgpack::str(colour)]);
        peripheral::book_action(self.addr, "setTextColour", &args);
    }

    pub fn read_last_set_text_colour(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTextColour")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// シグナル表示を設定する。front は必須、back はオプション。
    pub fn book_next_set_signal(
        &mut self,
        front: &CRSignalParams,
        back: Option<&CRSignalParams>,
    ) -> Result<(), PeripheralError> {
        let front_encoded = peripheral::encode(front)?;
        let mut args_vec = alloc::vec![front_encoded];
        if let Some(b) = back {
            args_vec.push(peripheral::encode(b)?);
        }
        let args = msgpack::array(&args_vec);
        peripheral::book_action(self.addr, "setSignal", &args);
        Ok(())
    }

    pub fn read_last_set_signal(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setSignal")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
