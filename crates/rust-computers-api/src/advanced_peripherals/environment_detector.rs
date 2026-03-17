//! AdvancedPeripherals EnvironmentDetector。

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// エンティティ情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
    pub id: String,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "canFreeze", default)]
    pub can_freeze: Option<bool>,
    #[serde(rename = "isGlowing", default)]
    pub is_glowing: Option<bool>,
    #[serde(rename = "isInWall", default)]
    pub is_in_wall: Option<bool>,
    #[serde(default)]
    pub health: Option<f64>,
    #[serde(rename = "maxHealth", default)]
    pub max_health: Option<f64>,
    #[serde(rename = "lastDamageSource", default)]
    pub last_damage_source: Option<String>,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(default)]
    pub z: Option<f64>,
}

/// EnvironmentDetector ペリフェラル。
pub struct EnvironmentDetector {
    addr: PeriphAddr,
}

impl Peripheral for EnvironmentDetector {
    const NAME: &'static str = "environment_detector";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl EnvironmentDetector {
    // ─── 環境・天候 ──────────────────────────────────────────

    /// バイオーム ID を返す。
    pub fn book_next_get_biome(&mut self) {
        peripheral::book_request(self.addr, "getBiome", &msgpack::array(&[]));
    }
    pub fn read_last_get_biome(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBiome")?;
        peripheral::decode(&data)
    }

    /// ディメンション ID を返す。
    pub fn book_next_get_dimension(&mut self) {
        peripheral::book_request(self.addr, "getDimension", &msgpack::array(&[]));
    }
    pub fn read_last_get_dimension(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getDimension")?;
        peripheral::decode(&data)
    }

    /// 指定ディメンションにいるかを返す。
    pub fn book_next_is_dimension(&mut self, dim: &str) {
        let args = msgpack::array(&[msgpack::str(dim)]);
        peripheral::book_request(self.addr, "isDimension", &args);
    }
    pub fn read_last_is_dimension(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isDimension")?;
        peripheral::decode(&data)
    }

    /// 全ディメンション ID のリストを返す。
    pub fn book_next_list_dimensions(&mut self) {
        peripheral::book_request(self.addr, "listDimensions", &msgpack::array(&[]));
    }
    pub fn read_last_list_dimensions(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "listDimensions")?;
        peripheral::decode(&data)
    }

    /// 雨が降っているかを返す。
    pub fn book_next_is_raining(&mut self) {
        peripheral::book_request(self.addr, "isRaining", &msgpack::array(&[]));
    }
    pub fn read_last_is_raining(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isRaining")?;
        peripheral::decode(&data)
    }

    /// 雷雨かを返す。
    pub fn book_next_is_thunder(&mut self) {
        peripheral::book_request(self.addr, "isThunder", &msgpack::array(&[]));
    }
    pub fn read_last_is_thunder(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isThunder")?;
        peripheral::decode(&data)
    }

    /// 晴れかを返す。
    pub fn book_next_is_sunny(&mut self) {
        peripheral::book_request(self.addr, "isSunny", &msgpack::array(&[]));
    }
    pub fn read_last_is_sunny(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isSunny")?;
        peripheral::decode(&data)
    }

    // ─── 光・時間 ────────────────────────────────────────────

    /// 空のライトレベル (0〜15) を返す。
    pub fn book_next_get_sky_light_level(&mut self) {
        peripheral::book_request(self.addr, "getSkyLightLevel", &msgpack::array(&[]));
    }
    pub fn read_last_get_sky_light_level(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSkyLightLevel")?;
        peripheral::decode(&data)
    }

    /// ブロックライトレベル (0〜15) を返す。
    pub fn book_next_get_block_light_level(&mut self) {
        peripheral::book_request(self.addr, "getBlockLightLevel", &msgpack::array(&[]));
    }
    pub fn read_last_get_block_light_level(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBlockLightLevel")?;
        peripheral::decode(&data)
    }

    /// 昼光レベル (0〜15) を返す。
    pub fn book_next_get_day_light_level(&mut self) {
        peripheral::book_request(self.addr, "getDayLightLevel", &msgpack::array(&[]));
    }
    pub fn read_last_get_day_light_level(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getDayLightLevel")?;
        peripheral::decode(&data)
    }

    /// ワールド時間 (ticks) を返す。
    pub fn book_next_get_time(&mut self) {
        peripheral::book_request(self.addr, "getTime", &msgpack::array(&[]));
    }
    pub fn read_last_get_time(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTime")?;
        peripheral::decode(&data)
    }

    // ─── 月相 ────────────────────────────────────────────────

    /// 現在の月相 ID (0〜7) を返す。
    pub fn book_next_get_moon_id(&mut self) {
        peripheral::book_request(self.addr, "getMoonId", &msgpack::array(&[]));
    }
    pub fn read_last_get_moon_id(&self) -> Result<i32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMoonId")?;
        peripheral::decode(&data)
    }

    /// 月相名を返す。
    pub fn book_next_get_moon_name(&mut self) {
        peripheral::book_request(self.addr, "getMoonName", &msgpack::array(&[]));
    }
    pub fn read_last_get_moon_name(&self) -> Result<String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMoonName")?;
        peripheral::decode(&data)
    }

    /// 指定月相かどうかを返す。
    pub fn book_next_is_moon(&mut self, phase: &str) {
        let args = msgpack::array(&[msgpack::str(phase)]);
        peripheral::book_request(self.addr, "isMoon", &args);
    }
    pub fn read_last_is_moon(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isMoon")?;
        peripheral::decode(&data)
    }

    // ─── 地形 ────────────────────────────────────────────────

    /// スライムチャンクかどうかを返す。
    pub fn book_next_is_slime_chunk(&mut self) {
        peripheral::book_request(self.addr, "isSlimeChunk", &msgpack::array(&[]));
    }
    pub fn read_last_is_slime_chunk(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isSlimeChunk")?;
        peripheral::decode(&data)
    }

    // ─── 睡眠 ────────────────────────────────────────────────

    /// このディメンションでスリープ可能かを返す。
    pub fn book_next_can_sleep_here(&mut self) {
        peripheral::book_request(self.addr, "canSleepHere", &msgpack::array(&[]));
    }
    pub fn read_last_can_sleep_here(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "canSleepHere")?;
        peripheral::decode(&data)
    }

    /// 指定プレイヤーがスリープ可能かを返す。
    pub fn book_next_can_sleep_player(&mut self, name: &str) {
        let args = msgpack::array(&[msgpack::str(name)]);
        peripheral::book_request(self.addr, "canSleepPlayer", &args);
    }
    pub fn read_last_can_sleep_player(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "canSleepPlayer")?;
        peripheral::decode(&data)
    }

    // ─── エンティティスキャン ────────────────────────────────

    /// 指定半径内のエンティティ一覧を返す。
    pub fn book_next_scan_entities(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "scanEntities", &args);
    }
    pub fn read_last_scan_entities(&self) -> Result<Vec<EntityInfo>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "scanEntities")?;
        peripheral::decode(&data)
    }

    /// scanEntities の演算コストを返す (imm 対応)。
    pub fn book_next_scan_cost(&mut self, radius: f64) {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        peripheral::book_request(self.addr, "scanCost", &args);
    }
    pub fn read_last_scan_cost(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "scanCost")?;
        peripheral::decode(&data)
    }

    pub fn scan_cost_imm(&self, radius: f64) -> Result<f64, PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(radius)]);
        let data = peripheral::request_info_imm(self.addr, "scanCost", &args)?;
        peripheral::decode(&data)
    }
}

impl EnvironmentDetector {
    // ─── async_* バリアント ──────────────────────────────────

    pub async fn async_get_biome(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_biome();
        crate::wait_for_next_tick().await;
        self.read_last_get_biome()
    }

    pub async fn async_get_dimension(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_dimension();
        crate::wait_for_next_tick().await;
        self.read_last_get_dimension()
    }

    pub async fn async_is_dimension(&mut self, dim: &str) -> Result<bool, PeripheralError> {
        self.book_next_is_dimension(dim);
        crate::wait_for_next_tick().await;
        self.read_last_is_dimension()
    }

    pub async fn async_list_dimensions(&mut self) -> Result<Vec<String>, PeripheralError> {
        self.book_next_list_dimensions();
        crate::wait_for_next_tick().await;
        self.read_last_list_dimensions()
    }

    pub async fn async_is_raining(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_raining();
        crate::wait_for_next_tick().await;
        self.read_last_is_raining()
    }

    pub async fn async_is_thunder(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_thunder();
        crate::wait_for_next_tick().await;
        self.read_last_is_thunder()
    }

    pub async fn async_is_sunny(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_sunny();
        crate::wait_for_next_tick().await;
        self.read_last_is_sunny()
    }

    pub async fn async_get_sky_light_level(&mut self) -> Result<i32, PeripheralError> {
        self.book_next_get_sky_light_level();
        crate::wait_for_next_tick().await;
        self.read_last_get_sky_light_level()
    }

    pub async fn async_get_block_light_level(&mut self) -> Result<i32, PeripheralError> {
        self.book_next_get_block_light_level();
        crate::wait_for_next_tick().await;
        self.read_last_get_block_light_level()
    }

    pub async fn async_get_day_light_level(&mut self) -> Result<i32, PeripheralError> {
        self.book_next_get_day_light_level();
        crate::wait_for_next_tick().await;
        self.read_last_get_day_light_level()
    }

    pub async fn async_get_time(&mut self) -> Result<i64, PeripheralError> {
        self.book_next_get_time();
        crate::wait_for_next_tick().await;
        self.read_last_get_time()
    }

    pub async fn async_get_moon_id(&mut self) -> Result<i32, PeripheralError> {
        self.book_next_get_moon_id();
        crate::wait_for_next_tick().await;
        self.read_last_get_moon_id()
    }

    pub async fn async_get_moon_name(&mut self) -> Result<String, PeripheralError> {
        self.book_next_get_moon_name();
        crate::wait_for_next_tick().await;
        self.read_last_get_moon_name()
    }

    pub async fn async_is_moon(&mut self, phase: &str) -> Result<bool, PeripheralError> {
        self.book_next_is_moon(phase);
        crate::wait_for_next_tick().await;
        self.read_last_is_moon()
    }

    pub async fn async_is_slime_chunk(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_slime_chunk();
        crate::wait_for_next_tick().await;
        self.read_last_is_slime_chunk()
    }

    pub async fn async_can_sleep_here(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_can_sleep_here();
        crate::wait_for_next_tick().await;
        self.read_last_can_sleep_here()
    }

    pub async fn async_can_sleep_player(&mut self, name: &str) -> Result<bool, PeripheralError> {
        self.book_next_can_sleep_player(name);
        crate::wait_for_next_tick().await;
        self.read_last_can_sleep_player()
    }

    pub async fn async_scan_entities(&mut self, radius: f64) -> Result<Vec<EntityInfo>, PeripheralError> {
        self.book_next_scan_entities(radius);
        crate::wait_for_next_tick().await;
        self.read_last_scan_entities()
    }

    pub async fn async_scan_cost(&mut self, radius: f64) -> Result<f64, PeripheralError> {
        self.book_next_scan_cost(radius);
        crate::wait_for_next_tick().await;
        self.read_last_scan_cost()
    }
}
