# Compass

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:compass`  
**Source:** `CompassPeripheral.java`

> **Note:** The Rust struct name is `Compass`. Use `APCompass` as an alias if there is a naming conflict with other compass types.

## Overview

The Compass peripheral is a turtle upgrade that returns the direction the turtle is facing. It provides a simple way to determine turtle orientation for navigation and directional automation systems.

## Three-Function Pattern

The Compass API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
// Rust example to be added
```
## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### `getFacing()` / `book_next_get_facing()` / `read_last_get_facing()` / `async_get_facing()`

Get the direction the turtle is facing.

**Rust Signatures:**
```rust
pub fn book_next_get_facing(&mut self)
pub fn read_last_get_facing(&self) -> Result<String, PeripheralError>
pub async fn async_get_facing(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — One of `"north"`, `"south"`, `"east"`, `"west"`

**Example:**
```rust
// Rust example to be added
```
---

## Events

The Compass peripheral does not generate events.

---

## Usage Examples

### Example 1: Check Turtle Direction

```rust
// Rust example to be added
```
### Example 2: Navigate to Specific Direction

```rust
// Rust example to be added
```
### Example 3: Direction-Based Movement

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Peripheral disconnected**: The Compass is no longer accessible
- **Not a turtle**: Compass can only be used on turtles

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

None specific to this peripheral.

---

## Notes

- The Compass is a turtle upgrade and can only be used on turtles
- Directions are always one of: "north", "south", "east", "west"
- The three-function pattern allows for efficient batch operations
- Useful for navigation and directional automation

---

## Related

- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [BlockReader](./BlockReader.md) — For reading block information
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
