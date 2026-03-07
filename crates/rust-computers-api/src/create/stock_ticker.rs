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
    pub async fn stock(&self) -> Result<Vec<CRSlotInfo>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "stock",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 在庫アイテムの詳細情報を取得する。
    pub async fn get_stock_item_detail(
        &self,
        slot: u32,
    ) -> Result<Option<CRItemDetail>, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        let data =
            peripheral::request_info(self.addr, "getStockItemDetail", &args).await?;
        peripheral::decode(&data)
    }

    /// フィルタ付きリクエストを送信する。
    pub async fn request_filtered(
        &self,
        filters: &[CRItemFilter],
    ) -> Result<(), PeripheralError> {
        let filters_vec: Vec<_> = filters.iter().cloned().collect();
        let encoded = peripheral::encode(&filters_vec)?;
        let args = msgpack::array(&[encoded]);
        peripheral::do_action(self.addr, "requestFiltered", &args).await?;
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
    pub async fn get_item_detail(
        &self,
        slot: u32,
    ) -> Result<Option<CRItemDetail>, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(slot as i32)]);
        let data =
            peripheral::request_info(self.addr, "getItemDetail", &args).await?;
        peripheral::decode(&data)
    }
}
