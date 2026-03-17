# RedstoneDuct

**Module:** `clockwork_cc_compat::redstone_duct`  
**Peripheral Type:** `clockwork:redstone_duct`

Clockwork CC Compat Redstone Duct peripheral. Provides access to redstone power level, conditional settings, and shared gas network information.

## Book-Read Methods

All methods below also have `_imm` immediate variants (see Immediate Methods section).

### `book_next_get_power` / `read_last_get_power`
Get the current redstone power level.
```rust
pub fn book_next_get_power(&mut self)
pub fn read_last_get_power(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_conditional` / `read_last_get_conditional`
Get the conditional settings of the redstone duct.
```rust
pub fn book_next_get_conditional(&mut self)
pub fn read_last_get_conditional(&self) -> Result<ConditionalInfo, PeripheralError>
```
**Returns:** `ConditionalInfo`

---

### GasNetwork Common Methods

See [GasNetwork](GasNetwork.md) for: `getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

- `get_power_imm(&self) -> Result<f64, PeripheralError>`
- `get_conditional_imm(&self) -> Result<ConditionalInfo, PeripheralError>`
- Plus all GasNetwork `_imm` variants

## Types

```rust
pub struct ConditionalInfo {
    pub more_than: bool,        // serde: "moreThan"
    pub comparison_value: f64,  // serde: "comparisonValue"
    pub filter_blacklist: bool, // serde: "filterBlacklist"
}

pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## Usage Example

```rust
use rust_computers_api::clockwork_cc_compat::RedstoneDuct;
use rust_computers_api::peripheral::Peripheral;

let mut duct = RedstoneDuct::wrap(addr);

loop {
    let power = duct.read_last_get_power();
    let cond = duct.read_last_get_conditional();

    duct.book_next_get_power();
    duct.book_next_get_conditional();
    wait_for_next_tick().await;
}
```
