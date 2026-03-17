# LinkBridge

**Module:** `control_craft::link_bridge`  
**Peripheral Type:** `controlcraft:link_bridge_peripheral`

Control-Craft Link Bridge peripheral for setting indexed input values and reading indexed output values. Provides a bridge interface for passing numeric data between linked components.

## Book-Read Methods

### Input

#### `book_next_set_input` / `read_last_set_input`
Set the input value at a specified index.
```rust
pub fn book_next_set_input(&mut self, index: f64, value: f64) { ... }
pub fn read_last_set_input(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `index: f64` — Input index, `value: f64` — Value to set

---

### Output (with imm support)

#### `book_next_get_output` / `read_last_get_output`
Get the output value at a specified index (book-read pattern).
```rust
pub fn book_next_get_output(&mut self, index: f64) { ... }
pub fn read_last_get_output(&self) -> Result<f64, PeripheralError> { ... }
```
**Parameters:** `index: f64` — Output index  
**Returns:** `f64`

#### `get_output_imm`
Get the output value at a specified index immediately.
```rust
pub fn get_output_imm(&self, index: f64) -> Result<f64, PeripheralError> { ... }
```
**Parameters:** `index: f64` — Output index  
**Returns:** `f64`

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Usage Example

```rust
use rust_computers_api::control_craft::link_bridge::*;
use rust_computers_api::peripheral::Peripheral;

let mut bridge = LinkBridge::find().unwrap();

// Set input value at index 0
bridge.book_next_set_input(0.0, 42.0);
wait_for_next_tick().await;
let _ = bridge.read_last_set_input();

// Read output value at index 1 immediately
let output = bridge.get_output_imm(1.0).unwrap();
```
