# CoalBurner

**Module:** `clockwork_cc_compat::coal_burner`  
**Peripheral Type:** `clockwork:coal_burner`

Clockwork CC Compat Coal Burner peripheral. Provides access to fuel status, burn time, and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_fuel_ticks` / `read_last_get_fuel_ticks`
Get the remaining fuel ticks.
```rust
pub fn book_next_get_fuel_ticks(&mut self)
pub fn read_last_get_fuel_ticks(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_max_burn_time` / `read_last_get_max_burn_time`
Get the maximum burn time of the current fuel.
```rust
pub fn book_next_get_max_burn_time(&mut self)
pub fn read_last_get_max_burn_time(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_is_burning` / `read_last_is_burning`
Check if the burner is currently burning fuel.
```rust
pub fn book_next_is_burning(&mut self)
pub fn read_last_is_burning(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

### GasNetwork Common Methods

#### `book_next_get_temperature` / `read_last_get_temperature`
Get the gas network temperature.
```rust
pub fn book_next_get_temperature(&mut self)
pub fn read_last_get_temperature(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_pressure` / `read_last_get_pressure`
Get the gas network pressure.
```rust
pub fn book_next_get_pressure(&mut self)
pub fn read_last_get_pressure(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_heat_energy` / `read_last_get_heat_energy`
Get the heat energy in the gas network.
```rust
pub fn book_next_get_heat_energy(&mut self)
pub fn read_last_get_heat_energy(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_gas_mass` / `read_last_get_gas_mass`
Get a map of gas names to their masses (kg).
```rust
pub fn book_next_get_gas_mass(&mut self)
pub fn read_last_get_gas_mass(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**Returns:** `BTreeMap<String, f64>`

---

#### `book_next_get_position` / `read_last_get_position`
Get the block position.
```rust
pub fn book_next_get_position(&mut self)
pub fn read_last_get_position(&self) -> Result<CLPosition, PeripheralError>
```
**Returns:** `CLPosition`

---

#### `book_next_get_network_info` / `read_last_get_network_info`
Get detailed gas network information (non-imm, book-read only).
```rust
pub fn book_next_get_network_info(&mut self)
pub fn read_last_get_network_info(&self) -> Result<Value, PeripheralError>
```
**Returns:** `Value` (dynamic table)

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

- `get_fuel_ticks_imm(&self) -> Result<f64, PeripheralError>`
- `get_max_burn_time_imm(&self) -> Result<f64, PeripheralError>`
- `is_burning_imm(&self) -> Result<bool, PeripheralError>`
- `get_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_pressure_imm(&self) -> Result<f64, PeripheralError>`
- `get_heat_energy_imm(&self) -> Result<f64, PeripheralError>`
- `get_gas_mass_imm(&self) -> Result<BTreeMap<String, f64>, PeripheralError>`
- `get_position_imm(&self) -> Result<CLPosition, PeripheralError>`

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
use rust_computers_api::clockwork_cc_compat::CoalBurner;
use rust_computers_api::peripheral::Peripheral;

let mut burner = CoalBurner::wrap(addr);

loop {
    let burning = burner.read_last_is_burning();
    let fuel = burner.read_last_get_fuel_ticks();

    burner.book_next_is_burning();
    burner.book_next_get_fuel_ticks();
    wait_for_next_tick().await;
}
```
