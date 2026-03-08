//! Create Packager ペリフェラル。
//! Create Packager peripheral.

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::common::{CRItemDetail, CRPackage, CRSlotInfo};

/// Packager ペリフェラル。
pub struct Packager {
    addr: PeriphAddr,
}

impl Peripheral for Packager {
    const NAME: &'static str = "create:packager";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Packager {
    /// パッケージを作成する。
    pub fn book_next_make_package(&mut self) {
        peripheral::book_action(self.addr, "makePackage", &msgpack::array(&[]));
    }

    pub fn read_last_make_package(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "makePackage")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
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

    pub fn read_last_set_address(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setAddress")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
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
