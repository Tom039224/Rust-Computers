# KineticResistor

**Module:** `control_craft::kinetic_resistor`  
**Peripheral Type:** `controlcraft:kinetic_resistor_peripheral`

Control-Craft Kinetic Resistor peripheral for controlling resistance ratio on kinetic resistance devices.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_ratio` | `read_last_get_ratio` | `get_ratio_imm` | `f64` |

---

### Setters

#### `book_next_set_ratio` / `read_last_set_ratio`
Set the resistance ratio (runs on mainThread).
```rust
pub fn book_next_set_ratio(&mut self, ratio: f64) { ... }
pub fn read_last_set_ratio(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `ratio: f64` — Resistance ratio

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Usage Example

```rust
use rust_computers_api::control_craft::kinetic_resistor::*;
use rust_computers_api::peripheral::Peripheral;

let mut resistor = KineticResistor::find().unwrap();

// Set resistance ratio
resistor.book_next_set_ratio(0.5);
wait_for_next_tick().await;
let _ = resistor.read_last_set_ratio();

// Read current ratio immediately
let ratio = resistor.get_ratio_imm().unwrap();
```
