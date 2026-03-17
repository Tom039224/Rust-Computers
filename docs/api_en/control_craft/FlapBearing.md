# FlapBearing

**Module:** `control_craft::flap_bearing`  
**Peripheral Type:** `controlcraft:flap_bearing_peripheral`

Control-Craft Flap Bearing peripheral for controlling wing angle and managing contraption assembly/disassembly.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |

---

### Setters

#### `book_next_set_angle` / `read_last_set_angle`
Set the wing angle in degrees.
```rust
pub fn book_next_set_angle(&mut self, angle: f64) { ... }
pub fn read_last_set_angle(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `angle: f64` — Wing angle in degrees

---

### Contraption Control

#### `book_next_assemble_next_tick` / `read_last_assemble_next_tick`
Assemble the contraption on the next tick.
```rust
pub fn book_next_assemble_next_tick(&mut self) { ... }
pub fn read_last_assemble_next_tick(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_disassemble_next_tick` / `read_last_disassemble_next_tick`
Disassemble the contraption on the next tick.
```rust
pub fn book_next_disassemble_next_tick(&mut self) { ... }
pub fn read_last_disassemble_next_tick(&self) -> Result<(), PeripheralError> { ... }
```

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Usage Example

```rust
use rust_computers_api::control_craft::flap_bearing::*;
use rust_computers_api::peripheral::Peripheral;

let mut bearing = FlapBearing::find().unwrap();

// Assemble the contraption
bearing.book_next_assemble_next_tick();
wait_for_next_tick().await;
let _ = bearing.read_last_assemble_next_tick();

// Set wing angle
bearing.book_next_set_angle(20.0);
wait_for_next_tick().await;
let _ = bearing.read_last_set_angle();

// Read current angle immediately
let angle = bearing.get_angle_imm().unwrap();
```
