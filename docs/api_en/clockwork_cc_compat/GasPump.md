# GasPump

**Module:** `clockwork_cc_compat::gas_pump`  
**Peripheral Type:** `clockwork:gas_pump`

Clockwork CC Compat Gas Pump peripheral. Provides access to pump pressure, speed, facing direction, and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_pump_pressure` / `read_last_get_pump_pressure`
Get the pump pressure.
```rust
pub fn book_next_get_pump_pressure(&mut self)
pub fn read_last_get_pump_pressure(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_speed` / `read_last_get_speed`
Get the current speed.
```rust
pub fn book_next_get_speed(&mut self)
pub fn read_last_get_speed(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_facing` / `read_last_get_facing`
Get the facing direction.
```rust
pub fn book_next_get_facing(&mut self)
pub fn read_last_get_facing(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

### GasNetwork Common Methods

See [GasNetwork](GasNetwork.md) for: `getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

- `get_pump_pressure_imm(&self) -> Result<f64, PeripheralError>`
- `get_speed_imm(&self) -> Result<f64, PeripheralError>`
- `get_facing_imm(&self) -> Result<String, PeripheralError>`
- Plus all GasNetwork `_imm` variants

## Types

```rust
pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## Usage Example

```rust
use rust_computers_api::clockwork_cc_compat::GasPump;
use rust_computers_api::peripheral::Peripheral;

let mut pump = GasPump::wrap(addr);

loop {
    let pressure = pump.read_last_get_pump_pressure();
    let speed = pump.read_last_get_speed();

    pump.book_next_get_pump_pressure();
    pump.book_next_get_speed();
    wait_for_next_tick().await;
}
```
