//! Create Postbox ペリフェラル。
//! Create Postbox peripheral.

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

use super::common::{CRItemDetail, CRPackage, CRSlotInfo};

/// Postbox ペリフェラル。
pub struct Postbox {
    dir: Direction,
}

impl Peripheral for Postbox {
    const NAME: &'static str = "create:postbox";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl Postbox {
    /// アドレスを設定する。
    pub async fn set_address(&self, address: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(address)]);
        peripheral::do_action(self.dir, "setAddress", &args).await?;
        Ok(())
    }

    /// アドレスを取得する。
    pub async fn get_address(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getAddress",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// インベントリ内のスロット一覧を取得する。
    pub async fn list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "list",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 指定スロットのアイテム詳細を取得する。
    pub async fn get_item_detail(&self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        let data =
            peripheral::request_info(self.dir, "getItemDetail", &args).await?;
        peripheral::decode(&data)
    }

    /// 構成情報を取得する。
    pub async fn get_configuration(&self) -> Result<CRPackage, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getConfiguration",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 構成情報を設定する。
    pub async fn set_configuration(
        &self,
        config: &CRPackage,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(config)?;
        let args = msgpack::array(&[encoded]);
        peripheral::do_action(self.dir, "setConfiguration", &args).await?;
        Ok(())
    }

    // ====== イベント系 / Events ======

    /// パッケージ受信イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_package_received(
        &self,
    ) -> Result<Option<CRPackage>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "try_pull_package_received",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// パッケージ受信イベントを受信するまで待機する。
    pub async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_package_received().await? {
                return Ok(v);
            }
        }
    }

    /// パッケージ送信イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_package_sent(
        &self,
    ) -> Result<Option<CRPackage>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "try_pull_package_sent",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// パッケージ送信イベントを受信するまで待機する。
    pub async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_package_sent().await? {
                return Ok(v);
            }
        }
    }
}
