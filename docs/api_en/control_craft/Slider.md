# Slider

**Module:** `control_craft::slider`  
**Peripheral Type:** `controlcraft:slider_peripheral`

Control-Craft Slider peripheral for linear motion control with PID, force output, target value management, and lock/unlock functionality.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_distance` | `read_last_get_distance` | `get_distance_imm` | `f64` |
| `book_next_get_current_value` | `read_last_get_current_value` | `get_current_value_imm` | `f64` |
| `book_next_get_target_value` | `read_last_get_target_value` | `get_target_value_imm` | `f64` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |
| `book_next_is_locked` | `read_last_is_locked` | `is_locked_imm` | `bool` |

---

### Setters

#### `book_next_set_output_force` / `read_last_set_output_force`
Set the output force scale.
```rust
pub fn book_next_set_output_force(&mut self, scale: f64) { ... }
pub fn read_last_set_output_force(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `scale: f64` — Force scale factor

#### `book_next_set_pid` / `read_last_set_pid`
Set the PID gains for slider control.
```rust
pub fn book_next_set_pid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_pid(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `p: f64` — Proportional gain, `i: f64` — Integral gain, `d: f64` — Derivative gain

#### `book_next_set_target_value` / `read_last_set_target_value`
Set the target value for slider position.
```rust
pub fn book_next_set_target_value(&mut self, target: f64) { ... }
pub fn read_last_set_target_value(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `target: f64` — Target position value

---

### Lock Control

#### `book_next_lock` / `read_last_lock`
Lock (fix) the slider in place.
```rust
pub fn book_next_lock(&mut self) { ... }
pub fn read_last_lock(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_unlock` / `read_last_unlock`
Unlock the slider.
```rust
pub fn book_next_unlock(&mut self) { ... }
pub fn read_last_unlock(&self) -> Result<(), PeripheralError> { ... }
```

## Usage Example

```rust
use rust_computers_api::control_craft::slider::*;
use rust_computers_api::peripheral::Peripheral;

let mut slider = Slider::find().unwrap();

// Configure PID and target
slider.book_next_set_pid(1.0, 0.1, 0.05);
slider.book_next_set_target_value(5.0);
wait_for_next_tick().await;
let _ = slider.read_last_set_pid();
let _ = slider.read_last_set_target_value();

// Check current distance immediately
let dist = slider.get_distance_imm().unwrap();

// Lock the slider
slider.book_next_lock();
wait_for_next_tick().await;
let _ = slider.read_last_lock();
```
