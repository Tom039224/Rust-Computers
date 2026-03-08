# CompactCannonMount

**Module:** `cbc_cc_control::compact_cannon_mount`  
**Peripheral Type:** `cbc_cannon_mount`

CBC CC Control peripheral for Create Big Cannons cannon mounts.
Provides control of pitch/yaw orientation, contraption assembly/disassembly, and firing.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_is_running` | `read_last_is_running` | `is_running_imm` | `bool` |
| `book_next_get_yaw` | `read_last_get_yaw` | `get_yaw_imm` | `f64` |
| `book_next_get_pitch` | `read_last_get_pitch` | `get_pitch_imm` | `f64` |

---

## Action Methods

### `book_next_assemble` / `read_last_assemble`
Assemble the contraption. Calls `tryUpdatingSpeed()`, `assemble()`, `sendData()` internally then returns `isRunning()`. Runs on mainThread.
```rust
pub fn book_next_assemble(&mut self) { ... }
pub fn read_last_assemble(&self) -> Vec<Result<bool, PeripheralError>> { ... }
```
**Returns:** `bool` — `true` if the contraption is running after assembly.

---

### `book_next_disassemble` / `read_last_disassemble`
Disassemble the contraption. Runs on mainThread.
```rust
pub fn book_next_disassemble(&mut self) { ... }
pub fn read_last_disassemble(&self) -> Vec<Result<(), PeripheralError>> { ... }
```

---

### `book_next_set_yaw` / `read_last_set_yaw`
Set the cannon yaw angle in degrees. Runs on mainThread.
```rust
pub fn book_next_set_yaw(&mut self, yaw: f64) { ... }
pub fn read_last_set_yaw(&self) -> Vec<Result<(), PeripheralError>> { ... }
```
**Parameters:** `yaw: f64` — Yaw angle in degrees

---

### `book_next_set_pitch` / `read_last_set_pitch`
Set the cannon pitch angle in degrees. Runs on mainThread.
```rust
pub fn book_next_set_pitch(&mut self, pitch: f64) { ... }
pub fn read_last_set_pitch(&self) -> Vec<Result<(), PeripheralError>> { ... }
```
**Parameters:** `pitch: f64` — Pitch angle in degrees

---

### `book_next_fire` / `read_last_fire`
Fire the cannon (calls `onRedstoneUpdate()` internally).
```rust
pub fn book_next_fire(&mut self) { ... }
pub fn read_last_fire(&self) -> Vec<Result<(), PeripheralError>> { ... }
```

---

## Usage Example

```rust
use rust_computers_api::cbc_cc_control::compact_cannon_mount::CompactCannonMount;
use rust_computers_api::peripheral::Peripheral;
use rust_computers_api::wait_for_next_tick;

let mut cannon = CompactCannonMount::find().unwrap();

// Read current orientation immediately
let yaw = cannon.get_yaw_imm().unwrap();
let pitch = cannon.get_pitch_imm().unwrap();

// Aim and fire
cannon.book_next_set_yaw(90.0);
cannon.book_next_set_pitch(-10.0);
wait_for_next_tick().await;
let _ = cannon.read_last_set_yaw();
let _ = cannon.read_last_set_pitch();

cannon.book_next_fire();
wait_for_next_tick().await;
let _ = cannon.read_last_fire();

// Assemble contraption and check if running
cannon.book_next_assemble();
wait_for_next_tick().await;
let results = cannon.read_last_assemble();
let is_running = results.into_iter().next().unwrap().unwrap();
```
