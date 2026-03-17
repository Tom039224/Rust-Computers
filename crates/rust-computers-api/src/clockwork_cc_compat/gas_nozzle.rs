//! Clockwork CC Compat GasNozzle。

use serde::{Deserialize, Serialize};
use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// リーク情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeakInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// GasNozzle ペリフェラル。
pub struct GasNozzle {
    addr: PeriphAddr,
}

impl Peripheral for GasNozzle {
    const NAME: &'static str = "cw_gas_nozzle";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

macro_rules! imm_getter {
    ($fn_book:ident, $fn_read:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub fn $fn_book(&mut self) {
            peripheral::book_request(self.addr, $method, &msgpack::array(&[]));
        }

        pub fn $fn_read(&self) -> Result<$ret, PeripheralError> {
            let data = peripheral::read_result(self.addr, $method)?;
            peripheral::decode(&data)
        }

        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.addr, $method, &msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl GasNozzle {
    // ---- 固有メソッド (setter) ----

    /// ポインター値（注入率制御）を設定する。
    pub fn book_next_set_pointer(&mut self, value: f64) {
        let args = msgpack::array(&[msgpack::float64(value)]);
        peripheral::book_action(self.addr, "setPointer", &args);
    }
    pub fn read_last_set_pointer(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setPointer")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    // ---- 固有メソッド (imm getters) ----

    imm_getter!(book_next_get_pointer, read_last_get_pointer, get_pointer_imm, "getPointer", f64);
    imm_getter!(book_next_get_pointer_speed, read_last_get_pointer_speed, get_pointer_speed_imm, "getPointerSpeed", f64);
    imm_getter!(book_next_get_pocket_temperature, read_last_get_pocket_temperature, get_pocket_temperature_imm, "getPocketTemperature", f64);
    imm_getter!(book_next_get_duct_temperature, read_last_get_duct_temperature, get_duct_temperature_imm, "getDuctTemperature", f64);
    imm_getter!(book_next_get_target_temperature, read_last_get_target_temperature, get_target_temperature_imm, "getTargetTemperature", f64);
    imm_getter!(book_next_get_balloon_volume, read_last_get_balloon_volume, get_balloon_volume_imm, "getBalloonVolume", f64);
    imm_getter!(book_next_get_leaks, read_last_get_leaks, get_leaks_imm, "getLeaks", alloc::vec::Vec<LeakInfo>);
    imm_getter!(book_next_get_temperature_delta, read_last_get_temperature_delta, get_temperature_delta_imm, "getTemperatureDelta", f64);
    imm_getter!(book_next_has_balloon, read_last_has_balloon, has_balloon_imm, "hasBalloon", bool);

    // ---- 固有メソッド (non-imm getters) ----

    /// 現在の浮力 (N) を予約する。
    pub fn book_next_get_buoyancy_force(&mut self) {
        peripheral::book_request(self.addr, "getBuoyancyForce", &msgpack::array(&[]));
    }
    pub fn read_last_get_buoyancy_force(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBuoyancyForce")?;
        peripheral::decode(&data)
    }

    /// 気球内圧力を予約する。
    pub fn book_next_get_balloon_pressure(&mut self) {
        peripheral::book_request(self.addr, "getBalloonPressure", &msgpack::array(&[]));
    }
    pub fn read_last_get_balloon_pressure(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBalloonPressure")?;
        peripheral::decode(&data)
    }

    /// 気球内ガス名→質量テーブルを予約する。
    pub fn book_next_get_balloon_gas_contents(&mut self) {
        peripheral::book_request(self.addr, "getBalloonGasContents", &msgpack::array(&[]));
    }
    pub fn read_last_get_balloon_gas_contents(&self) -> Result<alloc::collections::BTreeMap<alloc::string::String, f64>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBalloonGasContents")?;
        peripheral::decode(&data)
    }

    /// ガス損失レート (kg/s) を予約する。
    pub fn book_next_get_loss_rate(&mut self) {
        peripheral::book_request(self.addr, "getLossRate", &msgpack::array(&[]));
    }
    pub fn read_last_get_loss_rate(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getLossRate")?;
        peripheral::decode(&data)
    }

    /// ガス流入レート (kg/s) を予約する。
    pub fn book_next_get_inflow_rate(&mut self) {
        peripheral::book_request(self.addr, "getInflowRate", &msgpack::array(&[]));
    }
    pub fn read_last_get_inflow_rate(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getInflowRate")?;
        peripheral::decode(&data)
    }

    /// 気球に必要だが欠けているブロック座標リストを予約する。
    pub fn book_next_get_missing_positions(&mut self) {
        peripheral::book_request(self.addr, "getMissingPositions", &msgpack::array(&[]));
    }
    pub fn read_last_get_missing_positions(&self) -> Result<alloc::vec::Vec<super::CLPosition>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMissingPositions")?;
        peripheral::decode(&data)
    }

    /// 気球内の全ガス質量 (kg) を予約する。
    pub fn book_next_get_total_gas_mass(&mut self) {
        peripheral::book_request(self.addr, "getTotalGasMass", &msgpack::array(&[]));
    }
    pub fn read_last_get_total_gas_mass(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTotalGasMass")?;
        peripheral::decode(&data)
    }

    /// リーク耐性（0〜1）を予約する。
    pub fn book_next_get_leak_integrity(&mut self) {
        peripheral::book_request(self.addr, "getLeakIntegrity", &msgpack::array(&[]));
    }
    pub fn read_last_get_leak_integrity(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getLeakIntegrity")?;
        peripheral::decode(&data)
    }

    /// 許容最大リーク数を予約する。
    pub fn book_next_get_max_leaks(&mut self) {
        peripheral::book_request(self.addr, "getMaxLeaks", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_leaks(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxLeaks")?;
        peripheral::decode(&data)
    }

    /// 気球内部の密度を予約する。
    pub fn book_next_get_internal_density(&mut self) {
        peripheral::book_request(self.addr, "getInternalDensity", &msgpack::array(&[]));
    }
    pub fn read_last_get_internal_density(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getInternalDensity")?;
        peripheral::decode(&data)
    }

    // ---- GasNetwork 共通メソッド ----

    super::gas_network::gas_network_methods!();
}

impl GasNozzle {
    pub async fn async_set_pointer(&mut self, value: f64) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_pointer(value);
        crate::wait_for_next_tick().await;
        self.read_last_set_pointer()
    }

    pub async fn async_get_pointer(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_pointer();
        crate::wait_for_next_tick().await;
        self.read_last_get_pointer()
    }

    pub async fn async_get_pointer_speed(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_pointer_speed();
        crate::wait_for_next_tick().await;
        self.read_last_get_pointer_speed()
    }

    pub async fn async_get_pocket_temperature(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_pocket_temperature();
        crate::wait_for_next_tick().await;
        self.read_last_get_pocket_temperature()
    }

    pub async fn async_get_duct_temperature(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_duct_temperature();
        crate::wait_for_next_tick().await;
        self.read_last_get_duct_temperature()
    }

    pub async fn async_get_target_temperature(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_target_temperature();
        crate::wait_for_next_tick().await;
        self.read_last_get_target_temperature()
    }

    pub async fn async_get_balloon_volume(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_balloon_volume();
        crate::wait_for_next_tick().await;
        self.read_last_get_balloon_volume()
    }

    pub async fn async_get_leaks(&mut self) -> Result<alloc::vec::Vec<LeakInfo>, PeripheralError> {
        self.book_next_get_leaks();
        crate::wait_for_next_tick().await;
        self.read_last_get_leaks()
    }

    pub async fn async_get_temperature_delta(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_temperature_delta();
        crate::wait_for_next_tick().await;
        self.read_last_get_temperature_delta()
    }

    pub async fn async_has_balloon(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_has_balloon();
        crate::wait_for_next_tick().await;
        self.read_last_has_balloon()
    }

    pub async fn async_get_buoyancy_force(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_buoyancy_force();
        crate::wait_for_next_tick().await;
        self.read_last_get_buoyancy_force()
    }

    pub async fn async_get_balloon_pressure(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_balloon_pressure();
        crate::wait_for_next_tick().await;
        self.read_last_get_balloon_pressure()
    }

    pub async fn async_get_balloon_gas_contents(&mut self) -> Result<alloc::collections::BTreeMap<alloc::string::String, f64>, PeripheralError> {
        self.book_next_get_balloon_gas_contents();
        crate::wait_for_next_tick().await;
        self.read_last_get_balloon_gas_contents()
    }

    pub async fn async_get_loss_rate(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_loss_rate();
        crate::wait_for_next_tick().await;
        self.read_last_get_loss_rate()
    }

    pub async fn async_get_inflow_rate(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_inflow_rate();
        crate::wait_for_next_tick().await;
        self.read_last_get_inflow_rate()
    }

    pub async fn async_get_missing_positions(&mut self) -> Result<alloc::vec::Vec<super::CLPosition>, PeripheralError> {
        self.book_next_get_missing_positions();
        crate::wait_for_next_tick().await;
        self.read_last_get_missing_positions()
    }

    pub async fn async_get_total_gas_mass(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_total_gas_mass();
        crate::wait_for_next_tick().await;
        self.read_last_get_total_gas_mass()
    }

    pub async fn async_get_leak_integrity(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_leak_integrity();
        crate::wait_for_next_tick().await;
        self.read_last_get_leak_integrity()
    }

    pub async fn async_get_max_leaks(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_max_leaks();
        crate::wait_for_next_tick().await;
        self.read_last_get_max_leaks()
    }

    pub async fn async_get_internal_density(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_internal_density();
        crate::wait_for_next_tick().await;
        self.read_last_get_internal_density()
    }

    pub async fn async_get_temperature(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_temperature();
        crate::wait_for_next_tick().await;
        self.read_last_get_temperature()
    }

    pub async fn async_get_pressure(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_pressure();
        crate::wait_for_next_tick().await;
        self.read_last_get_pressure()
    }

    pub async fn async_get_heat_energy(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_heat_energy();
        crate::wait_for_next_tick().await;
        self.read_last_get_heat_energy()
    }

    pub async fn async_get_gas_mass(&mut self) -> Result<alloc::collections::BTreeMap<alloc::string::String, f64>, PeripheralError> {
        self.book_next_get_gas_mass();
        crate::wait_for_next_tick().await;
        self.read_last_get_gas_mass()
    }

    pub async fn async_get_position(&mut self) -> Result<super::CLPosition, PeripheralError> {
        self.book_next_get_position();
        crate::wait_for_next_tick().await;
        self.read_last_get_position()
    }

    pub async fn async_get_network_info(&mut self) -> Result<crate::msgpack::Value, PeripheralError> {
        self.book_next_get_network_info();
        crate::wait_for_next_tick().await;
        self.read_last_get_network_info()
    }
}
