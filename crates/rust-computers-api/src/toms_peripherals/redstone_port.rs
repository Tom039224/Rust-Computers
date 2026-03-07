//! Toms-Peripherals RedstonePort。

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// RedstonePort ペリフェラル。
pub struct RedstonePort {
    dir: Direction,
}

impl Peripheral for RedstonePort {
    const NAME: &'static str = "tm:redstone_port";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl RedstonePort {
    /// 利用可能なサイド一覧を取得する (imm のみ)。
    pub fn get_sides_imm(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getSides",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 指定サイドの入力を取得する。
    pub async fn get_input(&self, side: &str) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info(self.dir, "getInput", &args).await?;
        peripheral::decode(&data)
    }

    /// アナログ入力を取得する。
    pub async fn get_analog_input(&self, side: &str) -> Result<u8, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.dir, "getAnalogInput", &args).await?;
        peripheral::decode(&data)
    }

    /// バンドル入力を取得する。
    pub async fn get_bundled_input(&self, side: &str) -> Result<u16, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.dir, "getBundledInput", &args).await?;
        peripheral::decode(&data)
    }

    /// 出力を取得する (imm 対応)。
    pub async fn get_output(&self, side: &str) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.dir, "getOutput", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_output_imm(&self, side: &str) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.dir, "getOutput", &args)?;
        peripheral::decode(&data)
    }

    /// アナログ出力を取得する (imm 対応)。
    pub async fn get_analog_output(&self, side: &str) -> Result<u8, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.dir, "getAnalogOutput", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_analog_output_imm(&self, side: &str) -> Result<u8, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.dir, "getAnalogOutput", &args)?;
        peripheral::decode(&data)
    }

    /// バンドル出力を取得する (imm 対応)。
    pub async fn get_bundled_output(&self, side: &str) -> Result<u16, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data =
            peripheral::request_info(self.dir, "getBundledOutput", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_bundled_output_imm(&self, side: &str) -> Result<u16, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side)]);
        let data = peripheral::request_info_imm(self.dir, "getBundledOutput", &args)?;
        peripheral::decode(&data)
    }

    /// 出力を設定する。
    pub async fn set_output(&self, side: &str, value: bool) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side), msgpack::bool_val(value)]);
        peripheral::do_action(self.dir, "setOutput", &args).await?;
        Ok(())
    }

    /// アナログ出力を設定する。
    pub async fn set_analog_output(
        &self,
        side: &str,
        value: u8,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(value as i32)]);
        peripheral::do_action(self.dir, "setAnalogOutput", &args).await?;
        Ok(())
    }

    /// バンドル出力を設定する。
    pub async fn set_bundled_output(
        &self,
        side: &str,
        mask: u16,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(side), msgpack::int(mask as i32)]);
        peripheral::do_action(self.dir, "setBundledOutput", &args).await?;
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
            peripheral::request_info(self.dir, "testBundledInput", &args).await?;
        peripheral::decode(&data)
    }
}
