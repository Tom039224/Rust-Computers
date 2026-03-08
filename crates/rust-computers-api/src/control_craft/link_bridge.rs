//! Control-Craft LinkBridge ペリフェラル。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// リンクブリッジ ペリフェラル。
pub struct LinkBridge {
    addr: PeriphAddr,
}

impl Peripheral for LinkBridge {
    const NAME: &'static str = "controlcraft:link_bridge_peripheral";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl LinkBridge {
    // ====== 入力設定 ======

    /// 指定インデックスの入力値を設定する。
    pub fn book_next_set_input(&mut self, index: f64, value: f64) {
        let args = msgpack::array(&[msgpack::float64(index), msgpack::float64(value)]);
        peripheral::book_action(self.addr, "setInput", &args);
    }
    pub fn read_last_set_input(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setInput")?;
        Ok(())
    }

    // ====== 出力取得 (imm 対応) ======

    /// 指定インデックスの出力値を取得する (book)。
    pub fn book_next_get_output(&mut self, index: f64) {
        let args = msgpack::array(&[msgpack::float64(index)]);
        peripheral::book_request(self.addr, "getOutput", &args);
    }
    pub fn read_last_get_output(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getOutput")?;
        peripheral::decode(&data)
    }
    /// 指定インデックスの出力値を即時取得する。
    pub fn get_output_imm(&self, index: f64) -> Result<f64, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(index)]);
        let data = peripheral::request_info_imm(self.addr, "getOutput", &args)?;
        peripheral::decode(&data)
    }
}
