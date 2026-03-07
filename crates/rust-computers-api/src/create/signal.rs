//! Create Signal ペリフェラル。
//! Create Signal peripheral.

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Signal ペリフェラル。
pub struct Signal {
    addr: PeriphAddr,
}

impl Peripheral for Signal {
    const NAME: &'static str = "create:signal";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Signal {
    /// シグナルの状態を取得する。
    pub async fn get_state(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getState",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// シグナルの状態を即時取得する (imm 対応)。
    pub fn get_state_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getState",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 強制赤信号かどうかを取得する。
    pub async fn is_forced_red(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "isForcedRed",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 強制赤信号かどうかを即時取得する (imm 対応)。
    pub fn is_forced_red_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isForcedRed",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 強制赤信号を設定する。
    pub async fn set_forced_red(&self, powered: bool) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::bool_val(powered)]);
        peripheral::do_action(self.addr, "setForcedRed", &args).await?;
        Ok(())
    }

    /// ブロック中の列車名一覧を取得する。
    pub async fn list_blocking_train_names(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "listBlockingTrainNames",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// ブロック中の列車名一覧を即時取得する (imm 対応)。
    pub fn list_blocking_train_names_imm(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "listBlockingTrainNames",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// シグナルタイプを取得する。
    pub async fn get_signal_type(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getSignalType",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// シグナルタイプを即時取得する (imm 対応)。
    pub fn get_signal_type_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getSignalType",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// シグナルタイプをサイクルする。
    pub async fn cycle_signal_type(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.addr, "cycleSignalType", &msgpack::array(&[])).await?;
        Ok(())
    }

    // ====== イベント系 / Events ======

    /// シグナル状態変化イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_train_signal_state_change(
        &self,
    ) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "try_pull_train_signal_state_change",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// シグナル状態変化イベントを受信するまで待機する。
    pub async fn pull_train_signal_state_change(&self) -> Result<(), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_train_signal_state_change().await? {
                return Ok(v);
            }
        }
    }
}
