//! Create Stock Ticker ペリフェラル。
//! Create Stock Ticker peripheral.

use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

use super::common::{CRItemDetail, CRItemFilter, CRSlotInfo};

/// Stock Ticker ペリフェラル。
pub struct StockTicker {
    addr: PeriphAddr,
}

impl Peripheral for StockTicker {
    const NAME: &'static str = "create:stock_ticker";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl StockTicker {
    /// 在庫情報を取得する。
    pub fn book_next_stock(&mut self) {
        peripheral::book_request(
            self.addr,
            "stock",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_stock(&self) -> Result<Vec<CRSlotInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "stock")?;
        peripheral::decode(&data)
    }

    /// 在庫アイテムの詳細情報を取得する。
    pub fn book_next_get_stock_item_detail(&mut self, slot: u32) {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        peripheral::book_request(self.addr, "getStockItemDetail", &args);
    }

    pub fn read_last_get_stock_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getStockItemDetail")?;
        peripheral::decode(&data)
    }

    /// フィルタ付きリクエストを送信する。
    pub fn book_next_request_filtered(
        &mut self,
        filters: &[CRItemFilter],
    ) -> Result<(), PeripheralError> {
        let filters_vec: Vec<_> = filters.iter().cloned().collect();
        let encoded = peripheral::encode(&filters_vec)?;
        let args = msgpack::array(&[encoded]);
        peripheral::book_action(self.addr, "requestFiltered", &args);
        Ok(())
    }

    pub fn read_last_request_filtered(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "requestFiltered")?;
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
}
