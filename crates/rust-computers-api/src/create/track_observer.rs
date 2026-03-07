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
    const NAME: &'static str = "create:track_observer";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl TrackObserver {
    /// 列車が通過中かどうかを取得する。
    pub async fn is_train_passing(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "isTrainPassing",
            &msgpack::array(&[]),
        )
        .await?;
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
    pub async fn get_passing_train_name(&self) -> Result<Option<String>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getPassingTrainName",
            &msgpack::array(&[]),
        )
        .await?;
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
    pub async fn try_pull_train_passing(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "try_pull_train_passing",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車通過開始イベントを受信するまで待機する。
    pub async fn pull_train_passing(&self) -> Result<(), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_train_passing().await? {
                return Ok(v);
            }
        }
    }

    /// 列車通過完了イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_train_passed(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "try_pull_train_passed",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車通過完了イベントを受信するまで待機する。
    pub async fn pull_train_passed(&self) -> Result<(), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_train_passed().await? {
                return Ok(v);
            }
        }
    }
}
