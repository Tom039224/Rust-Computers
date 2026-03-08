# EnergyDetector

**Module:** `advanced_peripherals::energy_detector`  
**Peripheral Type:** `advancedPeripherals:energy_detector`

AdvancedPeripherals Energy Detector peripheral. Monitors energy transfer rates and allows setting transfer rate limits.

## Book-Read Methods

### `book_next_get_transfer_rate` / `read_last_get_transfer_rate`
Get the current energy transfer rate (FE/t).
```rust
pub fn book_next_get_transfer_rate(&mut self)
pub fn read_last_get_transfer_rate(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64` — Transfer rate in FE/t

---

### `book_next_get_transfer_rate_limit` / `read_last_get_transfer_rate_limit`
Get the current transfer rate limit (FE/t).
```rust
pub fn book_next_get_transfer_rate_limit(&mut self)
pub fn read_last_get_transfer_rate_limit(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64` — Rate limit in FE/t

---

### `book_next_set_transfer_rate_limit` / `read_last_set_transfer_rate_limit`
Set the maximum transfer rate limit (FE/t).
```rust
pub fn book_next_set_transfer_rate_limit(&mut self, rate: f64)
pub fn read_last_set_transfer_rate_limit(&self) -> Result<(), PeripheralError>
```
**Parameters:**
- `rate: f64` — Maximum transfer rate in FE/t

**Returns:** `()`

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::EnergyDetector;
use rust_computers_api::peripheral::Peripheral;

let mut detector = EnergyDetector::wrap(addr);

// Set rate limit
detector.book_next_set_transfer_rate_limit(10000.0);
wait_for_next_tick().await;

loop {
    let rate = detector.read_last_get_transfer_rate();

    detector.book_next_get_transfer_rate();
    wait_for_next_tick().await;
}
```
