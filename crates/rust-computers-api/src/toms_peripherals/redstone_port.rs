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
    const NAME: &'static str = "tm:redstone_port";

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
    pub async fn get_input(&self, side: &str) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info(self.addr, "getInput", &args).await?;
        peripheral::decode(&data)
    }

    /// アナログ入力を取得する。
    pub async fn get_analog_input(&self, side: &str) -> Result<u8, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.addr, "getAnalogInput", &args).await?;
        peripheral::decode(&data)
    }

    /// バンドル入力を取得する。
    pub async fn get_bundled_input(&self, side: &str) -> Result<u16, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.addr, "getBundledInput", &args).await?;
        peripheral::decode(&data)
    }

    /// 出力を取得する (imm 対応)。
    pub async fn get_output(&self, side: &str) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.addr, "getOutput", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_output_imm(&self, side: &str) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.addr, "getOutput", &args)?;
        peripheral::decode(&data)
    }

    /// アナログ出力を取得する (imm 対応)。
    pub async fn get_analog_output(&self, side: &str) -> Result<u8, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.addr, "getAnalogOutput", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_analog_output_imm(&self, side: &str) -> Result<u8, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.addr, "getAnalogOutput", &args)?;
        peripheral::decode(&data)
    }

    /// バンドル出力を取得する (imm 対応)。
    pub async fn get_bundled_output(&self, side: &str) -> Result<u16, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.addr, "getBundledOutput", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_bundled_output_imm(&self, side: &str) -> Result<u16, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.addr, "getBundledOutput", &args)?;
        peripheral::decode(&data)
    }

    /// 出力を設定する。
    pub async fn set_output(&self, side: &str, value: bool) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side), msgpack::bool_val(value)]);
        peripheral::do_action(self.addr, "setOutput", &args).await?;
        Ok(())
    }

    /// アナログ出力を設定する。
    pub async fn set_analog_output(
        &self,
        side: &str,
        value: u8,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(value as i32)]);
        peripheral::do_action(self.addr, "setAnalogOutput", &args).await?;
        Ok(())
    }

    /// バンドル出力を設定する。
    pub async fn set_bundled_output(
        &self,
        side: &str,
        mask: u16,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(mask as i32)]);
        peripheral::do_action(self.addr, "setBundledOutput", &args).await?;
        Ok(())
    }

    /// バンドル入力をテストする。
    pub async fn test_bundled_input(
        &self,
        side: &str,
        mask: u16,
    ) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(mask as i32)]);
        let data =
            peripheral::request_info(self.addr, "testBundledInput", &args).await?;
        peripheral::decode(&data)
    }
}
