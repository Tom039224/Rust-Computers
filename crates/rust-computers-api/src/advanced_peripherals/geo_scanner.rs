//! AdvancedPeripherals GeoScanner。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ブロックエントリ（scan 結果）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoBlockEntry {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// GeoScanner ペリフェラル。
pub struct GeoScanner {
    addr: PeriphAddr,
}

impl Peripheral for GeoScanner {
    const NAME: &'static str = "advancedPeripherals:geo_scanner";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl GeoScanner {
    /// 指定半径スキャンの演算コストを返す (imm 対応)。
    pub fn book_next_cost(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "cost", &args);
    }
    pub fn read_last_cost(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "cost")?;
        peripheral::decode(&data)
    }

    pub fn cost_imm(&self, radius: f64) -> Result<f64, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data = peripheral::request_info_imm(self.addr, "cost", &args)?;
        peripheral::decode(&data)
    }

    /// 指定半径内のブロック一覧を返す。
    pub fn book_next_scan(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "scan", &args);
    }
    pub fn read_last_scan(&self) -> Result<Vec<GeoBlockEntry>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "scan")?;
        peripheral::decode(&data)
    }

    /// 現在チャンクの鉱石分布（鉱石名→個数）を返す。
    pub fn book_next_chunk_analyze(&mut self) {
        peripheral::book_request(self.addr, "chunkAnalyze", &msgpack::array(&[]));
    }
    pub fn read_last_chunk_analyze(&self) -> Result<msgpack::Value, PeripheralError> {
        let data = peripheral::read_result(self.addr, "chunkAnalyze")?;
        peripheral::decode(&data)
    }
}
