//! Control-Craft SpatialAnchor ペリフェラル。

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// スペーシャルアンカー ペリフェラル。
pub struct SpatialAnchor {
    addr: PeriphAddr,
}

impl Peripheral for SpatialAnchor {
    const NAME: &'static str = "controlcraft:spatial_anchor_peripheral";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl SpatialAnchor {
    // ====== 状態変更系 ======

    /// 船を静的（動かない）状態に設定する。
    pub fn book_next_set_static(&mut self, enabled: bool) {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::book_action(self.addr, "setStatic", &args);
    }
    pub fn read_last_set_static(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setStatic")?;
        Ok(())
    }

    /// アンカーの動作状態を設定する。
    pub fn book_next_set_running(&mut self, enabled: bool) {
        let args = msgpack::array(&[msgpack::bool_val(enabled)]);
        peripheral::book_action(self.addr, "setRunning", &args);
    }
    pub fn read_last_set_running(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setRunning")?;
        Ok(())
    }

    /// アンカーのオフセット距離を設定する。
    pub fn book_next_set_offset(&mut self, offset: f64) {
        let args = msgpack::array(&[msgpack::float64(offset)]);
        peripheral::book_action(self.addr, "setOffset", &args);
    }
    pub fn read_last_set_offset(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setOffset")?;
        Ok(())
    }

    /// 位置制御 (PPID) のゲインを設定する。
    pub fn book_next_set_ppid(&mut self, p: f64, i: f64, d: f64) {
        let args = msgpack::array(&[
            msgpack::float64(p),
            msgpack::float64(i),
            msgpack::float64(d),
        ]);
        peripheral::book_action(self.addr, "setPPID", &args);
    }
    pub fn read_last_set_ppid(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setPPID")?;
        Ok(())
    }

    /// 回転制御 (QPID) のゲインを設定する。
    pub fn book_next_set_qpid(&mut self, p: f64, i: f64, d: f64) {
        let args = msgpack::array(&[
            msgpack::float64(p),
            msgpack::float64(i),
            msgpack::float64(d),
        ]);
        peripheral::book_action(self.addr, "setQPID", &args);
    }
    pub fn read_last_set_qpid(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setQPID")?;
        Ok(())
    }

    /// チャンネル番号 (long) を設定する。
    pub fn book_next_set_channel(&mut self, channel: i64) {
        let args = msgpack::array(&[msgpack::int64(channel)]);
        peripheral::book_action(self.addr, "setChannel", &args);
    }
    pub fn read_last_set_channel(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setChannel")?;
        Ok(())
    }
}
