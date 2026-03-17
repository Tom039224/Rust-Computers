# EnergyDetector

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:energy_detector`  
**Source:** `EnergyDetectorPeripheral.java`

## Overview

The EnergyDetector peripheral monitors energy transfer rates between adjacent blocks and allows setting transfer rate limits. It provides real-time information about energy flow and can be used to prevent energy network overloads or to monitor power consumption patterns. This is useful for energy management systems and power distribution automation.

## Three-Function Pattern

The EnergyDetector API uses the three-function pattern for all methods:

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

### `getTransferRate()` / `book_next_get_transfer_rate()` / `read_last_get_transfer_rate()` / `async_get_transfer_rate()`

Get the current energy transfer rate in FE/t (Forge Energy per tick).

**Rust Signatures:**
```rust
pub fn book_next_get_transfer_rate(&mut self)
pub fn read_last_get_transfer_rate(&self) -> Result<f64, PeripheralError>
pub async fn async_get_transfer_rate(&self) -> Result<f64, PeripheralError>
```

**Returns:** `number` — Current transfer rate in FE/t

**Example:**
```rust
// Rust example to be added
```
---

### `getTransferRateLimit()` / `book_next_get_transfer_rate_limit()` / `read_last_get_transfer_rate_limit()` / `async_get_transfer_rate_limit()`

Get the current transfer rate limit in FE/t.

**Rust Signatures:**
```rust
pub fn book_next_get_transfer_rate_limit(&mut self)
pub fn read_last_get_transfer_rate_limit(&self) -> Result<f64, PeripheralError>
pub async fn async_get_transfer_rate_limit(&self) -> Result<f64, PeripheralError>
```

**Returns:** `number` — Transfer rate limit in FE/t

**Example:**
```rust
// Rust example to be added
```
---

### `setTransferRateLimit(rate)` / `book_next_set_transfer_rate_limit(rate)` / `read_last_set_transfer_rate_limit()` / `async_set_transfer_rate_limit(rate)`

Set the maximum transfer rate limit in FE/t.

**Rust Signatures:**
```rust
pub fn book_next_set_transfer_rate_limit(&mut self, rate: f64)
pub fn read_last_set_transfer_rate_limit(&self) -> Result<(), PeripheralError>
pub async fn async_set_transfer_rate_limit(&self, rate: f64) -> Result<(), PeripheralError>
```

**Parameters:**
- `rate: number` — Maximum transfer rate in FE/t

**Returns:** `boolean` — `true` if limit was set successfully

**Example:**
```rust
// Rust example to be added
```
---

## Events

The EnergyDetector peripheral does not generate events.

---

## Usage Examples

### Example 1: Monitor Energy Transfer

```rust
// Rust example to be added
```
### Example 2: Dynamic Rate Limiting

```rust
// Rust example to be added
```
### Example 3: Energy Flow Monitoring

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Peripheral disconnected**: The EnergyDetector is no longer accessible
- **Invalid rate**: Transfer rate limit is negative or exceeds maximum
- **No adjacent energy blocks**: No energy-capable blocks adjacent to the detector

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

None specific to this peripheral.

---

## Notes

- Transfer rates are measured in FE/t (Forge Energy per tick)
- The detector measures energy flow between adjacent blocks
- Setting a rate limit can prevent energy network overloads
- The three-function pattern allows for efficient batch operations
- Energy transfer is measured in real-time and updates each tick

---

## Related

- [BlockReader](./BlockReader.md) — For reading block information
- [EnvironmentDetector](./EnvironmentDetector.md) — For environmental information
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
