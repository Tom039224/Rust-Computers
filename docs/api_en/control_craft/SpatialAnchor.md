# SpatialAnchor

**Module:** `control_craft::spatial_anchor`  
**Peripheral Type:** `controlcraft:spatial_anchor_peripheral`

Control-Craft Spatial Anchor peripheral for controlling ship static state, anchor running state, offset distance, position/rotation PID gains, and communication channel.

## Book-Read Methods

### Setters

#### `book_next_set_static` / `read_last_set_static`
Set the ship to a static (immovable) state.
```rust
pub fn book_next_set_static(&mut self, enabled: bool) { ... }
pub fn read_last_set_static(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `enabled: bool` — Whether the ship is static

#### `book_next_set_running` / `read_last_set_running`
Set the anchor running state.
```rust
pub fn book_next_set_running(&mut self, enabled: bool) { ... }
pub fn read_last_set_running(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `enabled: bool` — Whether the anchor is running

#### `book_next_set_offset` / `read_last_set_offset`
Set the anchor offset distance.
```rust
pub fn book_next_set_offset(&mut self, offset: f64) { ... }
pub fn read_last_set_offset(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `offset: f64` — Offset distance

#### `book_next_set_ppid` / `read_last_set_ppid`
Set the position control (PPID) gains.
```rust
pub fn book_next_set_ppid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_ppid(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `p: f64` — Proportional gain, `i: f64` — Integral gain, `d: f64` — Derivative gain

#### `book_next_set_qpid` / `read_last_set_qpid`
Set the rotation control (QPID) gains.
```rust
pub fn book_next_set_qpid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_qpid(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `p: f64` — Proportional gain, `i: f64` — Integral gain, `d: f64` — Derivative gain

#### `book_next_set_channel` / `read_last_set_channel`
Set the communication channel number.
```rust
pub fn book_next_set_channel(&mut self, channel: i64) { ... }
pub fn read_last_set_channel(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `channel: i64` — Channel number

## Usage Example

```rust
use rust_computers_api::control_craft::spatial_anchor::*;
use rust_computers_api::peripheral::Peripheral;

let mut anchor = SpatialAnchor::find().unwrap();

// Configure anchor
anchor.book_next_set_running(true);
anchor.book_next_set_offset(2.0);
anchor.book_next_set_ppid(1.0, 0.1, 0.05);
anchor.book_next_set_qpid(0.5, 0.05, 0.02);
wait_for_next_tick().await;
let _ = anchor.read_last_set_running();
let _ = anchor.read_last_set_offset();
let _ = anchor.read_last_set_ppid();
let _ = anchor.read_last_set_qpid();

// Set static mode
anchor.book_next_set_static(true);
wait_for_next_tick().await;
let _ = anchor.read_last_set_static();
```
