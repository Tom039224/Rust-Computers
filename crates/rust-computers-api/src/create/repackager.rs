//! Create Repackager ペリフェラル。
//! Create Repackager peripheral.

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::common::{CRItemDetail, CRPackage, CRSlotInfo};

/// Repackager ペリフェラル。
pub struct Repackager {
    addr: PeriphAddr,
}

impl Peripheral for Repackager {
    const NAME: &'static str = "create:repackager";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Repackager {
    /// パッケージを作成する。
    pub async fn make_package(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.addr, "makePackage", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// インベントリ内のスロット一覧を取得する。
    pub async fn list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
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
            peripheral::request_info(self.addr, "getItemDetail", &args).await?;
        peripheral::decode(&data)
    }

    /// アドレスを取得する。
    pub async fn get_address(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getAddress",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// アドレスを設定する。
    pub async fn set_address(&self, address: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(address)]);
        peripheral::do_action(self.addr, "setAddress", &args).await?;
        Ok(())
    }

    /// パッケージ情報を取得する。
    pub async fn get_package(&self) -> Result<Option<CRPackage>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getPackage",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    // ====== イベント系 / Events ======

    /// パッケージ再梱包イベントを 1tick 待機して取得する。来なければ None。
    /// 戻り値は (package, count) のタプル。
    pub async fn try_pull_package_repackaged(
        &self,
    ) -> Result<Option<(CRPackage, u32)>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "try_pull_package_repackaged",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// パッケージ再梱包イベントを受信するまで待機する。
    pub async fn pull_package_repackaged(&self) -> Result<(CRPackage, u32), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_package_repackaged().await? {
                return Ok(v);
            }
        }
    }

    /// パッケージ受信イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_package_received(
        &self,
    ) -> Result<Option<CRPackage>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
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
            self.addr,
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
