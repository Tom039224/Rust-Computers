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
    pub fn book_next_make_package(&mut self) {
        peripheral::book_action(self.addr, "makePackage", &msgpack::array(&[]));
    }

    pub fn read_last_make_package(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "makePackage")?;
        Ok(())
    }

    /// インベントリ内のスロット一覧を取得する。
    pub fn book_next_list(&mut self) {
        peripheral::book_request(
            self.addr,
            "list",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "list")?;
        peripheral::decode(&data)
    }

    /// 指定スロットのアイテム詳細を取得する。
    pub fn book_next_get_item_detail(&mut self, slot: u32) {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        peripheral::book_request(self.addr, "getItemDetail", &args);
    }

    pub fn read_last_get_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItemDetail")?;
        peripheral::decode(&data)
    }

    /// アドレスを取得する。
    pub fn book_next_get_address(&mut self) {
        peripheral::book_request(
            self.addr,
            "getAddress",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_address(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAddress")?;
        peripheral::decode(&data)
    }

    /// アドレスを設定する。
    pub fn book_next_set_address(&mut self, address: &str) {
        let args = msgpack::array(&[msgpack::str(address)]);
        peripheral::book_action(self.addr, "setAddress", &args);
    }

    pub fn read_last_set_address(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setAddress")?;
        Ok(())
    }

    /// パッケージ情報を取得する。
    pub fn book_next_get_package(&mut self) {
        peripheral::book_request(
            self.addr,
            "getPackage",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_package(&self) -> Result<Option<CRPackage>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPackage")?;
        peripheral::decode(&data)
    }

    // ====== イベント系 / Events ======

    /// パッケージ再梱包イベントを 1tick 待機して取得する。来なければ None。
    /// 戻り値は (package, count) のタプル。
    pub fn book_next_try_pull_package_repackaged(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_package_repackaged",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_package_repackaged(&self) -> Result<Option<(CRPackage, u32)>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_package_repackaged")?;
        peripheral::decode(&data)
    }

    /// パッケージ再梱包イベントを受信するまで待機する。
    pub async fn pull_package_repackaged(&self) -> Result<(CRPackage, u32), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_package_repackaged", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_package_repackaged")?;
            let result: Option<(CRPackage, u32)> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }

    /// パッケージ受信イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_package_received(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_package_received",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_package_received")?;
        peripheral::decode(&data)
    }

    /// パッケージ受信イベントを受信するまで待機する。
    pub async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_package_received", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_package_received")?;
            let result: Option<CRPackage> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }

    /// パッケージ送信イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_package_sent(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_package_sent",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_package_sent")?;
        peripheral::decode(&data)
    }

    /// パッケージ送信イベントを受信するまで待機する。
    pub async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_package_sent", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_package_sent")?;
            let result: Option<CRPackage> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }
}
