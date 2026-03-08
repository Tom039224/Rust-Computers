# ModularAccumulator

**Module:** `createaddition::modular_accumulator`  
**Peripheral Type:** `createaddition:modular_accumulator`

Create Additions Modular Accumulator peripheral. A scalable multiblock energy storage. Provides access to energy levels, capacity, charge percentage, transfer rates, and physical dimensions.

## Book-Read Methods

### `book_next_get_energy` / `read_last_get_energy`
Get the current stored energy (FE).
```rust
pub fn book_next_get_energy(&mut self)
pub fn read_last_get_energy(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_capacity` / `read_last_get_capacity`
Get the maximum energy capacity (FE).
```rust
pub fn book_next_get_capacity(&mut self)
pub fn read_last_get_capacity(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_percent` / `read_last_get_percent`
Get the charge percentage (0.0–100.0%).
```rust
pub fn book_next_get_percent(&mut self)
pub fn read_last_get_percent(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_max_insert` / `read_last_get_max_insert`
Get the maximum input rate (FE/t).
```rust
pub fn book_next_get_max_insert(&mut self)
pub fn read_last_get_max_insert(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_max_extract` / `read_last_get_max_extract`
Get the maximum output rate (FE/t).
```rust
pub fn book_next_get_max_extract(&mut self)
pub fn read_last_get_max_extract(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_height` / `read_last_get_height`
Get the multiblock height (blocks).
```rust
pub fn book_next_get_height(&mut self)
pub fn read_last_get_height(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

### `book_next_get_width` / `read_last_get_width`
Get the multiblock width (blocks).
```rust
pub fn book_next_get_width(&mut self)
pub fn read_last_get_width(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::createaddition::ModularAccumulator;
use rust_computers_api::peripheral::Peripheral;

let mut acc = ModularAccumulator::wrap(addr);

loop {
    let energy = acc.read_last_get_energy();
    let percent = acc.read_last_get_percent();

    acc.book_next_get_energy();
    acc.book_next_get_percent();
    wait_for_next_tick().await;
}
```
