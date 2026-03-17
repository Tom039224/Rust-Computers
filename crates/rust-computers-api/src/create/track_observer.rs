//! Create Track Observer ペリフェラル。
//! Create Track Observer peripheral.

use alloc::string::String;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Track Observer ペリフェラル。
pub struct TrackObserver {
    addr: PeriphAddr,
}

impl Peripheral for TrackObserver {
    const NAME: &'static str = "Create_TrainObserver";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl TrackObserver {
    /// 列車が通過中かどうかを取得する。
    pub fn book_next_is_train_passing(&mut self) {
        peripheral::book_request(
            self.addr,
            "isTrainPassing",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_train_passing(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isTrainPassing")?;
        peripheral::decode(&data)
    }

    /// 列車が通過中かどうかを即時取得する (imm 対応)。
    pub fn is_train_passing_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isTrainPassing",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 通過中の列車名を取得する。列車がない場合は None。
    pub fn book_next_get_passing_train_name(&mut self) {
        peripheral::book_request(
            self.addr,
            "getPassingTrainName",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_passing_train_name(&self) -> Result<Option<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPassingTrainName")?;
        peripheral::decode(&data)
    }

    /// 通過中の列車名を即時取得する (imm 対応)。
    pub fn get_passing_train_name_imm(&self) -> Result<Option<String>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getPassingTrainName",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    // ====== イベント系 / Events ======

    /// 列車通過開始イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_train_passing(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_train_passing",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_train_passing(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_train_passing")?;
        peripheral::decode(&data)
    }

    /// 列車通過開始イベントを受信するまで待機する。
    pub async fn pull_train_passing(&self) -> Result<(), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_train_passing", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_train_passing")?;
            let result: Option<()> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }

    /// 列車通過完了イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_train_passed(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_train_passed",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_train_passed(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_train_passed")?;
        peripheral::decode(&data)
    }

    /// 列車通過完了イベントを受信するまで待機する。
    pub async fn pull_train_passed(&self) -> Result<(), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_train_passed", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_train_passed")?;
            let result: Option<()> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }
}

impl TrackObserver {
    pub async fn async_is_train_passing(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_train_passing();
        crate::wait_for_next_tick().await;
        self.read_last_is_train_passing()
    }

    pub async fn async_get_passing_train_name(&mut self) -> Result<Option<String>, PeripheralError> {
        self.book_next_get_passing_train_name();
        crate::wait_for_next_tick().await;
        self.read_last_get_passing_train_name()
    }
}
