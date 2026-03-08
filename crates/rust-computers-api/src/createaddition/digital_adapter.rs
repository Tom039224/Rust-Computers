//! Create Additions DigitalAdapter。

use alloc::string::String;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// DigitalAdapter ペリフェラル。
pub struct DigitalAdapter {
    addr: PeriphAddr,
}

impl Peripheral for DigitalAdapter {
    const NAME: &'static str = "createaddition:digital_adapter";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl DigitalAdapter {
    // ─── ディスプレイ操作 ────────────────────────────────────

    /// 指定行をクリアする。
    pub fn book_next_clear_line(&mut self, line: i32) {
        let args = msgpack::array(&[msgpack::int(line)]);
        peripheral::book_action(self.addr, "clearLine", &args);
    }
    pub fn read_last_clear_line(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "clearLine")?;
        Ok(())
    }

    /// 全行をクリアする。
    pub fn book_next_clear(&mut self) {
        peripheral::book_action(self.addr, "clear", &msgpack::array(&[]));
    }
    pub fn read_last_clear(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "clear")?;
        Ok(())
    }

    /// 次の空き行にテキストを出力する。
    pub fn book_next_print(&mut self, text: &str) {
        let args = msgpack::array(&[msgpack::str(text)]);
        peripheral::book_action(self.addr, "print", &args);
    }
    pub fn read_last_print(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "print")?;
        Ok(())
    }

    /// 指定行のテキストを取得する。
    pub fn book_next_get_line(&mut self, line: i32) {
        let args = msgpack::array(&[msgpack::int(line)]);
        peripheral::book_request(self.addr, "getLine", &args);
    }
    pub fn read_last_get_line(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getLine")?;
        peripheral::decode(&data)
    }

    /// 指定行にテキストを設定する。
    pub fn book_next_set_line(&mut self, line: i32, text: &str) {
        let args = msgpack::array(&[msgpack::int(line), msgpack::str(text)]);
        peripheral::book_action(self.addr, "setLine", &args);
    }
    pub fn read_last_set_line(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setLine")?;
        Ok(())
    }

    /// ディスプレイの最大行数を返す。
    pub fn book_next_get_max_lines(&mut self) {
        peripheral::book_request(self.addr, "getMaxLines", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_lines(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxLines")?;
        peripheral::decode(&data)
    }

    // ─── キネティック制御 ────────────────────────────────────

    /// 指定方向の機械の目標速度を設定する。
    pub fn book_next_set_target_speed(&mut self, dir: &str, speed: f64) {
        let args = msgpack::array(&[msgpack::str(dir), msgpack::float64(speed)]);
        peripheral::book_action(self.addr, "setTargetSpeed", &args);
    }
    pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setTargetSpeed")?;
        Ok(())
    }

    /// 指定方向の目標速度を取得する。
    pub fn book_next_get_target_speed(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getTargetSpeed", &args);
    }
    pub fn read_last_get_target_speed(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTargetSpeed")?;
        peripheral::decode(&data)
    }

    /// 指定方向の応力 (SU) を取得する。
    pub fn book_next_get_kinetic_stress(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getKineticStress", &args);
    }
    pub fn read_last_get_kinetic_stress(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getKineticStress")?;
        peripheral::decode(&data)
    }

    /// 指定方向の応力容量 (SU) を取得する。
    pub fn book_next_get_kinetic_capacity(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getKineticCapacity", &args);
    }
    pub fn read_last_get_kinetic_capacity(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getKineticCapacity")?;
        peripheral::decode(&data)
    }

    /// 指定方向の実際の速度 (RPM) を取得する。
    pub fn book_next_get_kinetic_speed(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getKineticSpeed", &args);
    }
    pub fn read_last_get_kinetic_speed(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getKineticSpeed")?;
        peripheral::decode(&data)
    }

    /// キネティックネットワークの最高速度 (RPM) を返す。
    pub fn book_next_get_kinetic_top_speed(&mut self) {
        peripheral::book_request(self.addr, "getKineticTopSpeed", &msgpack::array(&[]));
    }
    pub fn read_last_get_kinetic_top_speed(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getKineticTopSpeed")?;
        peripheral::decode(&data)
    }

    // ─── 機械状態読み取り ────────────────────────────────────

    /// 指定方向のプーリーの伸長距離 (blocks) を返す。
    pub fn book_next_get_pulley_distance(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getPulleyDistance", &args);
    }
    pub fn read_last_get_pulley_distance(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPulleyDistance")?;
        peripheral::decode(&data)
    }

    /// 指定方向のピストンの伸長距離 (blocks) を返す。
    pub fn book_next_get_piston_distance(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getPistonDistance", &args);
    }
    pub fn read_last_get_piston_distance(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPistonDistance")?;
        peripheral::decode(&data)
    }

    /// 指定方向のベアリングの角度 (deg) を返す。
    pub fn book_next_get_bearing_angle(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getBearingAngle", &args);
    }
    pub fn read_last_get_bearing_angle(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBearingAngle")?;
        peripheral::decode(&data)
    }

    /// 指定方向のエレベーターの現在フロア番号を返す。
    pub fn book_next_get_elevator_floor(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getElevatorFloor", &args);
    }
    pub fn read_last_get_elevator_floor(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getElevatorFloor")?;
        peripheral::decode(&data)
    }

    /// 指定方向のエレベーターが目的フロアに到着したかを返す。
    pub fn book_next_has_elevator_arrived(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "hasElevatorArrived", &args);
    }
    pub fn read_last_has_elevator_arrived(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "hasElevatorArrived")?;
        peripheral::decode(&data)
    }

    /// 指定方向のエレベーターの総フロア数を返す。
    pub fn book_next_get_elevator_floors(&mut self, dir: &str) {
        let args = msgpack::array(&[msgpack::str(dir)]);
        peripheral::book_request(self.addr, "getElevatorFloors", &args);
    }
    pub fn read_last_get_elevator_floors(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getElevatorFloors")?;
        peripheral::decode(&data)
    }

    /// 指定フロアの名前を返す。
    pub fn book_next_get_elevator_floor_name(&mut self, dir: &str, index: i32) {
        let args = msgpack::array(&[msgpack::str(dir), msgpack::int(index)]);
        peripheral::book_request(self.addr, "getElevatorFloorName", &args);
    }
    pub fn read_last_get_elevator_floor_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getElevatorFloorName")?;
        peripheral::decode(&data)
    }

    /// 指定フロアへ移動し、Y 座標差分を返す。
    pub fn book_next_goto_elevator_floor(&mut self, dir: &str, index: i32) {
        let args = msgpack::array(&[msgpack::str(dir), msgpack::int(index)]);
        peripheral::book_action(self.addr, "gotoElevatorFloor", &args);
    }
    pub fn read_last_goto_elevator_floor(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "gotoElevatorFloor")?;
        peripheral::decode(&data)
    }

    // ─── ユーティリティ ──────────────────────────────────────

    /// 指定角度を指定 RPM で回転する所要秒数を返す。
    pub fn book_next_get_duration_angle(&mut self, degrees: f64, rpm: f64) {
        let args = msgpack::array(&[msgpack::float64(degrees), msgpack::float64(rpm)]);
        peripheral::book_request(self.addr, "getDurationAngle", &args);
    }
    pub fn read_last_get_duration_angle(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getDurationAngle")?;
        peripheral::decode(&data)
    }

    /// 指定距離を指定 RPM で移動する所要秒数を返す。
    pub fn book_next_get_duration_distance(&mut self, blocks: f64, rpm: f64) {
        let args = msgpack::array(&[msgpack::float64(blocks), msgpack::float64(rpm)]);
        peripheral::book_request(self.addr, "getDurationDistance", &args);
    }
    pub fn read_last_get_duration_distance(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getDurationDistance")?;
        peripheral::decode(&data)
    }
}
