# DigitalAdapter

**Module:** `createaddition::digital_adapter`  
**Peripheral Type:** `createaddition:digital_adapter`

Create Additions Digital Adapter peripheral. A versatile peripheral that provides display control, kinetic network management, machine state reading (pulleys, pistons, bearings, elevators), and utility calculations for Create mod contraptions.

## Book-Read Methods

### Display Operations

#### `book_next_clear_line` / `read_last_clear_line`
Clear a specific display line.
```rust
pub fn book_next_clear_line(&mut self, line: i32)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
```
**Parameters:**
- `line: i32` — Line number to clear

**Returns:** `()`

---

#### `book_next_clear` / `read_last_clear`
Clear all display lines.
```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
```
**Returns:** `()`

---

#### `book_next_print` / `read_last_print`
Print text to the next available line.
```rust
pub fn book_next_print(&mut self, text: &str)
pub fn read_last_print(&self) -> Result<(), PeripheralError>
```
**Parameters:**
- `text: &str` — Text to print

**Returns:** `()`

---

#### `book_next_get_line` / `read_last_get_line`
Get the text on a specific line.
```rust
pub fn book_next_get_line(&mut self, line: i32)
pub fn read_last_get_line(&self) -> Result<String, PeripheralError>
```
**Parameters:**
- `line: i32` — Line number

**Returns:** `String`

---

#### `book_next_set_line` / `read_last_set_line`
Set the text on a specific line.
```rust
pub fn book_next_set_line(&mut self, line: i32, text: &str)
pub fn read_last_set_line(&self) -> Result<(), PeripheralError>
```
**Parameters:**
- `line: i32` — Line number
- `text: &str` — Text to set

**Returns:** `()`

---

#### `book_next_get_max_lines` / `read_last_get_max_lines`
Get the maximum number of display lines.
```rust
pub fn book_next_get_max_lines(&mut self)
pub fn read_last_get_max_lines(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

### Kinetic Control

#### `book_next_set_target_speed` / `read_last_set_target_speed`
Set the target speed for a mechanical component on a given side.
```rust
pub fn book_next_set_target_speed(&mut self, dir: &str, speed: f64)
pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError>
```
**Parameters:**
- `dir: &str` — Direction (e.g. `"north"`, `"up"`)
- `speed: f64` — Target RPM

**Returns:** `()`

---

#### `book_next_get_target_speed` / `read_last_get_target_speed`
Get the target speed for a direction.
```rust
pub fn book_next_get_target_speed(&mut self, dir: &str)
pub fn read_last_get_target_speed(&self) -> Result<f64, PeripheralError>
```
**Parameters:**
- `dir: &str` — Direction

**Returns:** `f64`

---

#### `book_next_get_kinetic_stress` / `read_last_get_kinetic_stress`
Get the stress (SU) on a given side.
```rust
pub fn book_next_get_kinetic_stress(&mut self, dir: &str)
pub fn read_last_get_kinetic_stress(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_kinetic_capacity` / `read_last_get_kinetic_capacity`
Get the stress capacity (SU) on a given side.
```rust
pub fn book_next_get_kinetic_capacity(&mut self, dir: &str)
pub fn read_last_get_kinetic_capacity(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_kinetic_speed` / `read_last_get_kinetic_speed`
Get the actual speed (RPM) on a given side.
```rust
pub fn book_next_get_kinetic_speed(&mut self, dir: &str)
pub fn read_last_get_kinetic_speed(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_kinetic_top_speed` / `read_last_get_kinetic_top_speed`
Get the maximum speed (RPM) of the kinetic network.
```rust
pub fn book_next_get_kinetic_top_speed(&mut self)
pub fn read_last_get_kinetic_top_speed(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### Machine State

#### `book_next_get_pulley_distance` / `read_last_get_pulley_distance`
Get the extension distance of a pulley (blocks).
```rust
pub fn book_next_get_pulley_distance(&mut self, dir: &str)
pub fn read_last_get_pulley_distance(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_piston_distance` / `read_last_get_piston_distance`
Get the extension distance of a piston (blocks).
```rust
pub fn book_next_get_piston_distance(&mut self, dir: &str)
pub fn read_last_get_piston_distance(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_bearing_angle` / `read_last_get_bearing_angle`
Get the bearing angle (degrees).
```rust
pub fn book_next_get_bearing_angle(&mut self, dir: &str)
pub fn read_last_get_bearing_angle(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_elevator_floor` / `read_last_get_elevator_floor`
Get the current elevator floor number.
```rust
pub fn book_next_get_elevator_floor(&mut self, dir: &str)
pub fn read_last_get_elevator_floor(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_has_elevator_arrived` / `read_last_has_elevator_arrived`
Check if the elevator has arrived at its destination floor.
```rust
pub fn book_next_has_elevator_arrived(&mut self, dir: &str)
pub fn read_last_has_elevator_arrived(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

#### `book_next_get_elevator_floors` / `read_last_get_elevator_floors`
Get the total number of elevator floors.
```rust
pub fn book_next_get_elevator_floors(&mut self, dir: &str)
pub fn read_last_get_elevator_floors(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_elevator_floor_name` / `read_last_get_elevator_floor_name`
Get the name of a specific floor.
```rust
pub fn book_next_get_elevator_floor_name(&mut self, dir: &str, index: i32)
pub fn read_last_get_elevator_floor_name(&self) -> Result<String, PeripheralError>
```
**Parameters:**
- `dir: &str` — Direction
- `index: i32` — Floor index

**Returns:** `String`

---

#### `book_next_goto_elevator_floor` / `read_last_goto_elevator_floor`
Move the elevator to a specified floor. Returns the Y offset.
```rust
pub fn book_next_goto_elevator_floor(&mut self, dir: &str, index: i32)
pub fn read_last_goto_elevator_floor(&self) -> Result<f64, PeripheralError>
```
**Parameters:**
- `dir: &str` — Direction
- `index: i32` — Target floor index

**Returns:** `f64` — Y coordinate delta

---

### Utility

#### `book_next_get_duration_angle` / `read_last_get_duration_angle`
Calculate the time (seconds) to rotate a given angle at a given RPM.
```rust
pub fn book_next_get_duration_angle(&mut self, degrees: f64, rpm: f64)
pub fn read_last_get_duration_angle(&self) -> Result<f64, PeripheralError>
```
**Parameters:**
- `degrees: f64` — Rotation angle
- `rpm: f64` — Speed in RPM

**Returns:** `f64` — Duration in seconds

---

#### `book_next_get_duration_distance` / `read_last_get_duration_distance`
Calculate the time (seconds) to move a given distance at a given RPM.
```rust
pub fn book_next_get_duration_distance(&mut self, blocks: f64, rpm: f64)
pub fn read_last_get_duration_distance(&self) -> Result<f64, PeripheralError>
```
**Parameters:**
- `blocks: f64` — Distance in blocks
- `rpm: f64` — Speed in RPM

**Returns:** `f64` — Duration in seconds

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::createaddition::DigitalAdapter;
use rust_computers_api::peripheral::Peripheral;

let mut adapter = DigitalAdapter::wrap(addr);

// Set speed and read state
adapter.book_next_set_target_speed("north", 64.0);
adapter.book_next_get_kinetic_speed("north");
wait_for_next_tick().await;

loop {
    let speed = adapter.read_last_get_kinetic_speed();
    let angle = adapter.read_last_get_bearing_angle();

    adapter.book_next_get_kinetic_speed("north");
    adapter.book_next_get_bearing_angle("north");
    wait_for_next_tick().await;
}
```
