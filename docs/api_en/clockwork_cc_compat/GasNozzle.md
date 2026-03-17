# GasNozzle

**Module:** `clockwork_cc_compat::gas_nozzle`  
**Peripheral Type:** `clockwork:gas_nozzle`

Clockwork CC Compat Gas Nozzle peripheral. Controls gas injection into balloons and provides extensive balloon monitoring including buoyancy, pressure, leaks, and gas contents.

## Book-Read Methods

### Setter Methods

#### `book_next_set_pointer` / `read_last_set_pointer`
Set the pointer value (injection rate control).
```rust
pub fn book_next_set_pointer(&mut self, value: f64)
pub fn read_last_set_pointer(&self) -> Result<(), PeripheralError>
```
**Parameters:**
- `value: f64` — Pointer value for injection rate control

**Returns:** `()`

---

### Immediate Getter Methods (with `_imm` variants)

#### `book_next_get_pointer` / `read_last_get_pointer`
Get the current pointer value.
```rust
pub fn book_next_get_pointer(&mut self)
pub fn read_last_get_pointer(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_pointer_speed` / `read_last_get_pointer_speed`
Get the pointer speed.
```rust
pub fn book_next_get_pointer_speed(&mut self)
pub fn read_last_get_pointer_speed(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_pocket_temperature` / `read_last_get_pocket_temperature`
Get the pocket temperature.
```rust
pub fn book_next_get_pocket_temperature(&mut self)
pub fn read_last_get_pocket_temperature(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_duct_temperature` / `read_last_get_duct_temperature`
Get the duct temperature.
```rust
pub fn book_next_get_duct_temperature(&mut self)
pub fn read_last_get_duct_temperature(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_target_temperature` / `read_last_get_target_temperature`
Get the target temperature.
```rust
pub fn book_next_get_target_temperature(&mut self)
pub fn read_last_get_target_temperature(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_balloon_volume` / `read_last_get_balloon_volume`
Get the balloon volume.
```rust
pub fn book_next_get_balloon_volume(&mut self)
pub fn read_last_get_balloon_volume(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_leaks` / `read_last_get_leaks`
Get the list of leak positions.
```rust
pub fn book_next_get_leaks(&mut self)
pub fn read_last_get_leaks(&self) -> Result<Vec<LeakInfo>, PeripheralError>
```
**Returns:** `Vec<LeakInfo>`

---

#### `book_next_get_temperature_delta` / `read_last_get_temperature_delta`
Get the temperature delta.
```rust
pub fn book_next_get_temperature_delta(&mut self)
pub fn read_last_get_temperature_delta(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_has_balloon` / `read_last_has_balloon`
Check if a balloon is attached.
```rust
pub fn book_next_has_balloon(&mut self)
pub fn read_last_has_balloon(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

### Non-Immediate Getter Methods (book-read only)

#### `book_next_get_buoyancy_force` / `read_last_get_buoyancy_force`
Get the current buoyancy force (N).
```rust
pub fn book_next_get_buoyancy_force(&mut self)
pub fn read_last_get_buoyancy_force(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_balloon_pressure` / `read_last_get_balloon_pressure`
Get the balloon internal pressure.
```rust
pub fn book_next_get_balloon_pressure(&mut self)
pub fn read_last_get_balloon_pressure(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_balloon_gas_contents` / `read_last_get_balloon_gas_contents`
Get the balloon gas contents as a map of gas names to masses (kg).
```rust
pub fn book_next_get_balloon_gas_contents(&mut self)
pub fn read_last_get_balloon_gas_contents(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**Returns:** `BTreeMap<String, f64>`

---

#### `book_next_get_loss_rate` / `read_last_get_loss_rate`
Get the gas loss rate (kg/s).
```rust
pub fn book_next_get_loss_rate(&mut self)
pub fn read_last_get_loss_rate(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_inflow_rate` / `read_last_get_inflow_rate`
Get the gas inflow rate (kg/s).
```rust
pub fn book_next_get_inflow_rate(&mut self)
pub fn read_last_get_inflow_rate(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_missing_positions` / `read_last_get_missing_positions`
Get the list of missing block positions required for the balloon.
```rust
pub fn book_next_get_missing_positions(&mut self)
pub fn read_last_get_missing_positions(&self) -> Result<Vec<CLPosition>, PeripheralError>
```
**Returns:** `Vec<CLPosition>`

---

#### `book_next_get_total_gas_mass` / `read_last_get_total_gas_mass`
Get the total gas mass inside the balloon (kg).
```rust
pub fn book_next_get_total_gas_mass(&mut self)
pub fn read_last_get_total_gas_mass(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_leak_integrity` / `read_last_get_leak_integrity`
Get the leak integrity (0–1).
```rust
pub fn book_next_get_leak_integrity(&mut self)
pub fn read_last_get_leak_integrity(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_max_leaks` / `read_last_get_max_leaks`
Get the maximum number of allowed leaks.
```rust
pub fn book_next_get_max_leaks(&mut self)
pub fn read_last_get_max_leaks(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_internal_density` / `read_last_get_internal_density`
Get the internal density of the balloon.
```rust
pub fn book_next_get_internal_density(&mut self)
pub fn read_last_get_internal_density(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### GasNetwork Common Methods

See [GasNetwork](GasNetwork.md) for: `getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

- `get_pointer_imm(&self) -> Result<f64, PeripheralError>`
- `get_pointer_speed_imm(&self) -> Result<f64, PeripheralError>`
- `get_pocket_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_duct_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_target_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_balloon_volume_imm(&self) -> Result<f64, PeripheralError>`
- `get_leaks_imm(&self) -> Result<Vec<LeakInfo>, PeripheralError>`
- `get_temperature_delta_imm(&self) -> Result<f64, PeripheralError>`
- `has_balloon_imm(&self) -> Result<bool, PeripheralError>`
- Plus all GasNetwork `_imm` variants

## Types

```rust
pub struct LeakInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## Usage Example

```rust
use rust_computers_api::clockwork_cc_compat::GasNozzle;
use rust_computers_api::peripheral::Peripheral;

let mut nozzle = GasNozzle::wrap(addr);

loop {
    let has = nozzle.read_last_has_balloon();
    let buoyancy = nozzle.read_last_get_buoyancy_force();

    nozzle.book_next_has_balloon();
    nozzle.book_next_get_buoyancy_force();
    nozzle.book_next_set_pointer(0.5);
    wait_for_next_tick().await;
}
```
