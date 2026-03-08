//! Clockwork CC Compat GasNozzle。

use serde::{Deserialize, Serialize};

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
    const NAME: &'static str = "clockwork:gas_nozzle";

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
    pub fn read_last_set_pointer(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setPointer")?;
        Ok(())
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
