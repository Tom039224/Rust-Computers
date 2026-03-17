# CannonMount

**Module:** `control_craft::cannon_mount`  
**Peripheral Type:** `controlcraft:cannon_mount_peripheral`

Control-Craft Cannon Mount peripheral for controlling cannon pitch and yaw orientation, as well as assembling/disassembling contraptions.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_pitch` | `read_last_get_pitch` | `get_pitch_imm` | `f64` |
| `book_next_get_yaw` | `read_last_get_yaw` | `get_yaw_imm` | `f64` |

---

### Setters

#### `book_next_set_pitch` / `read_last_set_pitch`
Set the cannon pitch angle in degrees.
```rust
pub fn book_next_set_pitch(&mut self, pitch: f64) { ... }
pub fn read_last_set_pitch(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `pitch: f64` — Pitch angle in degrees

#### `book_next_set_yaw` / `read_last_set_yaw`
Set the cannon yaw angle in degrees.
```rust
pub fn book_next_set_yaw(&mut self, yaw: f64) { ... }
pub fn read_last_set_yaw(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `yaw: f64` — Yaw angle in degrees

---

### Contraption Control

#### `book_next_assemble` / `read_last_assemble`
Assemble the contraption (runs on mainThread).
```rust
pub fn book_next_assemble(&mut self) { ... }
pub fn read_last_assemble(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_disassemble` / `read_last_disassemble`
Disassemble the contraption (runs on mainThread).
```rust
pub fn book_next_disassemble(&mut self) { ... }
pub fn read_last_disassemble(&self) -> Result<(), PeripheralError> { ... }
```

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Usage Example

```rust
use rust_computers_api::control_craft::cannon_mount::*;
use rust_computers_api::peripheral::Peripheral;

let mut mount = CannonMount::find().unwrap();

// Set cannon orientation
mount.book_next_set_pitch(-15.0);
mount.book_next_set_yaw(45.0);
wait_for_next_tick().await;
let _ = mount.read_last_set_pitch();
let _ = mount.read_last_set_yaw();

// Read current pitch immediately
let pitch = mount.get_pitch_imm().unwrap();

// Assemble the contraption
mount.book_next_assemble();
wait_for_next_tick().await;
let _ = mount.read_last_assemble();
```
