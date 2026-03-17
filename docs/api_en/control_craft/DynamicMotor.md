# DynamicMotor

**Module:** `control_craft::dynamic_motor`  
**Peripheral Type:** `controlcraft:dynamic_motor_peripheral`

Control-Craft Dynamic Motor peripheral for controlling rotational motors with PID control, angle adjustment, torque output, and lock/unlock functionality.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_target_value` | `read_last_get_target_value` | `get_target_value_imm` | `f64` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |
| `book_next_get_angular_velocity` | `read_last_get_angular_velocity` | `get_angular_velocity_imm` | `f64` |
| `book_next_get_current_value` | `read_last_get_current_value` | `get_current_value_imm` | `f64` |
| `book_next_get_relative` | `read_last_get_relative` | `get_relative_imm` | `[[f64; 3]; 3]` |
| `book_next_is_locked` | `read_last_is_locked` | `is_locked_imm` | `bool` |

---

### Setters

#### `book_next_set_pid` / `read_last_set_pid`
Set the PID gains for motor control.
```rust
pub fn book_next_set_pid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_pid(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `p: f64` — Proportional gain, `i: f64` — Integral gain, `d: f64` — Derivative gain

#### `book_next_set_target_value` / `read_last_set_target_value`
Set the target value (angle in degrees).
```rust
pub fn book_next_set_target_value(&mut self, value: f64) { ... }
pub fn read_last_set_target_value(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `value: f64` — Target angle in degrees

#### `book_next_set_output_torque` / `read_last_set_output_torque`
Set the output torque scale.
```rust
pub fn book_next_set_output_torque(&mut self, scale: f64) { ... }
pub fn read_last_set_output_torque(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `scale: f64` — Torque scale factor

#### `book_next_set_is_adjusting_angle` / `read_last_set_is_adjusting_angle`
Enable or disable angle adjustment mode.
```rust
pub fn book_next_set_is_adjusting_angle(&mut self, enabled: bool) { ... }
pub fn read_last_set_is_adjusting_angle(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `enabled: bool` — Whether angle adjustment is enabled

---

### Lock Control

#### `book_next_lock` / `read_last_lock`
Lock (fix) the motor in place.
```rust
pub fn book_next_lock(&mut self) { ... }
pub fn read_last_lock(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_unlock` / `read_last_unlock`
Unlock the motor.
```rust
pub fn book_next_unlock(&mut self) { ... }
pub fn read_last_unlock(&self) -> Result<(), PeripheralError> { ... }
```

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Usage Example

```rust
use rust_computers_api::control_craft::dynamic_motor::*;
use rust_computers_api::peripheral::Peripheral;

let mut motor = DynamicMotor::find().unwrap();

// Configure PID and target
motor.book_next_set_pid(1.0, 0.1, 0.05);
motor.book_next_set_target_value(90.0);
wait_for_next_tick().await;
let _ = motor.read_last_set_pid();
let _ = motor.read_last_set_target_value();

// Check current angle immediately
let angle = motor.get_angle_imm().unwrap();

// Lock the motor
motor.book_next_lock();
wait_for_next_tick().await;
let _ = motor.read_last_lock();
```
