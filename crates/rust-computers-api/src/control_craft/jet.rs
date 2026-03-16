//! Control-Craft Jet ペリフェラル。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ジェット（推力機器） ペリフェラル。
pub struct Jet {
    addr: PeriphAddr,
}

impl Peripheral for Jet {
    const NAME: &'static str = "attacker";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Jet {
    // ====== 状態変更系 ======

    /// 出力推力スケールを設定する。
    pub fn book_next_set_output_thrust(&mut self, thrust: f64) {
        let args = msgpack::array(&[msgpack::float64(thrust)]);
        peripheral::book_action(self.addr, "setOutputThrust", &args);
    }
    pub fn read_last_set_output_thrust(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setOutputThrust")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 水平方向チルト角度 (deg) を設定する。
    pub fn book_next_set_horizontal_tilt(&mut self, angle: f64) {
        let args = msgpack::array(&[msgpack::float64(angle)]);
        peripheral::book_action(self.addr, "setHorizontalTilt", &args);
    }
    pub fn read_last_set_horizontal_tilt(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setHorizontalTilt")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 垂直方向チルト角度 (deg) を設定する。
    pub fn book_next_set_vertical_tilt(&mut self, angle: f64) {
        let args = msgpack::array(&[msgpack::float64(angle)]);
        peripheral::book_action(self.addr, "setVerticalTilt", &args);
    }
    pub fn read_last_set_vertical_tilt(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setVerticalTilt")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
