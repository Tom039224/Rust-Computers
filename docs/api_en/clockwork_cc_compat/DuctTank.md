# DuctTank

**Module:** `clockwork_cc_compat::duct_tank`  
**Peripheral Type:** `clockwork:duct_tank`

Clockwork CC Compat Duct Tank peripheral. Provides access to tank dimensions and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_height` / `read_last_get_height`
Get the tank height.
```rust
pub fn book_next_get_height(&mut self)
pub fn read_last_get_height(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_width` / `read_last_get_width`
Get the tank width.
```rust
pub fn book_next_get_width(&mut self)
pub fn read_last_get_width(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

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

- `get_height_imm(&self) -> Result<f64, PeripheralError>`
- `get_width_imm(&self) -> Result<f64, PeripheralError>`
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
use rust_computers_api::clockwork_cc_compat::DuctTank;
use rust_computers_api::peripheral::Peripheral;

let mut tank = DuctTank::wrap(addr);

loop {
    let height = tank.read_last_get_height();
    let pressure = tank.read_last_get_pressure();

    tank.book_next_get_height();
    tank.book_next_get_pressure();
    wait_for_next_tick().await;
}
```
