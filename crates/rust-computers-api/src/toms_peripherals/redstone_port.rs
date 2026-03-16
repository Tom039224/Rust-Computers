//! Toms-Peripherals RedstonePort。

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// RedstonePort ペリフェラル。
pub struct RedstonePort {
    addr: PeriphAddr,
}

impl Peripheral for RedstonePort {
    const NAME: &'static str = "tm_rsPort";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl RedstonePort {
    /// 利用可能なサイド一覧を取得する (imm のみ)。
    pub fn get_sides_imm(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getSides",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 指定サイドの入力を取得する。
    pub fn book_next_get_input(&mut self, side: &str) {
        let args = msgpack::array(&[msgpack::str(side)]);
        peripheral::book_request(self.addr, "getInput", &args);
    }

    pub fn read_last_get_input(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getInput")?;
        peripheral::decode(&data)
    }

    /// アナログ入力を取得する。
    pub fn book_next_get_analog_input(&mut self, side: &str) {
        let args = msgpack::array(&[msgpack::str(side)]);
        peripheral::book_request(self.addr, "getAnalogInput", &args);
    }

    pub fn read_last_get_analog_input(&self) -> Result<u8, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAnalogInput")?;
        peripheral::decode(&data)
    }

    /// バンドル入力を取得する。
    pub fn book_next_get_bundled_input(&mut self, side: &str) {
        let args = msgpack::array(&[msgpack::str(side)]);
        peripheral::book_request(self.addr, "getBundledInput", &args);
    }

    pub fn read_last_get_bundled_input(&self) -> Result<u16, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBundledInput")?;
        peripheral::decode(&data)
    }

    /// 出力を取得する (imm 対応)。
    pub fn book_next_get_output(&mut self, side: &str) {
        let args = msgpack::array(&[msgpack::str(side)]);
        peripheral::book_request(self.addr, "getOutput", &args);
    }

    pub fn read_last_get_output(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getOutput")?;
        peripheral::decode(&data)
    }

    pub fn get_output_imm(&self, side: &str) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.addr, "getOutput", &args)?;
        peripheral::decode(&data)
    }

    /// アナログ出力を取得する (imm 対応)。
    pub fn book_next_get_analog_output(&mut self, side: &str) {
        let args = msgpack::array(&[msgpack::str(side)]);
        peripheral::book_request(self.addr, "getAnalogOutput", &args);
    }

    pub fn read_last_get_analog_output(&self) -> Result<u8, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAnalogOutput")?;
        peripheral::decode(&data)
    }

    pub fn get_analog_output_imm(&self, side: &str) -> Result<u8, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.addr, "getAnalogOutput", &args)?;
        peripheral::decode(&data)
    }

    /// バンドル出力を取得する (imm 対応)。
    pub fn book_next_get_bundled_output(&mut self, side: &str) {
        let args = msgpack::array(&[msgpack::str(side)]);
        peripheral::book_request(self.addr, "getBundledOutput", &args);
    }

    pub fn read_last_get_bundled_output(&self) -> Result<u16, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBundledOutput")?;
        peripheral::decode(&data)
    }

    pub fn get_bundled_output_imm(&self, side: &str) -> Result<u16, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.addr, "getBundledOutput", &args)?;
        peripheral::decode(&data)
    }

    /// 出力を設定する。
    pub fn book_next_set_output(&mut self, side: &str, value: bool) {
        let args = msgpack::array(&[msgpack::str(side), msgpack::bool_val(value)]);
        peripheral::book_action(self.addr, "setOutput", &args);
    }

    pub fn read_last_set_output(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setOutput")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// アナログ出力を設定する。
    pub fn book_next_set_analog_output(&mut self, side: &str, value: u8) {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(value as i32)]);
        peripheral::book_action(self.addr, "setAnalogOutput", &args);
    }

    pub fn read_last_set_analog_output(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setAnalogOutput")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// バンドル出力を設定する。
    pub fn book_next_set_bundled_output(&mut self, side: &str, mask: u16) {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(mask as i32)]);
        peripheral::book_action(self.addr, "setBundledOutput", &args);
    }

    pub fn read_last_set_bundled_output(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setBundledOutput")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// バンドル入力をテストする。
    pub fn book_next_test_bundled_input(&mut self, side: &str, mask: u16) {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(mask as i32)]);
        peripheral::book_request(self.addr, "testBundledInput", &args);
    }

    pub fn read_last_test_bundled_input(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "testBundledInput")?;
        peripheral::decode(&data)
    }
}
