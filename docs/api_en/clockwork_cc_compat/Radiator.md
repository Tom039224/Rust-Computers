# Radiator

**Module:** `clockwork_cc_compat::radiator`  
**Peripheral Type:** `clockwork:radiator`

Clockwork CC Compat Radiator peripheral. Provides comprehensive monitoring of radiator state including fan information, thermal properties, gas conversions, and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_fan_type` / `read_last_get_fan_type`
Get the fan type name.
```rust
pub fn book_next_get_fan_type(&mut self)
pub fn read_last_get_fan_type(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

### `book_next_get_fan_rpm` / `read_last_get_fan_rpm`
Get the fan RPM.
```rust
pub fn book_next_get_fan_rpm(&mut self)
pub fn read_last_get_fan_rpm(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_fan_count` / `read_last_get_fan_count`
Get the number of fans.
```rust
pub fn book_next_get_fan_count(&mut self)
pub fn read_last_get_fan_count(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_fans` / `read_last_get_fans`
Get detailed information about all fans.
```rust
pub fn book_next_get_fans(&mut self)
pub fn read_last_get_fans(&self) -> Result<Vec<FanInfo>, PeripheralError>
```
**Returns:** `Vec<FanInfo>`

---

### `book_next_is_active` / `read_last_is_active`
Check if the radiator is active.
```rust
pub fn book_next_is_active(&mut self)
pub fn read_last_is_active(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

### `book_next_is_cooling` / `read_last_is_cooling`
Check if the radiator is in cooling mode.
```rust
pub fn book_next_is_cooling(&mut self)
pub fn read_last_is_cooling(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

### `book_next_is_heating` / `read_last_is_heating`
Check if the radiator is in heating mode.
```rust
pub fn book_next_is_heating(&mut self)
pub fn read_last_is_heating(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

### `book_next_get_target_temp` / `read_last_get_target_temp`
Get the target temperature.
```rust
pub fn book_next_get_target_temp(&mut self)
pub fn read_last_get_target_temp(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_input_temperature` / `read_last_get_input_temperature`
Get the input temperature.
```rust
pub fn book_next_get_input_temperature(&mut self)
pub fn read_last_get_input_temperature(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_output_temperature` / `read_last_get_output_temperature`
Get the output temperature.
```rust
pub fn book_next_get_output_temperature(&mut self)
pub fn read_last_get_output_temperature(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_thermal_factor` / `read_last_get_thermal_factor`
Get the thermal factor.
```rust
pub fn book_next_get_thermal_factor(&mut self)
pub fn read_last_get_thermal_factor(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_atmospheric_pressure` / `read_last_get_atmospheric_pressure`
Get the atmospheric pressure.
```rust
pub fn book_next_get_atmospheric_pressure(&mut self)
pub fn read_last_get_atmospheric_pressure(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_pressure_scale` / `read_last_get_pressure_scale`
Get the pressure scale.
```rust
pub fn book_next_get_pressure_scale(&mut self)
pub fn read_last_get_pressure_scale(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_thermal_power` / `read_last_get_thermal_power`
Get the thermal power.
```rust
pub fn book_next_get_thermal_power(&mut self)
pub fn read_last_get_thermal_power(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_status` / `read_last_get_status`
Get the radiator status.
```rust
pub fn book_next_get_status(&mut self)
pub fn read_last_get_status(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

### `book_next_get_conversion_rate` / `read_last_get_conversion_rate`
Get the gas conversion rate.
```rust
pub fn book_next_get_conversion_rate(&mut self)
pub fn read_last_get_conversion_rate(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_conversions` / `read_last_get_conversions`
Get the list of gas conversions.
```rust
pub fn book_next_get_conversions(&mut self)
pub fn read_last_get_conversions(&self) -> Result<Vec<ConversionInfo>, PeripheralError>
```
**Returns:** `Vec<ConversionInfo>`

---

### GasNetwork Common Methods

See [GasNetwork](GasNetwork.md) for: `getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`.

## Immediate Methods

All imm_getter methods have `_imm` variants:

- `get_fan_type_imm`, `get_fan_rpm_imm`, `get_fan_count_imm`, `get_fans_imm`
- `is_active_imm`, `is_cooling_imm`, `is_heating_imm`
- `get_target_temp_imm`, `get_input_temperature_imm`, `get_output_temperature_imm`
- `get_thermal_factor_imm`, `get_atmospheric_pressure_imm`, `get_pressure_scale_imm`
- `get_thermal_power_imm`, `get_status_imm`
- `get_conversion_rate_imm`, `get_conversions_imm`
- Plus all GasNetwork `_imm` variants

## Types

```rust
pub struct FanInfo {
    pub r#type: String,
    pub rpm: f64,
    pub dir: String,
    pub dist: f64,
}

pub struct ConversionInfo {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## Usage Example

```rust
use rust_computers_api::clockwork_cc_compat::Radiator;
use rust_computers_api::peripheral::Peripheral;

let mut radiator = Radiator::wrap(addr);

loop {
    let active = radiator.read_last_is_active();
    let status = radiator.read_last_get_status();
    let fans = radiator.read_last_get_fans();

    radiator.book_next_is_active();
    radiator.book_next_get_status();
    radiator.book_next_get_fans();
    wait_for_next_tick().await;
}
```
