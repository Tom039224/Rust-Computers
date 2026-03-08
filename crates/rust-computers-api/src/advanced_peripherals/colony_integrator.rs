//! AdvancedPeripherals ColonyIntegrator。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// 3D 座標。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonyPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 市民情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenInfo {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub job: Option<String>,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(default)]
    pub health: Option<f64>,
    #[serde(rename = "maxHealth", default)]
    pub max_health: Option<f64>,
    #[serde(default)]
    pub happiness: Option<f64>,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(default)]
    pub z: Option<f64>,
    #[serde(rename = "bedPos", default)]
    pub bed_pos: Option<ColonyPosition>,
}

/// 建物情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingInfo {
    #[serde(rename = "type", default)]
    pub building_type: Option<String>,
    #[serde(default)]
    pub location: Option<ColonyPosition>,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(rename = "maxLevel", default)]
    pub max_level: Option<i32>,
    #[serde(default)]
    pub style: Option<String>,
}

/// ワークオーダー情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrderInfo {
    pub id: i32,
    #[serde(rename = "type", default)]
    pub order_type: Option<String>,
    #[serde(default)]
    pub builder: Option<ColonyPosition>,
    #[serde(default)]
    pub location: Option<ColonyPosition>,
    #[serde(default)]
    pub priority: Option<i32>,
}

/// ColonyIntegrator ペリフェラル。
pub struct ColonyIntegrator {
    addr: PeriphAddr,
}

impl Peripheral for ColonyIntegrator {
    const NAME: &'static str = "advancedPeripherals:colony_integrator";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl ColonyIntegrator {
    // ─── コロニー情報 ────────────────────────────────────────

    /// コロニー内にあるかを返す。
    pub fn book_next_is_in_colony(&mut self) {
        peripheral::book_request(self.addr, "isInColony", &msgpack::array(&[]));
    }
    pub fn read_last_is_in_colony(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isInColony")?;
        peripheral::decode(&data)
    }

    /// コロニー ID を返す。
    pub fn book_next_get_colony_id(&mut self) {
        peripheral::book_request(self.addr, "getColonyID", &msgpack::array(&[]));
    }
    pub fn read_last_get_colony_id(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getColonyID")?;
        peripheral::decode(&data)
    }

    /// コロニー名を返す。
    pub fn book_next_get_colony_name(&mut self) {
        peripheral::book_request(self.addr, "getColonyName", &msgpack::array(&[]));
    }
    pub fn read_last_get_colony_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getColonyName")?;
        peripheral::decode(&data)
    }

    /// コロニーのスタイルを返す。
    pub fn book_next_get_colony_style(&mut self) {
        peripheral::book_request(self.addr, "getColonyStyle", &msgpack::array(&[]));
    }
    pub fn read_last_get_colony_style(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getColonyStyle")?;
        peripheral::decode(&data)
    }

    /// コロニーがアクティブかを返す。
    pub fn book_next_is_active(&mut self) {
        peripheral::book_request(self.addr, "isActive", &msgpack::array(&[]));
    }
    pub fn read_last_is_active(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isActive")?;
        peripheral::decode(&data)
    }

    /// 現在の市民数を返す。
    pub fn book_next_get_amount_of_citizens(&mut self) {
        peripheral::book_request(self.addr, "getAmountOfCitizens", &msgpack::array(&[]));
    }
    pub fn read_last_get_amount_of_citizens(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAmountOfCitizens")?;
        peripheral::decode(&data)
    }

    /// 最大市民数を返す。
    pub fn book_next_get_max_citizens(&mut self) {
        peripheral::book_request(self.addr, "getMaxCitizens", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_citizens(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxCitizens")?;
        peripheral::decode(&data)
    }

    /// コロニーの幸福度を返す。
    pub fn book_next_get_happiness(&mut self) {
        peripheral::book_request(self.addr, "getHappiness", &msgpack::array(&[]));
    }
    pub fn read_last_get_happiness(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getHappiness")?;
        peripheral::decode(&data)
    }

    /// タウンホール座標を返す。
    pub fn book_next_get_position(&mut self) {
        peripheral::book_request(self.addr, "getPosition", &msgpack::array(&[]));
    }
    pub fn read_last_get_position(&self) -> Result<ColonyPosition, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getPosition")?;
        peripheral::decode(&data)
    }

    // ─── 市民 ────────────────────────────────────────────────

    /// 全市民の情報リストを返す。
    pub fn book_next_get_citizens(&mut self) {
        peripheral::book_request(self.addr, "getCitizens", &msgpack::array(&[]));
    }
    pub fn read_last_get_citizens(&self) -> Result<Vec<CitizenInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getCitizens")?;
        peripheral::decode(&data)
    }

    /// 指定 ID の市民情報を返す。
    pub fn book_next_get_citizen_info(&mut self, id: i32) {
        let args = msgpack::array(&[msgpack::int(id)]);
        peripheral::book_request(self.addr, "getCitizenInfo", &args);
    }
    pub fn read_last_get_citizen_info(&self) -> Result<CitizenInfo, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getCitizenInfo")?;
        peripheral::decode(&data)
    }

    // ─── 建物 ────────────────────────────────────────────────

    /// 全建物の情報リストを返す。
    pub fn book_next_get_buildings(&mut self) {
        peripheral::book_request(self.addr, "getBuildings", &msgpack::array(&[]));
    }
    pub fn read_last_get_buildings(&self) -> Result<Vec<BuildingInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBuildings")?;
        peripheral::decode(&data)
    }

    /// 指定座標の建物情報を返す。
    pub fn book_next_get_building_info(&mut self, x: f64, y: f64, z: f64) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_request(self.addr, "getBuildingInfo", &args);
    }
    pub fn read_last_get_building_info(&self) -> Result<BuildingInfo, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBuildingInfo")?;
        peripheral::decode(&data)
    }

    // ─── ワークオーダー・リクエスト ──────────────────────────

    /// ワークオーダー一覧を返す。
    pub fn book_next_get_work_orders(&mut self) {
        peripheral::book_request(self.addr, "getWorkOrders", &msgpack::array(&[]));
    }
    pub fn read_last_get_work_orders(&self) -> Result<Vec<WorkOrderInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getWorkOrders")?;
        peripheral::decode(&data)
    }

    /// 未解決リクエスト一覧を返す。
    pub fn book_next_get_requests(&mut self) {
        peripheral::book_request(self.addr, "getRequests", &msgpack::array(&[]));
    }
    pub fn read_last_get_requests(&self) -> Result<msgpack::Value, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getRequests")?;
        peripheral::decode(&data)
    }

    /// 指定ビルダーに必要なリソース一覧を返す。
    pub fn book_next_get_builder_resources(&mut self, x: f64, y: f64, z: f64) {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::book_request(self.addr, "getBuilderResources", &args);
    }
    pub fn read_last_get_builder_resources(&self) -> Result<msgpack::Value, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBuilderResources")?;
        peripheral::decode(&data)
    }

    // ─── 攻撃 ────────────────────────────────────────────────

    /// コロニーが攻撃を受けているかを返す。
    pub fn book_next_is_under_attack(&mut self) {
        peripheral::book_request(self.addr, "isUnderAttack", &msgpack::array(&[]));
    }
    pub fn read_last_is_under_attack(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isUnderAttack")?;
        peripheral::decode(&data)
    }
}
