//! Create Frogport ペリフェラル。
//! Create Frogport peripheral.
//!
//! すべてのメソッドは mainThread=true のため imm 非対応。
//! All methods are mainThread=true, so no imm variants.

use alloc::string::String;
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::common::{CRItemDetail, CRPackage, CRSlotInfo};

/// Frogport ペリフェラル。
pub struct Frogport {
    addr: PeriphAddr,
}

impl Peripheral for Frogport {
    const NAME: &'static str = "Create_Frogport";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Frogport {
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

    /// 構成情報を取得する。
    pub fn book_next_get_configuration(&mut self) {
        peripheral::book_request(
            self.addr,
            "getConfiguration",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_configuration(&self) -> Result<CRPackage, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getConfiguration")?;
        peripheral::decode(&data)
    }

    /// 構成情報を設定する。
    pub fn book_next_set_configuration(
        &mut self,
        config: &CRPackage,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(config)?;
        let args = msgpack::array(&[encoded]);
        peripheral::book_action(self.addr, "setConfiguration", &args);
        Ok(())
    }

    pub fn read_last_set_configuration(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setConfiguration")
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

impl Frogport {
    pub async fn async_set_address(&mut self, address: &str) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_address(address);
        crate::wait_for_next_tick().await;
        self.read_last_set_address()
    }

    pub async fn async_get_address(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_address();
        crate::wait_for_next_tick().await;
        self.read_last_get_address()
    }

    pub async fn async_get_configuration(&mut self) -> Result<CRPackage, PeripheralError> {
        self.book_next_get_configuration();
        crate::wait_for_next_tick().await;
        self.read_last_get_configuration()
    }

    pub async fn async_set_configuration(&mut self, config: &CRPackage) -> Vec<Result<(), PeripheralError>> {
        let _ = self.book_next_set_configuration(config);
        crate::wait_for_next_tick().await;
        self.read_last_set_configuration()
    }

    pub async fn async_list(&mut self) -> Result<Vec<CRSlotInfo>, PeripheralError> {
        self.book_next_list();
        crate::wait_for_next_tick().await;
        self.read_last_list()
    }

    pub async fn async_get_item_detail(&mut self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError> {
        self.book_next_get_item_detail(slot);
        crate::wait_for_next_tick().await;
        self.read_last_get_item_detail()
    }
}
