# GasValve

**Module:** `clockwork_cc_compat::gas_valve`  
**Peripheral Type:** `clockwork:gas_valve`

Clockwork CC Compat Gas Valve peripheral. Provides access to valve aperture, facing direction, and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_aperture` / `read_last_get_aperture`
Get the current valve aperture.
```rust
pub fn book_next_get_aperture(&mut self)
pub fn read_last_get_aperture(&self) -> Result<f64, PeripheralError>
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

## Immediate Methods

- `get_aperture_imm(&self) -> Result<f64, PeripheralError>`
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
use rust_computers_api::clockwork_cc_compat::GasValve;
use rust_computers_api::peripheral::Peripheral;

let mut valve = GasValve::wrap(addr);

loop {
    let aperture = valve.read_last_get_aperture();
    let facing = valve.read_last_get_facing();

    valve.book_next_get_aperture();
    valve.book_next_get_facing();
    wait_for_next_tick().await;
}
```
