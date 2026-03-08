# AirCompressor

**Module:** `clockwork_cc_compat::air_compressor`  
**Peripheral Type:** `clockwork:air_compressor`

Clockwork CC Compat Air Compressor peripheral. Provides access to compressor status, speed, facing direction, and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_status` / `read_last_get_status`
Get the current status of the air compressor.
```rust
pub fn book_next_get_status(&mut self)
pub fn read_last_get_status(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

### `book_next_get_speed` / `read_last_get_speed`
Get the current speed of the compressor.
```rust
pub fn book_next_get_speed(&mut self)
pub fn read_last_get_speed(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_facing` / `read_last_get_facing`
Get the facing direction of the compressor.
```rust
pub fn book_next_get_facing(&mut self)
pub fn read_last_get_facing(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

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

## Immediate Methods

All imm_getter methods above have `_imm` variants that return immediately without the book-read cycle:

- `get_status_imm(&self) -> Result<String, PeripheralError>`
- `get_speed_imm(&self) -> Result<f64, PeripheralError>`
- `get_facing_imm(&self) -> Result<String, PeripheralError>`
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
use rust_computers_api::clockwork_cc_compat::AirCompressor;
use rust_computers_api::peripheral::Peripheral;

let mut compressor = AirCompressor::wrap(addr);

loop {
    let status = compressor.read_last_get_status();
    let speed = compressor.read_last_get_speed();
    let temp = compressor.read_last_get_temperature();

    compressor.book_next_get_status();
    compressor.book_next_get_speed();
    compressor.book_next_get_temperature();
    wait_for_next_tick().await;
}
```
