//! Create Station ペリフェラル。
//! Create Station peripheral.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::msgpack::Value;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Station ペリフェラル。
pub struct Station {
    addr: PeriphAddr,
}

impl Peripheral for Station {
    const NAME: &'static str = "create:station";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Station {
    /// 列車を組み立てる。
    pub fn book_next_assemble(&mut self) {
        peripheral::book_action(self.addr, "assemble", &msgpack::array(&[]));
    }

    pub fn read_last_assemble(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "assemble")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 列車を分解する。
    pub fn book_next_disassemble(&mut self) {
        peripheral::book_action(self.addr, "disassemble", &msgpack::array(&[]));
    }

    pub fn read_last_disassemble(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "disassemble")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 組み立てモードを設定する。
    pub fn book_next_set_assembly_mode(&mut self, mode: bool) {
        let args = msgpack::array(&[msgpack::bool_val(mode)]);
        peripheral::book_action(self.addr, "setAssemblyMode", &args);
    }

    pub fn read_last_set_assembly_mode(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setAssemblyMode")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 組み立てモードかどうかを取得する。
    pub fn book_next_is_in_assembly_mode(&mut self) {
        peripheral::book_request(
            self.addr,
            "isInAssemblyMode",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_in_assembly_mode(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isInAssemblyMode")?;
        peripheral::decode(&data)
    }

    /// 組み立てモードかどうかを即時取得する (imm 対応)。
    pub fn is_in_assembly_mode_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isInAssemblyMode",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ステーション名を取得する。
    pub fn book_next_get_station_name(&mut self) {
        peripheral::book_request(
            self.addr,
            "getStationName",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_station_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getStationName")?;
        peripheral::decode(&data)
    }

    /// ステーション名を即時取得する (imm 対応)。
    pub fn get_station_name_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getStationName",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ステーション名を設定する。
    pub fn book_next_set_station_name(&mut self, name: &str) {
        let args = msgpack::array(&[msgpack::str(name)]);
        peripheral::book_action(self.addr, "setStationName", &args);
    }

    pub fn read_last_set_station_name(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setStationName")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 列車が存在するかどうかを取得する。
    pub fn book_next_is_train_present(&mut self) {
        peripheral::book_request(
            self.addr,
            "isTrainPresent",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_train_present(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isTrainPresent")?;
        peripheral::decode(&data)
    }

    /// 列車が存在するかどうかを即時取得する (imm 対応)。
    pub fn is_train_present_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isTrainPresent",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車が間もなく到着するかどうかを取得する。
    pub fn book_next_is_train_imminent(&mut self) {
        peripheral::book_request(
            self.addr,
            "isTrainImminent",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_train_imminent(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isTrainImminent")?;
        peripheral::decode(&data)
    }

    /// 列車が間もなく到着するかどうかを即時取得する (imm 対応)。
    pub fn is_train_imminent_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isTrainImminent",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車が経路上にあるかどうかを取得する。
    pub fn book_next_is_train_enroute(&mut self) {
        peripheral::book_request(
            self.addr,
            "isTrainEnroute",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_train_enroute(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isTrainEnroute")?;
        peripheral::decode(&data)
    }

    /// 列車が経路上にあるかどうかを即時取得する (imm 対応)。
    pub fn is_train_enroute_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isTrainEnroute",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車名を取得する。
    pub fn book_next_get_train_name(&mut self) {
        peripheral::book_request(
            self.addr,
            "getTrainName",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_train_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTrainName")?;
        peripheral::decode(&data)
    }

    /// 列車名を即時取得する (imm 対応)。
    pub fn get_train_name_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getTrainName",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車名を設定する。
    pub fn book_next_set_train_name(&mut self, name: &str) {
        let args = msgpack::array(&[msgpack::str(name)]);
        peripheral::book_action(self.addr, "setTrainName", &args);
    }

    pub fn read_last_set_train_name(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTrainName")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// スケジュールが存在するかどうかを取得する。
    pub fn book_next_has_schedule(&mut self) {
        peripheral::book_request(
            self.addr,
            "hasSchedule",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_has_schedule(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "hasSchedule")?;
        peripheral::decode(&data)
    }

    /// スケジュールが存在するかどうかを即時取得する (imm 対応)。
    pub fn has_schedule_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "hasSchedule",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// スケジュールを取得する。
    pub fn book_next_get_schedule(&mut self) {
        peripheral::book_request(
            self.addr,
            "getSchedule",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_schedule(
        &self,
    ) -> Result<BTreeMap<String, Value>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSchedule")?;
        peripheral::decode(&data)
    }

    /// スケジュールを即時取得する (imm 対応)。
    pub fn get_schedule_imm(
        &self,
    ) -> Result<BTreeMap<String, Value>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getSchedule",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// スケジュールを設定する。
    pub fn book_next_set_schedule(
        &mut self,
        schedule: &BTreeMap<String, Value>,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(schedule)?;
        let args = msgpack::array(&[encoded]);
        peripheral::book_action(self.addr, "setSchedule", &args);
        Ok(())
    }

    pub fn read_last_set_schedule(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setSchedule")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 列車が指定駅に到達可能かどうかを取得する。
    /// 戻り値は (reachable, reason) のタプル。
    pub fn book_next_can_train_reach(&mut self, dest: &str) {
        let args = msgpack::array(&[msgpack::str(dest)]);
        peripheral::book_request(self.addr, "canTrainReach", &args);
    }

    pub fn read_last_can_train_reach(&self) -> Result<(bool, Option<String>), PeripheralError> {
        let data = peripheral::read_result(self.addr, "canTrainReach")?;
        peripheral::decode(&data)
    }

    /// 列車が指定駅に到達可能かどうかを即時取得する (imm 対応)。
    pub fn can_train_reach_imm(
        &self,
        dest: &str,
    ) -> Result<(bool, Option<String>), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(dest)]);
        let data =
            peripheral::request_info_imm(self.addr, "canTrainReach", &args)?;
        peripheral::decode(&data)
    }

    /// 指定駅までの距離を取得する。
    /// 戻り値は (distance, reason) のタプル。
    pub fn book_next_distance_to(&mut self, dest: &str) {
        let args = msgpack::array(&[msgpack::str(dest)]);
        peripheral::book_request(self.addr, "distanceTo", &args);
    }

    pub fn read_last_distance_to(&self) -> Result<(Option<f64>, Option<String>), PeripheralError> {
        let data = peripheral::read_result(self.addr, "distanceTo")?;
        peripheral::decode(&data)
    }

    /// 指定駅までの距離を即時取得する (imm 対応)。
    pub fn distance_to_imm(
        &self,
        dest: &str,
    ) -> Result<(Option<f64>, Option<String>), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(dest)]);
        let data =
            peripheral::request_info_imm(self.addr, "distanceTo", &args)?;
        peripheral::decode(&data)
    }

    // ====== イベント系 / Events ======

    /// 列車到着イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_train_arrive(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_train_arrive",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_train_arrive(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_train_arrive")?;
        peripheral::decode(&data)
    }

    /// 列車到着イベントを受信するまで待機する。
    pub async fn pull_train_arrive(&self) -> Result<(), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_train_arrive", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_train_arrive")?;
            let result: Option<()> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }

    /// 列車出発イベントを 1tick 待機して取得する。来なければ None。
    pub fn book_next_try_pull_train_depart(&mut self) {
        peripheral::book_request(
            self.addr,
            "try_pull_train_depart",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_try_pull_train_depart(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_train_depart")?;
        peripheral::decode(&data)
    }

    /// 列車出発イベントを受信するまで待機する。
    pub async fn pull_train_depart(&self) -> Result<(), PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_train_depart", &msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_train_depart")?;
            let result: Option<()> = peripheral::decode(&data)?;
            if let Some(v) = result {
                return Ok(v);
            }
        }
    }
}

impl Station {
    // ─── async_* バリアント ──────────────────────────────────

    pub async fn async_assemble(&mut self) -> Vec<Result<(), PeripheralError>> {
        self.book_next_assemble();
        crate::wait_for_next_tick().await;
        self.read_last_assemble()
    }

    pub async fn async_disassemble(&mut self) -> Vec<Result<(), PeripheralError>> {
        self.book_next_disassemble();
        crate::wait_for_next_tick().await;
        self.read_last_disassemble()
    }

    pub async fn async_set_assembly_mode(&mut self, mode: bool) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_assembly_mode(mode);
        crate::wait_for_next_tick().await;
        self.read_last_set_assembly_mode()
    }

    pub async fn async_is_in_assembly_mode(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_in_assembly_mode();
        crate::wait_for_next_tick().await;
        self.read_last_is_in_assembly_mode()
    }

    pub async fn async_get_station_name(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_station_name();
        crate::wait_for_next_tick().await;
        self.read_last_get_station_name()
    }

    pub async fn async_set_station_name(&mut self, name: &str) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_station_name(name);
        crate::wait_for_next_tick().await;
        self.read_last_set_station_name()
    }

    pub async fn async_is_train_present(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_train_present();
        crate::wait_for_next_tick().await;
        self.read_last_is_train_present()
    }

    pub async fn async_is_train_imminent(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_train_imminent();
        crate::wait_for_next_tick().await;
        self.read_last_is_train_imminent()
    }

    pub async fn async_is_train_enroute(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_train_enroute();
        crate::wait_for_next_tick().await;
        self.read_last_is_train_enroute()
    }

    pub async fn async_get_train_name(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_train_name();
        crate::wait_for_next_tick().await;
        self.read_last_get_train_name()
    }

    pub async fn async_set_train_name(&mut self, name: &str) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_train_name(name);
        crate::wait_for_next_tick().await;
        self.read_last_set_train_name()
    }

    pub async fn async_has_schedule(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_has_schedule();
        crate::wait_for_next_tick().await;
        self.read_last_has_schedule()
    }

    pub async fn async_get_schedule(&mut self) -> Result<BTreeMap<String, Value>, PeripheralError> {
        self.book_next_get_schedule();
        crate::wait_for_next_tick().await;
        self.read_last_get_schedule()
    }

    pub async fn async_set_schedule(&mut self, schedule: &BTreeMap<String, Value>) -> Vec<Result<(), PeripheralError>> {
        let _ = self.book_next_set_schedule(schedule);
        crate::wait_for_next_tick().await;
        self.read_last_set_schedule()
    }

    pub async fn async_can_train_reach(&mut self, dest: &str) -> Result<(bool, Option<String>), PeripheralError> {
        self.book_next_can_train_reach(dest);
        crate::wait_for_next_tick().await;
        self.read_last_can_train_reach()
    }

    pub async fn async_distance_to(&mut self, dest: &str) -> Result<(Option<f64>, Option<String>), PeripheralError> {
        self.book_next_distance_to(dest);
        crate::wait_for_next_tick().await;
        self.read_last_distance_to()
    }
}
