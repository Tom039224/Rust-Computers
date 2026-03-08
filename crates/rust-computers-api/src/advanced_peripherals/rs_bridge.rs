//! AdvancedPeripherals RSBridge。
//! MEBridge とほぼ同一のインターフェースを持つ Refined Storage 版。

use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

// 型は me_bridge モジュールから再利用する。
pub use super::me_bridge::{MEChemicalEntry, MEFluidEntry, MEItemEntry};

/// RSBridge ペリフェラル。
pub struct RSBridge {
    addr: PeriphAddr,
}

impl Peripheral for RSBridge {
    const NAME: &'static str = "advancedPeripherals:rs_bridge";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl RSBridge {
    // ─── アイテム操作 ────────────────────────────────────────

    /// ネットワーク内の全アイテム一覧を返す。
    pub fn book_next_list_items(&mut self) {
        peripheral::book_request(self.addr, "listItems", &msgpack::array(&[]));
    }
    pub fn read_last_list_items(&self) -> Result<Vec<MEItemEntry>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "listItems")?;
        peripheral::decode(&data)
    }

    /// 条件に合う最初のアイテムを返す。
    pub fn book_next_get_item(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_request(self.addr, "getItem", &args);
    }
    pub fn read_last_get_item(&self) -> Result<MEItemEntry, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getItem")?;
        peripheral::decode(&data)
    }

    /// アイテムを外部インベントリに出力する。
    pub fn book_next_export_item(&mut self, filter: &[u8], side: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(side)]);
        peripheral::book_action(self.addr, "exportItem", &args);
    }
    pub fn read_last_export_item(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "exportItem")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 外部インベントリからアイテムを取り込む。
    pub fn book_next_import_item(&mut self, filter: &[u8], side: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(side)]);
        peripheral::book_action(self.addr, "importItem", &args);
    }
    pub fn read_last_import_item(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "importItem")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定ペリフェラルへアイテムを出力する。
    pub fn book_next_export_item_to_peripheral(&mut self, filter: &[u8], target_name: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(target_name)]);
        peripheral::book_action(self.addr, "exportItemToPeripheral", &args);
    }
    pub fn read_last_export_item_to_peripheral(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "exportItemToPeripheral")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定ペリフェラルからアイテムを取り込む。
    pub fn book_next_import_item_from_peripheral(&mut self, filter: &[u8], target_name: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(target_name)]);
        peripheral::book_action(self.addr, "importItemFromPeripheral", &args);
    }
    pub fn read_last_import_item_from_peripheral(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "importItemFromPeripheral")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    // ─── 流体操作 ────────────────────────────────────────────

    /// ネットワーク内の全流体一覧を返す。
    pub fn book_next_list_fluids(&mut self) {
        peripheral::book_request(self.addr, "listFluids", &msgpack::array(&[]));
    }
    pub fn read_last_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "listFluids")?;
        peripheral::decode(&data)
    }

    /// 条件に合う最初の流体を返す。
    pub fn book_next_get_fluid(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_request(self.addr, "getFluid", &args);
    }
    pub fn read_last_get_fluid(&self) -> Result<MEFluidEntry, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getFluid")?;
        peripheral::decode(&data)
    }

    /// 流体を出力する。
    pub fn book_next_export_fluid(&mut self, filter: &[u8], side: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(side)]);
        peripheral::book_action(self.addr, "exportFluid", &args);
    }
    pub fn read_last_export_fluid(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "exportFluid")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 外部から流体を取り込む。
    pub fn book_next_import_fluid(&mut self, filter: &[u8], side: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(side)]);
        peripheral::book_action(self.addr, "importFluid", &args);
    }
    pub fn read_last_import_fluid(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "importFluid")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定ペリフェラルへ流体を出力する。
    pub fn book_next_export_fluid_to_peripheral(&mut self, filter: &[u8], target_name: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(target_name)]);
        peripheral::book_action(self.addr, "exportFluidToPeripheral", &args);
    }
    pub fn read_last_export_fluid_to_peripheral(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "exportFluidToPeripheral")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定ペリフェラルから流体を取り込む。
    pub fn book_next_import_fluid_from_peripheral(&mut self, filter: &[u8], target_name: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(target_name)]);
        peripheral::book_action(self.addr, "importFluidFromPeripheral", &args);
    }
    pub fn read_last_import_fluid_from_peripheral(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "importFluidFromPeripheral")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    // ─── ケミカル操作 ────────────────────────────────────────

    /// ネットワーク内の全ケミカル一覧を返す。
    pub fn book_next_list_chemicals(&mut self) {
        peripheral::book_request(self.addr, "listChemicals", &msgpack::array(&[]));
    }
    pub fn read_last_list_chemicals(&self) -> Result<Vec<MEChemicalEntry>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "listChemicals")?;
        peripheral::decode(&data)
    }

    /// 条件に合う最初のケミカルを返す。
    pub fn book_next_get_chemical(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_request(self.addr, "getChemical", &args);
    }
    pub fn read_last_get_chemical(&self) -> Result<MEChemicalEntry, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getChemical")?;
        peripheral::decode(&data)
    }

    /// ケミカルを出力する。
    pub fn book_next_export_chemical(&mut self, filter: &[u8], side: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(side)]);
        peripheral::book_action(self.addr, "exportChemical", &args);
    }
    pub fn read_last_export_chemical(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "exportChemical")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 外部からケミカルを取り込む。
    pub fn book_next_import_chemical(&mut self, filter: &[u8], side: &str) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(side)]);
        peripheral::book_action(self.addr, "importChemical", &args);
    }
    pub fn read_last_import_chemical(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "importChemical")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定ペリフェラルへケミカルを出力する。
    pub fn book_next_export_chemical_to_peripheral(
        &mut self,
        filter: &[u8],
        target_name: &str,
    ) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(target_name)]);
        peripheral::book_action(self.addr, "exportChemicalToPeripheral", &args);
    }
    pub fn read_last_export_chemical_to_peripheral(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "exportChemicalToPeripheral")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定ペリフェラルからケミカルを取り込む。
    pub fn book_next_import_chemical_from_peripheral(
        &mut self,
        filter: &[u8],
        target_name: &str,
    ) {
        let args = msgpack::array(&[filter.to_vec(), msgpack::str(target_name)]);
        peripheral::book_action(self.addr, "importChemicalFromPeripheral", &args);
    }
    pub fn read_last_import_chemical_from_peripheral(&self) -> Vec<Result<i64, PeripheralError>> {
        peripheral::read_action_results(self.addr, "importChemicalFromPeripheral")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    // ─── クラフト ────────────────────────────────────────────

    /// 指定アイテムのクラフトを要求する。
    pub fn book_next_craft_item(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_action(self.addr, "craftItem", &args);
    }
    pub fn read_last_craft_item(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "craftItem")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定流体のクラフトを要求する。
    pub fn book_next_craft_fluid(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_action(self.addr, "craftFluid", &args);
    }
    pub fn read_last_craft_fluid(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "craftFluid")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定ケミカルのクラフトを要求する。
    pub fn book_next_craft_chemical(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_action(self.addr, "craftChemical", &args);
    }
    pub fn read_last_craft_chemical(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "craftChemical")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// 指定アイテムのクラフトが進行中かを返す。
    pub fn book_next_is_item_crafting(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_request(self.addr, "isItemCrafting", &args);
    }
    pub fn read_last_is_item_crafting(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isItemCrafting")?;
        peripheral::decode(&data)
    }

    /// 指定流体のクラフトが進行中かを返す。
    pub fn book_next_is_fluid_crafting(&mut self, filter: &[u8]) {
        let args = msgpack::array(&[filter.to_vec()]);
        peripheral::book_request(self.addr, "isFluidCrafting", &args);
    }
    pub fn read_last_is_fluid_crafting(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isFluidCrafting")?;
        peripheral::decode(&data)
    }

    // ─── ストレージ容量 ──────────────────────────────────────

    /// ネットワークの蓄積エネルギーを返す。
    pub fn book_next_get_energy_storage(&mut self) {
        peripheral::book_request(self.addr, "getEnergyStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_energy_storage(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getEnergyStorage")?;
        peripheral::decode(&data)
    }

    /// ネットワークの最大エネルギー容量を返す。
    pub fn book_next_get_max_energy_storage(&mut self) {
        peripheral::book_request(self.addr, "getMaxEnergyStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_max_energy_storage(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getMaxEnergyStorage")?;
        peripheral::decode(&data)
    }

    /// 平均電力使用量を返す。
    pub fn book_next_get_avg_power_usage(&mut self) {
        peripheral::book_request(self.addr, "getAvgPowerUsage", &msgpack::array(&[]));
    }
    pub fn read_last_get_avg_power_usage(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAvgPowerUsage")?;
        peripheral::decode(&data)
    }

    /// 平均電力注入量を返す。
    pub fn book_next_get_avg_power_injection(&mut self) {
        peripheral::book_request(self.addr, "getAvgPowerInjection", &msgpack::array(&[]));
    }
    pub fn read_last_get_avg_power_injection(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAvgPowerInjection")?;
        peripheral::decode(&data)
    }

    /// アイテムストレージの総容量を返す。
    pub fn book_next_get_total_item_storage(&mut self) {
        peripheral::book_request(self.addr, "getTotalItemStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_total_item_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTotalItemStorage")?;
        peripheral::decode(&data)
    }

    /// アイテムストレージの使用済み容量を返す。
    pub fn book_next_get_used_item_storage(&mut self) {
        peripheral::book_request(self.addr, "getUsedItemStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_used_item_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getUsedItemStorage")?;
        peripheral::decode(&data)
    }

    /// アイテムストレージの空き容量を返す。
    pub fn book_next_get_available_item_storage(&mut self) {
        peripheral::book_request(self.addr, "getAvailableItemStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_available_item_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAvailableItemStorage")?;
        peripheral::decode(&data)
    }

    /// 流体ストレージの総容量を返す。
    pub fn book_next_get_total_fluid_storage(&mut self) {
        peripheral::book_request(self.addr, "getTotalFluidStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_total_fluid_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTotalFluidStorage")?;
        peripheral::decode(&data)
    }

    /// 流体ストレージの使用済み容量を返す。
    pub fn book_next_get_used_fluid_storage(&mut self) {
        peripheral::book_request(self.addr, "getUsedFluidStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_used_fluid_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getUsedFluidStorage")?;
        peripheral::decode(&data)
    }

    /// 流体ストレージの空き容量を返す。
    pub fn book_next_get_available_fluid_storage(&mut self) {
        peripheral::book_request(self.addr, "getAvailableFluidStorage", &msgpack::array(&[]));
    }
    pub fn read_last_get_available_fluid_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAvailableFluidStorage")?;
        peripheral::decode(&data)
    }

    /// ケミカルストレージの総容量を返す。
    pub fn book_next_get_total_chemical_storage(&mut self) {
        peripheral::book_request(
            self.addr,
            "getTotalChemicalStorage",
            &msgpack::array(&[]),
        );
    }
    pub fn read_last_get_total_chemical_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTotalChemicalStorage")?;
        peripheral::decode(&data)
    }

    /// ケミカルストレージの使用済み容量を返す。
    pub fn book_next_get_used_chemical_storage(&mut self) {
        peripheral::book_request(
            self.addr,
            "getUsedChemicalStorage",
            &msgpack::array(&[]),
        );
    }
    pub fn read_last_get_used_chemical_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getUsedChemicalStorage")?;
        peripheral::decode(&data)
    }

    /// ケミカルストレージの空き容量を返す。
    pub fn book_next_get_available_chemical_storage(&mut self) {
        peripheral::book_request(
            self.addr,
            "getAvailableChemicalStorage",
            &msgpack::array(&[]),
        );
    }
    pub fn read_last_get_available_chemical_storage(&self) -> Result<i64, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getAvailableChemicalStorage")?;
        peripheral::decode(&data)
    }
}
