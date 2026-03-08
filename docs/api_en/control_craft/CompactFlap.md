# CompactFlap

**Module:** `control_craft::compact_flap`  
**Peripheral Type:** `controlcraft:compact_flap_peripheral`

Control-Craft Compact Flap peripheral for controlling flap angle and tilt on compact flap surfaces.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |
| `book_next_get_tilt` | `read_last_get_tilt` | `get_tilt_imm` | `f64` |

---

### Setters

#### `book_next_set_angle` / `read_last_set_angle`
Set the flap angle in degrees.
```rust
pub fn book_next_set_angle(&mut self, angle: f64) { ... }
pub fn read_last_set_angle(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `angle: f64` — Flap angle in degrees

#### `book_next_set_tilt` / `read_last_set_tilt`
Set the flap tilt angle in degrees.
```rust
pub fn book_next_set_tilt(&mut self, tilt: f64) { ... }
pub fn read_last_set_tilt(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `tilt: f64` — Tilt angle in degrees

## Usage Example

```rust
use rust_computers_api::control_craft::compact_flap::*;
use rust_computers_api::peripheral::Peripheral;

let mut flap = CompactFlap::find().unwrap();

// Set flap angle and tilt
flap.book_next_set_angle(30.0);
flap.book_next_set_tilt(10.0);
wait_for_next_tick().await;
let _ = flap.read_last_set_angle();
let _ = flap.read_last_set_tilt();

// Read current angle immediately
let angle = flap.get_angle_imm().unwrap();
```
