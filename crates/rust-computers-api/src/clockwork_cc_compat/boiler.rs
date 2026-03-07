//! Clockwork CC Compat Boiler。

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// 流体情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLFluidInfo {
    pub fluid: alloc::string::String,
    pub amount: u32,
    pub capacity: u32,
}

/// 座標。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Boiler ペリフェラル。
pub struct Boiler {
    dir: Direction,
}

impl Peripheral for Boiler {
    const NAME: &'static str = "clockwork:boiler";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

macro_rules! imm_getter {
    ($fn_async:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub async fn $fn_async(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info(self.dir, $method, &msgpack::array(&[])).await?;
            peripheral::decode(&data)
        }

        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.dir, $method, &msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl Boiler {
    imm_getter!(is_active, is_active_imm, "isActive", bool);
    imm_getter!(get_heat_level, get_heat_level_imm, "getHeatLevel", f64);
    imm_getter!(get_active_heat, get_active_heat_imm, "getActiveHeat", f64);
    imm_getter!(is_passive_heat, is_passive_heat_imm, "isPassiveHeat", bool);
    imm_getter!(get_water_supply, get_water_supply_imm, "getWaterSupply", f64);
    imm_getter!(get_attached_engines, get_attached_engines_imm, "getAttachedEngines", u32);
    imm_getter!(get_attached_whistles, get_attached_whistles_imm, "getAttachedWhistles", u32);
    imm_getter!(get_engine_efficiency, get_engine_efficiency_imm, "getEngineEfficiency", f64);
    imm_getter!(get_boiler_size, get_boiler_size_imm, "getBoilerSize", f64);
    imm_getter!(get_width, get_width_imm, "getWidth", u32);
    imm_getter!(get_height, get_height_imm, "getHeight", u32);
    imm_getter!(get_max_heat_for_size, get_max_heat_for_size_imm, "getMaxHeatForSize", f64);
    imm_getter!(get_max_heat_for_water, get_max_heat_for_water_imm, "getMaxHeatForWater", f64);
    imm_getter!(get_fill_state, get_fill_state_imm, "getFillState", f64);
    imm_getter!(get_fluid_contents, get_fluid_contents_imm, "getFluidContents", CLFluidInfo);
    imm_getter!(get_controller_pos, get_controller_pos_imm, "getControllerPos", CLPosition);
}
