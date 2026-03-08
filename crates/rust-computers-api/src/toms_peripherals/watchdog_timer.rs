//! Toms-Peripherals WatchDogTimer。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// WatchDogTimer ペリフェラル。
pub struct WatchDogTimer {
    addr: PeriphAddr,
}

impl Peripheral for WatchDogTimer {
    const NAME: &'static str = "tm:watchdog_timer";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl WatchDogTimer {
    /// 有効かどうかを取得する (imm 対応)。
    pub fn book_next_is_enabled(&mut self) {
        peripheral::book_request(self.addr, "isEnabled", &msgpack::array(&[]));
    }

    pub fn read_last_is_enabled(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isEnabled")?;
        peripheral::decode(&data)
    }

    pub fn is_enabled_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isEnabled",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// タイムアウト値を取得する (imm 対応, ticks)。
    pub fn book_next_get_timeout(&mut self) {
        peripheral::book_request(self.addr, "getTimeout", &msgpack::array(&[]));
    }

    pub fn read_last_get_timeout(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTimeout")?;
        peripheral::decode(&data)
    }

    pub fn get_timeout_imm(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getTimeout",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 有効/無効を設定する。
    pub fn book_next_set_enabled(&mut self, enabled: bool) {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::book_action(self.addr, "setEnabled", &args);
    }

    pub fn read_last_set_enabled(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setEnabled")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// タイムアウトを設定する (ticks)。
    pub fn book_next_set_timeout(&mut self, ticks: u32) {
        let args = msgpack::array(&[msgpack::int(ticks as i32)]);
        peripheral::book_action(self.addr, "setTimeout", &args);
    }

    pub fn read_last_set_timeout(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTimeout")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// タイマーをリセットする。
    pub fn book_next_reset(&mut self) {
        peripheral::book_action(self.addr, "reset", &msgpack::array(&[]));
    }

    pub fn read_last_reset(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "reset")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
