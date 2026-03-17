# PortableEnergyInterface

**Module:** `createaddition::portable_energy_interface`  
**Peripheral Type:** `createaddition:portable_energy_interface`

Create Additions Portable Energy Interface peripheral. Transfers energy to and from Create contraptions. Provides access to buffer energy level, capacity, connection status, and transfer rates.

## Book-Read Methods

### `book_next_get_energy` / `read_last_get_energy`
Get the buffer energy level (FE).
```rust
pub fn book_next_get_energy(&mut self)
pub fn read_last_get_energy(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_capacity` / `read_last_get_capacity`
Get the buffer maximum capacity (FE).
```rust
pub fn book_next_get_capacity(&mut self)
pub fn read_last_get_capacity(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_is_connected` / `read_last_is_connected`
Check if a contraption is currently connected.
```rust
pub fn book_next_is_connected(&mut self)
pub fn read_last_is_connected(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

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

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::createaddition::PortableEnergyInterface;
use rust_computers_api::peripheral::Peripheral;

let mut pei = PortableEnergyInterface::wrap(addr);

loop {
    let connected = pei.read_last_is_connected();
    let energy = pei.read_last_get_energy();

    pei.book_next_is_connected();
    pei.book_next_get_energy();
    wait_for_next_tick().await;
}
```
