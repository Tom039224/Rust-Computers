//! Clockwork CC Compat Boiler。

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

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
    addr: PeriphAddr,
}

impl Peripheral for Boiler {
    const NAME: &'static str = "Create_Boiler";

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

impl Boiler {
    imm_getter!(book_next_is_active, read_last_is_active, is_active_imm, "isActive", bool);
    imm_getter!(book_next_get_heat_level, read_last_get_heat_level, get_heat_level_imm, "getHeatLevel", f64);
    imm_getter!(book_next_get_active_heat, read_last_get_active_heat, get_active_heat_imm, "getActiveHeat", f64);
    imm_getter!(book_next_is_passive_heat, read_last_is_passive_heat, is_passive_heat_imm, "isPassiveHeat", bool);
    imm_getter!(book_next_get_water_supply, read_last_get_water_supply, get_water_supply_imm, "getWaterSupply", f64);
    imm_getter!(book_next_get_attached_engines, read_last_get_attached_engines, get_attached_engines_imm, "getAttachedEngines", u32);
    imm_getter!(book_next_get_attached_whistles, read_last_get_attached_whistles, get_attached_whistles_imm, "getAttachedWhistles", u32);
    imm_getter!(book_next_get_engine_efficiency, read_last_get_engine_efficiency, get_engine_efficiency_imm, "getEngineEfficiency", f64);
    imm_getter!(book_next_get_boiler_size, read_last_get_boiler_size, get_boiler_size_imm, "getBoilerSize", f64);
    imm_getter!(book_next_get_width, read_last_get_width, get_width_imm, "getWidth", u32);
    imm_getter!(book_next_get_height, read_last_get_height, get_height_imm, "getHeight", u32);
    imm_getter!(book_next_get_max_heat_for_size, read_last_get_max_heat_for_size, get_max_heat_for_size_imm, "getMaxHeatForSize", f64);
    imm_getter!(book_next_get_max_heat_for_water, read_last_get_max_heat_for_water, get_max_heat_for_water_imm, "getMaxHeatForWater", f64);
    imm_getter!(book_next_get_fill_state, read_last_get_fill_state, get_fill_state_imm, "getFillState", f64);
    imm_getter!(book_next_get_fluid_contents, read_last_get_fluid_contents, get_fluid_contents_imm, "getFluidContents", CLFluidInfo);
    imm_getter!(book_next_get_controller_pos, read_last_get_controller_pos, get_controller_pos_imm, "getControllerPos", CLPosition);
}

impl Boiler {
    pub async fn async_is_active(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_active();
        crate::wait_for_next_tick().await;
        self.read_last_is_active()
    }

    pub async fn async_get_heat_level(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_heat_level();
        crate::wait_for_next_tick().await;
        self.read_last_get_heat_level()
    }

    pub async fn async_get_active_heat(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_active_heat();
        crate::wait_for_next_tick().await;
        self.read_last_get_active_heat()
    }

    pub async fn async_is_passive_heat(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_passive_heat();
        crate::wait_for_next_tick().await;
        self.read_last_is_passive_heat()
    }

    pub async fn async_get_water_supply(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_water_supply();
        crate::wait_for_next_tick().await;
        self.read_last_get_water_supply()
    }

    pub async fn async_get_attached_engines(&mut self) -> Result<u32, PeripheralError> {
        self.book_next_get_attached_engines();
        crate::wait_for_next_tick().await;
        self.read_last_get_attached_engines()
    }

    pub async fn async_get_attached_whistles(&mut self) -> Result<u32, PeripheralError> {
        self.book_next_get_attached_whistles();
        crate::wait_for_next_tick().await;
        self.read_last_get_attached_whistles()
    }

    pub async fn async_get_engine_efficiency(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_engine_efficiency();
        crate::wait_for_next_tick().await;
        self.read_last_get_engine_efficiency()
    }

    pub async fn async_get_boiler_size(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_boiler_size();
        crate::wait_for_next_tick().await;
        self.read_last_get_boiler_size()
    }

    pub async fn async_get_width(&mut self) -> Result<u32, PeripheralError> {
        self.book_next_get_width();
        crate::wait_for_next_tick().await;
        self.read_last_get_width()
    }

    pub async fn async_get_height(&mut self) -> Result<u32, PeripheralError> {
        self.book_next_get_height();
        crate::wait_for_next_tick().await;
        self.read_last_get_height()
    }

    pub async fn async_get_max_heat_for_size(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_max_heat_for_size();
        crate::wait_for_next_tick().await;
        self.read_last_get_max_heat_for_size()
    }

    pub async fn async_get_max_heat_for_water(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_max_heat_for_water();
        crate::wait_for_next_tick().await;
        self.read_last_get_max_heat_for_water()
    }

    pub async fn async_get_fill_state(&mut self) -> Result<f64, PeripheralError> {
        self.book_next_get_fill_state();
        crate::wait_for_next_tick().await;
        self.read_last_get_fill_state()
    }

    pub async fn async_get_fluid_contents(&mut self) -> Result<CLFluidInfo, PeripheralError> {
        self.book_next_get_fluid_contents();
        crate::wait_for_next_tick().await;
        self.read_last_get_fluid_contents()
    }

    pub async fn async_get_controller_pos(&mut self) -> Result<CLPosition, PeripheralError> {
        self.book_next_get_controller_pos();
        crate::wait_for_next_tick().await;
        self.read_last_get_controller_pos()
    }
}
