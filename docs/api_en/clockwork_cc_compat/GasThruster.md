# GasThruster

**Module:** `clockwork_cc_compat::gas_thruster`  
**Peripheral Type:** `clockwork:gas_thruster`

Clockwork CC Compat Gas Thruster peripheral. Provides access to thrust, flow rate, gas mass flow, facing direction, and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_thrust` / `read_last_get_thrust`
Get the current thrust value.
```rust
pub fn book_next_get_thrust(&mut self)
pub fn read_last_get_thrust(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_flow_rate` / `read_last_get_flow_rate`
Get the current flow rate.
```rust
pub fn book_next_get_flow_rate(&mut self)
pub fn read_last_get_flow_rate(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_gas_mass_flow` / `read_last_get_gas_mass_flow`
Get a map of gas names to their mass flow rates (kg/s).
```rust
pub fn book_next_get_gas_mass_flow(&mut self)
pub fn read_last_get_gas_mass_flow(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**Returns:** `BTreeMap<String, f64>`

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

## Immediate Methods

- `get_thrust_imm(&self) -> Result<f64, PeripheralError>`
- `get_flow_rate_imm(&self) -> Result<f64, PeripheralError>`
- `get_gas_mass_flow_imm(&self) -> Result<BTreeMap<String, f64>, PeripheralError>`
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
use rust_computers_api::clockwork_cc_compat::GasThruster;
use rust_computers_api::peripheral::Peripheral;

let mut thruster = GasThruster::wrap(addr);

loop {
    let thrust = thruster.read_last_get_thrust();
    let flow = thruster.read_last_get_flow_rate();

    thruster.book_next_get_thrust();
    thruster.book_next_get_flow_rate();
    wait_for_next_tick().await;
}
```
