//! CC-VS Drag API。
//! CC-VS Drag API for ship drag/lift control.

use super::ship::VSVector3;
use alloc::vec::Vec;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Drag API (シップ上に配置されたコンピュータから呼び出す)。
/// Drag API (called from a computer placed on a ship).
pub struct Drag {
    addr: PeriphAddr,
}

impl Peripheral for Drag {
    const NAME: &'static str = "vs_drag";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Drag {
    // ====== 読み取り系 (book/read + imm 対応) ======

    /// 抗力ベクトルを取得する (book/read)。
    pub fn book_next_get_drag_force(&mut self) {
        peripheral::book_request(
            self.addr,
            "getDragForce",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_drag_force(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getDragForce")?;
        peripheral::decode(&data)
    }

    pub fn get_drag_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getDragForce",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 揚力ベクトルを取得する (book/read)。
    pub fn book_next_get_lift_force(&mut self) {
        peripheral::book_request(
            self.addr,
            "getLiftForce",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_lift_force(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getLiftForce")?;
        peripheral::decode(&data)
    }

    pub fn get_lift_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getLiftForce",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    // ====== 状態変更系 (allow_op) ======

    /// ドラッグを有効化する。
    pub fn book_next_enable_drag(&mut self) {
        peripheral::book_action(self.addr, "enableDrag", &msgpack::array(&[]));
    }

    pub fn read_last_enable_drag(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "enableDrag")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ドラッグを無効化する。
    pub fn book_next_disable_drag(&mut self) {
        peripheral::book_action(self.addr, "disableDrag", &msgpack::array(&[]));
    }

    pub fn read_last_disable_drag(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "disableDrag")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// リフトを有効化する。
    pub fn book_next_enable_lift(&mut self) {
        peripheral::book_action(self.addr, "enableLift", &msgpack::array(&[]));
    }

    pub fn read_last_enable_lift(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "enableLift")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// リフトを無効化する。
    pub fn book_next_disable_lift(&mut self) {
        peripheral::book_action(self.addr, "disableLift", &msgpack::array(&[]));
    }

    pub fn read_last_disable_lift(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "disableLift")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 回転ドラッグを有効化する。
    pub fn book_next_enable_rot_drag(&mut self) {
        peripheral::book_action(self.addr, "enableRotDrag", &msgpack::array(&[]));
    }

    pub fn read_last_enable_rot_drag(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "enableRotDrag")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 回転ドラッグを無効化する。
    pub fn book_next_disable_rot_drag(&mut self) {
        peripheral::book_action(self.addr, "disableRotDrag", &msgpack::array(&[]));
    }

    pub fn read_last_disable_rot_drag(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "disableRotDrag")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 風向を設定する。
    pub fn book_next_set_wind_direction(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_action(self.addr, "setWindDirection", &args);
    }

    pub fn read_last_set_wind_direction(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setWindDirection")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 風速を設定する。
    pub fn book_next_set_wind_speed(&mut self, speed: f64) {
        let args = msgpack::array(&[msgpack::float64(speed)]);
        peripheral::book_action(self.addr, "setWindSpeed", &args);
    }

    pub fn read_last_set_wind_speed(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setWindSpeed")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 風インパルスを印加する。
    pub fn book_next_apply_wind_impulse(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        speed: f64,
    ) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
            msgpack::float64(speed),
        ]);
        peripheral::book_action(self.addr, "applyWindImpulse", &args);
    }

    pub fn read_last_apply_wind_impulse(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "applyWindImpulse")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
