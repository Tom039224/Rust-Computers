# ElectricMotor

**Module:** `createaddition::electric_motor`  
**Peripheral Type:** `createaddition:electric_motor`

Create Additions Electric Motor peripheral for controlling Create mod rotational components. Supports speed control, rotation/translation commands, and energy monitoring.

## Book-Read Methods

### `book_next_get_type` / `read_last_get_type`
Get the peripheral type name.
```rust
pub fn book_next_get_type(&mut self) { ... }
pub fn read_last_get_type(&self) -> Result<String, PeripheralError> { ... }
```
**Returns:** `String`

---

### `book_next_set_speed` / `read_last_set_speed`
Set the motor RPM. Sign controls direction.
```rust
pub fn book_next_set_speed(&mut self, speed: f64) { ... }
pub fn read_last_set_speed(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `speed: f64` — RPM value (positive = one direction, negative = reverse)

**Returns:** `()`

---

### `book_next_stop` / `read_last_stop`
Stop the motor.
```rust
pub fn book_next_stop(&mut self) { ... }
pub fn read_last_stop(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_get_speed` / `read_last_get_speed`
Get the current speed.
```rust
pub fn book_next_get_speed(&mut self) { ... }
pub fn read_last_get_speed(&self) -> Result<f64, PeripheralError> { ... }
```
**Returns:** `f64` — Current RPM

---

### `book_next_get_stress_capacity` / `read_last_get_stress_capacity`
Get the stress capacity.
```rust
pub fn book_next_get_stress_capacity(&mut self) { ... }
pub fn read_last_get_stress_capacity(&self) -> Result<f64, PeripheralError> { ... }
```
**Returns:** `f64`

---

### `book_next_get_energy_consumption` / `read_last_get_energy_consumption`
Get the current energy consumption.
```rust
pub fn book_next_get_energy_consumption(&mut self) { ... }
pub fn read_last_get_energy_consumption(&self) -> Result<f64, PeripheralError> { ... }
```
**Returns:** `f64`

---

### `book_next_rotate` / `read_last_rotate`
Rotate by a specified number of degrees. Returns the time in seconds required.
```rust
pub fn book_next_rotate(&mut self, degrees: f64, rpm: Option<f64>) { ... }
pub fn read_last_rotate(&self) -> Result<f64, PeripheralError> { ... }
```
**Parameters:**
- `degrees: f64` — Rotation angle in degrees
- `rpm: Option<f64>` — RPM to use for rotation (optional)

**Returns:** `f64` — Duration in seconds

---

### `book_next_translate` / `read_last_translate`
Translate by a specified distance. Returns the time in seconds required.
```rust
pub fn book_next_translate(&mut self, distance: f64, rpm: Option<f64>) { ... }
pub fn read_last_translate(&self) -> Result<f64, PeripheralError> { ... }
```
**Parameters:**
- `distance: f64` — Distance to translate
- `rpm: Option<f64>` — RPM to use (optional)

**Returns:** `f64` — Duration in seconds

---

### `book_next_get_max_insert` / `read_last_get_max_insert`
Get the maximum energy insertion rate.
```rust
pub fn book_next_get_max_insert(&mut self) { ... }
pub fn read_last_get_max_insert(&self) -> Result<f64, PeripheralError> { ... }
```
**Returns:** `f64`

---

### `book_next_get_max_extract` / `read_last_get_max_extract`
Get the maximum energy extraction rate.
```rust
pub fn book_next_get_max_extract(&mut self) { ... }
pub fn read_last_get_max_extract(&self) -> Result<f64, PeripheralError> { ... }
```
**Returns:** `f64`

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `get_type_imm`
Immediately get the peripheral type name.
```rust
pub fn get_type_imm(&self) -> Result<String, PeripheralError> { ... }
```

## Usage Example

```rust
use rust_computers_api::createaddition::electric_motor::*;
use rust_computers_api::peripheral::Peripheral;

let mut motor = ElectricMotor::find().unwrap();

// Set speed to 64 RPM
motor.book_next_set_speed(64.0);
wait_for_next_tick().await;
let _ = motor.read_last_set_speed();

// Rotate 90 degrees
motor.book_next_rotate(90.0, Some(32.0));
wait_for_next_tick().await;
let duration = motor.read_last_rotate().unwrap();

// Check energy consumption
motor.book_next_get_energy_consumption();
wait_for_next_tick().await;
let consumption = motor.read_last_get_energy_consumption().unwrap();

// Stop
motor.book_next_stop();
wait_for_next_tick().await;
let _ = motor.read_last_stop();
```
