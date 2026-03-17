# Drag

**Module:** `cc_vs::drag`  
**Peripheral Type:** `vs_drag`

CC-VS Drag API for controlling ship drag, lift, and wind effects. Called from a computer placed on a Valkyrien Skies ship.

## Book-Read Methods

### `book_next_get_drag_force` / `read_last_get_drag_force`
Get the current drag force vector.
```rust
pub fn book_next_get_drag_force(&mut self) { ... }
pub fn read_last_get_drag_force(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```
**Returns:** `Option<VSVector3>`

---

### `book_next_get_lift_force` / `read_last_get_lift_force`
Get the current lift force vector.
```rust
pub fn book_next_get_lift_force(&mut self) { ... }
pub fn read_last_get_lift_force(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```
**Returns:** `Option<VSVector3>`

---

### `book_next_enable_drag` / `read_last_enable_drag`
Enable drag simulation.
```rust
pub fn book_next_enable_drag(&mut self) { ... }
pub fn read_last_enable_drag(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_disable_drag` / `read_last_disable_drag`
Disable drag simulation.
```rust
pub fn book_next_disable_drag(&mut self) { ... }
pub fn read_last_disable_drag(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_enable_lift` / `read_last_enable_lift`
Enable lift simulation.
```rust
pub fn book_next_enable_lift(&mut self) { ... }
pub fn read_last_enable_lift(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_disable_lift` / `read_last_disable_lift`
Disable lift simulation.
```rust
pub fn book_next_disable_lift(&mut self) { ... }
pub fn read_last_disable_lift(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_enable_rot_drag` / `read_last_enable_rot_drag`
Enable rotational drag.
```rust
pub fn book_next_enable_rot_drag(&mut self) { ... }
pub fn read_last_enable_rot_drag(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_disable_rot_drag` / `read_last_disable_rot_drag`
Disable rotational drag.
```rust
pub fn book_next_disable_rot_drag(&mut self) { ... }
pub fn read_last_disable_rot_drag(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_set_wind_direction` / `read_last_set_wind_direction`
Set the wind direction vector.
```rust
pub fn book_next_set_wind_direction(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_set_wind_direction(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `x, y, z: f64` — Wind direction vector components

**Returns:** `()`

---

### `book_next_set_wind_speed` / `read_last_set_wind_speed`
Set the wind speed.
```rust
pub fn book_next_set_wind_speed(&mut self, speed: f64) { ... }
pub fn read_last_set_wind_speed(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `speed: f64` — Wind speed

**Returns:** `()`

---

### `book_next_apply_wind_impulse` / `read_last_apply_wind_impulse`
Apply a wind impulse.
```rust
pub fn book_next_apply_wind_impulse(&mut self, x: f64, y: f64, z: f64, speed: f64) { ... }
pub fn read_last_apply_wind_impulse(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `x, y, z: f64` — Wind impulse direction
- `speed: f64` — Impulse speed

**Returns:** `()`

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `get_drag_force_imm`
Immediately get the drag force vector.
```rust
pub fn get_drag_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```

### `get_lift_force_imm`
Immediately get the lift force vector.
```rust
pub fn get_lift_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```

## Types

### `VSVector3`
(Imported from `cc_vs::ship`)
```rust
pub struct VSVector3 { pub x: f64, pub y: f64, pub z: f64 }
```

## Usage Example

```rust
use rust_computers_api::cc_vs::drag::*;
use rust_computers_api::peripheral::Peripheral;

let mut drag = Drag::find().unwrap();

// Enable drag and lift
drag.book_next_enable_drag();
wait_for_next_tick().await;
let _ = drag.read_last_enable_drag();

drag.book_next_enable_lift();
wait_for_next_tick().await;
let _ = drag.read_last_enable_lift();

// Set wind
drag.book_next_set_wind_direction(1.0, 0.0, 0.0);
wait_for_next_tick().await;
let _ = drag.read_last_set_wind_direction();

drag.book_next_set_wind_speed(10.0);
wait_for_next_tick().await;
let _ = drag.read_last_set_wind_speed();

// Read drag force
let force = drag.get_drag_force_imm().unwrap();
```
