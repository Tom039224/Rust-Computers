//! Toms-Peripherals WatchDogTimer。

use crate::error::PeripheralError;
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
    pub async fn is_enabled(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "isEnabled",
            &msgpack::array(&[]),
        )
        .await?;
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
    pub async fn get_timeout(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getTimeout",
            &msgpack::array(&[]),
        )
        .await?;
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
    pub async fn set_enabled(&self, enabled: bool) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::do_action(self.addr, "setEnabled", &args).await?;
        Ok(())
    }

    /// タイムアウトを設定する (ticks)。
    pub async fn set_timeout(&self, ticks: u32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(ticks as i32)]);
        peripheral::do_action(self.addr, "setTimeout", &args).await?;
        Ok(())
    }

    /// タイマーをリセットする。
    pub async fn reset(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.addr, "reset", &msgpack::array(&[])).await?;
        Ok(())
    }
}
