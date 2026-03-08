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
    pub fn book_next_get_state(&mut self) {
        peripheral::book_request(
            self.addr,
            "getState",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_state(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getState")?;
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
    pub fn book_next_is_forced_red(&mut self) {
        peripheral::book_request(
            self.addr,
            "isForcedRed",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_forced_red(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isForcedRed")?;
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
    pub fn book_next_set_forced_red(&mut self, powered: bool) {
        let args = msgpack::array(&[msgpack::bool_val(powered)]);
        peripheral::book_action(self.addr, "setForcedRed", &args);
    }

    pub fn read_last_set_forced_red(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setForcedRed")?;
        Ok(())
    }

    /// ブロック中の列車名一覧を取得する。
    pub fn book_next_list_blocking_train_names(&mut self) {
        peripheral::book_request(
            self.addr,
            "listBlockingTrainNames",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_list_blocking_train_names(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "listBlockingTrainNames")?;
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
    pub fn book_next_get_signal_type(&mut self) {
        peripheral::book_request(
            self.addr,
            "getSignalType",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_signal_type(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSignalType")?;
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
    pub fn book_next_cycle_signal_type(&mut self) {
        peripheral::book_action(self.addr, "cycleSignalType", &msgpack::array(&[]));
    }

    pub fn read_last_cycle_signal_type(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "cycleSignalType")?;
        Ok(())
    }

    // ====== イベント系 / Events ======

    /// シグナル状態変化イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_train_signal_state_change(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_train_signal_state_change",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_train_signal_state_change(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_train_signal_state_change")?;
        peripheral::decode(&data)
    }

    /// シグナル状態変化イベントを受信するまで待機する。
    pub async fn pull_train_signal_state_change(&self) -> Result<(), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_train_signal_state_change", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_train_signal_state_change")?;
            let result: Option<()> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }
}
