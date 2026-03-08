# Compass (APCompass)

**Module:** `advanced_peripherals::compass`  
**Peripheral Type:** `advancedPeripherals:compass`

AdvancedPeripherals Compass peripheral (turtle upgrade). Returns the direction the turtle is facing.

> **Note:** The Rust struct name is `Compass`. Use `APCompass` as an alias if there is a naming conflict with other compass types.

## Book-Read Methods

### `book_next_get_facing` / `read_last_get_facing`
Get the direction the turtle is facing.
```rust
pub fn book_next_get_facing(&mut self)
pub fn read_last_get_facing(&self) -> Result<String, PeripheralError>
```
**Returns:** `String` — One of `"north"`, `"south"`, `"east"`, `"west"`

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::Compass;
use rust_computers_api::peripheral::Peripheral;

let mut compass = Compass::wrap(addr);

loop {
    let facing = compass.read_last_get_facing();

    compass.book_next_get_facing();
    wait_for_next_tick().await;
}
```
