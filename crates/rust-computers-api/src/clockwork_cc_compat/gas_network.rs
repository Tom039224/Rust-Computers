//! Clockwork CC Compat GasNetwork 基底マクロ。
//!
//! 全ガス系ペリフェラルが継承する共通メソッドをマクロで提供する。

/// GasNetwork 共通メソッドを生成するマクロ。
///
/// `imm_getter!` マクロがスコープ内に存在する前提で使用する。
/// `getNetworkInfo` は non-imm なので手動で定義する。
macro_rules! gas_network_methods {
    () => {
        // ---- GasNetwork 共通 (imm) ----

        imm_getter!(
            book_next_get_temperature,
            read_last_get_temperature,
            get_temperature_imm,
            "getTemperature",
            f64
        );

        imm_getter!(
            book_next_get_pressure,
            read_last_get_pressure,
            get_pressure_imm,
            "getPressure",
            f64
        );

        imm_getter!(
            book_next_get_heat_energy,
            read_last_get_heat_energy,
            get_heat_energy_imm,
            "getHeatEnergy",
            f64
        );

        imm_getter!(
            book_next_get_gas_mass,
            read_last_get_gas_mass,
            get_gas_mass_imm,
            "getGasMass",
            alloc::collections::BTreeMap<alloc::string::String, f64>
        );

        imm_getter!(
            book_next_get_position,
            read_last_get_position,
            get_position_imm,
            "getPosition",
            super::CLPosition
        );

        // ---- GasNetwork 共通 (non-imm) ----

        /// ネットワーク詳細情報を予約する。
        pub fn book_next_get_network_info(&mut self) {
            crate::peripheral::book_request(
                self.addr,
                "getNetworkInfo",
                &crate::msgpack::array(&[]),
            );
        }

        /// ネットワーク詳細情報を読み取る。
        pub fn read_last_get_network_info(
            &self,
        ) -> Result<crate::msgpack::Value, crate::error::PeripheralError> {
            let data =
                crate::peripheral::read_result(self.addr, "getNetworkInfo")?;
            crate::peripheral::decode(&data)
        }
    };
}

pub(crate) use gas_network_methods;
