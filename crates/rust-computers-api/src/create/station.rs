//! Create Station ペリフェラル。
//! Create Station peripheral.

use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::msgpack::Value;
use crate::peripheral::{self, Direction, Peripheral};

/// Station ペリフェラル。
pub struct Station {
    dir: Direction,
}

impl Peripheral for Station {
    const NAME: &'static str = "create:station";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl Station {
    /// 列車を組み立てる。
    pub async fn assemble(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "assemble", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// 列車を分解する。
    pub async fn disassemble(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "disassemble", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// 組み立てモードを設定する。
    pub async fn set_assembly_mode(&self, mode: bool) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::bool_val(mode)]);
        peripheral::do_action(self.dir, "setAssemblyMode", &args).await?;
        Ok(())
    }

    /// 組み立てモードかどうかを取得する。
    pub async fn is_in_assembly_mode(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "isInAssemblyMode",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 組み立てモードかどうかを即時取得する (imm 対応)。
    pub fn is_in_assembly_mode_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "isInAssemblyMode",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ステーション名を取得する。
    pub async fn get_station_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getStationName",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// ステーション名を即時取得する (imm 対応)。
    pub fn get_station_name_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getStationName",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ステーション名を設定する。
    pub async fn set_station_name(&self, name: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(name)]);
        peripheral::do_action(self.dir, "setStationName", &args).await?;
        Ok(())
    }

    /// 列車が存在するかどうかを取得する。
    pub async fn is_train_present(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "isTrainPresent",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車が存在するかどうかを即時取得する (imm 対応)。
    pub fn is_train_present_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "isTrainPresent",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車が間もなく到着するかどうかを取得する。
    pub async fn is_train_imminent(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "isTrainImminent",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車が間もなく到着するかどうかを即時取得する (imm 対応)。
    pub fn is_train_imminent_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "isTrainImminent",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車が経路上にあるかどうかを取得する。
    pub async fn is_train_enroute(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "isTrainEnroute",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車が経路上にあるかどうかを即時取得する (imm 対応)。
    pub fn is_train_enroute_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "isTrainEnroute",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車名を取得する。
    pub async fn get_train_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getTrainName",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車名を即時取得する (imm 対応)。
    pub fn get_train_name_imm(&self) -> Result<String, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getTrainName",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 列車名を設定する。
    pub async fn set_train_name(&self, name: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(name)]);
        peripheral::do_action(self.dir, "setTrainName", &args).await?;
        Ok(())
    }

    /// スケジュールが存在するかどうかを取得する。
    pub async fn has_schedule(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "hasSchedule",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// スケジュールが存在するかどうかを即時取得する (imm 対応)。
    pub fn has_schedule_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "hasSchedule",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// スケジュールを取得する。
    pub async fn get_schedule(
        &self,
    ) -> Result<BTreeMap<String, Value>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getSchedule",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// スケジュールを即時取得する (imm 対応)。
    pub fn get_schedule_imm(
        &self,
    ) -> Result<BTreeMap<String, Value>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getSchedule",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// スケジュールを設定する。
    pub async fn set_schedule(
        &self,
        schedule: &BTreeMap<String, Value>,
    ) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(schedule)?;
        let args = msgpack::array(&[encoded]);
        peripheral::do_action(self.dir, "setSchedule", &args).await?;
        Ok(())
    }

    /// 列車が指定駅に到達可能かどうかを取得する。
    /// 戻り値は (reachable, reason) のタプル。
    pub async fn can_train_reach(
        &self,
        dest: &str,
    ) -> Result<(bool, Option<String>), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(dest)]);
        let data =
            peripheral::request_info(self.dir, "canTrainReach", &args).await?;
        peripheral::decode(&data)
    }

    /// 列車が指定駅に到達可能かどうかを即時取得する (imm 対応)。
    pub fn can_train_reach_imm(
        &self,
        dest: &str,
    ) -> Result<(bool, Option<String>), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(dest)]);
        let data =
            peripheral::request_info_imm(self.dir, "canTrainReach", &args)?;
        peripheral::decode(&data)
    }

    /// 指定駅までの距離を取得する。
    /// 戻り値は (distance, reason) のタプル。
    pub async fn distance_to(
        &self,
        dest: &str,
    ) -> Result<(Option<f64>, Option<String>), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(dest)]);
        let data =
            peripheral::request_info(self.dir, "distanceTo", &args).await?;
        peripheral::decode(&data)
    }

    /// 指定駅までの距離を即時取得する (imm 対応)。
    pub fn distance_to_imm(
        &self,
        dest: &str,
    ) -> Result<(Option<f64>, Option<String>), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(dest)]);
        let data =
            peripheral::request_info_imm(self.dir, "distanceTo", &args)?;
        peripheral::decode(&data)
    }

    // ====== イベント系 / Events ======

    /// 列車到着イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_train_arrive(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "try_pull_train_arrive",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車到着イベントを受信するまで待機する。
    pub async fn pull_train_arrive(&self) -> Result<(), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_train_arrive().await? {
                return Ok(v);
            }
        }
    }

    /// 列車出発イベントを 1tick 待機して取得する。来なければ None。
    pub async fn try_pull_train_depart(&self) -> Result<Option<()>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "try_pull_train_depart",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 列車出発イベントを受信するまで待機する。
    pub async fn pull_train_depart(&self) -> Result<(), PeripheralError> {
        loop {
            if let Some(v) = self.try_pull_train_depart().await? {
                return Ok(v);
            }
        }
    }
}
