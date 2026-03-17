# Jet

**Module:** `control_craft::jet`  
**Peripheral Type:** `controlcraft:jet_peripheral`

Control-Craft Jet peripheral for controlling thrust output and tilt angles on jet propulsion devices.

## Book-Read Methods

### Setters

#### `book_next_set_output_thrust` / `read_last_set_output_thrust`
Set the output thrust scale.
```rust
pub fn book_next_set_output_thrust(&mut self, thrust: f64) { ... }
pub fn read_last_set_output_thrust(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `thrust: f64` — Thrust scale factor

#### `book_next_set_horizontal_tilt` / `read_last_set_horizontal_tilt`
Set the horizontal tilt angle in degrees.
```rust
pub fn book_next_set_horizontal_tilt(&mut self, angle: f64) { ... }
pub fn read_last_set_horizontal_tilt(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `angle: f64` — Horizontal tilt in degrees

#### `book_next_set_vertical_tilt` / `read_last_set_vertical_tilt`
Set the vertical tilt angle in degrees.
```rust
pub fn book_next_set_vertical_tilt(&mut self, angle: f64) { ... }
pub fn read_last_set_vertical_tilt(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `angle: f64` — Vertical tilt in degrees

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Usage Example

```rust
use rust_computers_api::control_craft::jet::*;
use rust_computers_api::peripheral::Peripheral;

let mut jet = Jet::find().unwrap();

// Set thrust and tilt
jet.book_next_set_output_thrust(1.0);
jet.book_next_set_horizontal_tilt(5.0);
jet.book_next_set_vertical_tilt(-10.0);
wait_for_next_tick().await;
let _ = jet.read_last_set_output_thrust();
let _ = jet.read_last_set_horizontal_tilt();
let _ = jet.read_last_set_vertical_tilt();
```
