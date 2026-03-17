# KinematicMotor

**Module:** `control_craft::kinematic_motor`  
**Peripheral Type:** `controlcraft:kinematic_motor_peripheral`

Control-Craft Kinematic Motor peripheral for position-based motor control with target angle management, control target selection, and force angle mode.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_target_angle` | `read_last_get_target_angle` | `get_target_angle_imm` | `f64` |
| `book_next_get_control_target` | `read_last_get_control_target` | `get_control_target_imm` | `String` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |
| `book_next_get_relative` | `read_last_get_relative` | `get_relative_imm` | `[[f64; 3]; 3]` |

---

### Setters

#### `book_next_set_target_angle` / `read_last_set_target_angle`
Set the target angle in degrees.
```rust
pub fn book_next_set_target_angle(&mut self, value: f64) { ... }
pub fn read_last_set_target_angle(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `value: f64` — Target angle in degrees

#### `book_next_set_control_target` / `read_last_set_control_target`
Set the control target identifier.
```rust
pub fn book_next_set_control_target(&mut self, target: &str) { ... }
pub fn read_last_set_control_target(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `target: &str` — Control target name

#### `book_next_set_is_forcing_angle` / `read_last_set_is_forcing_angle`
Enable or disable force angle mode.
```rust
pub fn book_next_set_is_forcing_angle(&mut self, enabled: bool) { ... }
pub fn read_last_set_is_forcing_angle(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `enabled: bool` — Whether force angle mode is enabled

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Usage Example

```rust
use rust_computers_api::control_craft::kinematic_motor::*;
use rust_computers_api::peripheral::Peripheral;

let mut motor = KinematicMotor::find().unwrap();

// Set target angle
motor.book_next_set_target_angle(45.0);
wait_for_next_tick().await;
let _ = motor.read_last_set_target_angle();

// Read current angle immediately
let angle = motor.get_angle_imm().unwrap();

// Check current control target
let target = motor.get_control_target_imm().unwrap();
```
