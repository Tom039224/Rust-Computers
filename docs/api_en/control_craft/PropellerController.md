# PropellerController

**Module:** `control_craft::propeller_controller`  
**Peripheral Type:** `controlcraft:propeller_controller_peripheral`

Control-Craft Propeller Controller peripheral for setting and reading the target speed of propellers.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_target_speed` | `read_last_get_target_speed` | `get_target_speed_imm` | `f64` |

---

### Setters

#### `book_next_set_target_speed` / `read_last_set_target_speed`
Set the propeller target speed in RPM.
```rust
pub fn book_next_set_target_speed(&mut self, speed: f64) { ... }
pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `speed: f64` — Target speed in RPM

## Usage Example

```rust
use rust_computers_api::control_craft::propeller_controller::*;
use rust_computers_api::peripheral::Peripheral;

let mut prop = PropellerController::find().unwrap();

// Set target speed
prop.book_next_set_target_speed(256.0);
wait_for_next_tick().await;
let _ = prop.read_last_set_target_speed();

// Read current target speed immediately
let speed = prop.get_target_speed_imm().unwrap();
```
