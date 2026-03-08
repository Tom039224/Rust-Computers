# GasNetwork

**Module:** `clockwork_cc_compat::gas_network`

Shared macro-based methods inherited by all Clockwork gas peripherals. This is not a standalone peripheral but a trait providing common gas network methods to: AirCompressor, CoalBurner, DuctTank, Exhaust, GasNozzle, GasPump, GasThruster, GasValve, Radiator, and RedstoneDuct.

## Book-Read Methods

All `imm_getter` methods have `_imm` immediate variants.

### `book_next_get_temperature` / `read_last_get_temperature`
Get the gas network temperature.
```rust
pub fn book_next_get_temperature(&mut self)
pub fn read_last_get_temperature(&self) -> Result<f64, PeripheralError>
pub fn get_temperature_imm(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_pressure` / `read_last_get_pressure`
Get the gas network pressure.
```rust
pub fn book_next_get_pressure(&mut self)
pub fn read_last_get_pressure(&self) -> Result<f64, PeripheralError>
pub fn get_pressure_imm(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_heat_energy` / `read_last_get_heat_energy`
Get the heat energy in the gas network.
```rust
pub fn book_next_get_heat_energy(&mut self)
pub fn read_last_get_heat_energy(&self) -> Result<f64, PeripheralError>
pub fn get_heat_energy_imm(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_gas_mass` / `read_last_get_gas_mass`
Get a map of gas names to their masses (kg).
```rust
pub fn book_next_get_gas_mass(&mut self)
pub fn read_last_get_gas_mass(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
pub fn get_gas_mass_imm(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**Returns:** `BTreeMap<String, f64>`

---

### `book_next_get_position` / `read_last_get_position`
Get the block position.
```rust
pub fn book_next_get_position(&mut self)
pub fn read_last_get_position(&self) -> Result<CLPosition, PeripheralError>
pub fn get_position_imm(&self) -> Result<CLPosition, PeripheralError>
```
**Returns:** `CLPosition`

---

### `book_next_get_network_info` / `read_last_get_network_info`
Get detailed gas network information. This method is **non-imm** (book-read only).
```rust
pub fn book_next_get_network_info(&mut self)
pub fn read_last_get_network_info(&self) -> Result<Value, PeripheralError>
```
**Returns:** `Value` (dynamic table)

## Types

```rust
pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```
