# Core Infrastructure API

**v0.2.0+ (book-read pattern)**

## Overview

The core infrastructure that drives execution control and peripheral communication for Rust Computers.  
The book-read pattern enables efficient programming where **1 loop iteration = 1 tick**.

## Core Components

| Component | Description |
|---|---|
| `wait_for_next_tick()` | A Future that waits until the next Game Tick. Batch-issues all booked requests via FFI and collects results |
| `book_next_*()` / `read_last_*()` | Peripheral method pairs. Book a request / read the result |
| BookStore | Global state management (pending/in_flight/results) |
| PeriphAddr | Peripheral address (u32: 0–5 = directly adjacent, 6+ = wired modem) |

---

## Execution Flow

### Per-Tick Behavior

```
GT N [Rust]
  ├─ read_last_*()     ← Retrieve previous tick's results
  ├─ Computation
  └─ book_next_*()     ← Book requests for the next tick

  [Await Point]
  wait_for_next_tick().await
    ├─ 1st Poll: Issue pending FFI calls (flush)
    └─ Subsequent Polls: Poll for results (poll_all)

GT N+1 [Java]
  ├─ Execute requests
  └─ Store results in the BookStore results map

GT N+1 [Rust]
  ├─ Results available via read_last_*()
  └─ Loop continues
```

### Memory Timeline

```
GT N:
  ┌──────────────────┐
  │ book_next_A()    │  ← pending {A}
  │ book_next_B()    │  ← pending {A, B}
  │ wait_for...await │
  └──────────────────┘
        ↓ (first poll)
  ┌──────────────────┐
  │ flush()          │  ← in_flight {A, B} (FFI issued)
  │ Poll::Pending    │
  └──────────────────┘

GT N+1:
  ┌──────────────────┐
  │ wait_for...await │
  └──────────────────┘
        ↓ (poll again)
  ┌──────────────────┐
  │ poll_all()       │  ← results {A→data, B→data}
  │ Poll::Ready(())  │
  └──────────────────┘
```

---

## API Reference

### `wait_for_next_tick()`

Returns a Future that waits until the next Game Tick.

```rust
pub fn wait_for_next_tick() -> WaitForNextTickFuture
```

**Usage Pattern:**
```rust
loop {
    // Retrieve previous tick's results
    let data = sensor.read_last_get_data()?;
    
    // Book a request
    sensor.book_next_get_data();
    
    // Wait for the next tick (batch-issues all booked requests via FFI)
    wait_for_next_tick().await;
}
```

**Internal Behavior:**
- **1st poll**: `BookStore::flush()` — moves pending → in_flight, issues FFI calls (`host_request_info` / `host_do_action`)
- **Subsequent polls**: `BookStore::poll_all()` — polls all requests via `host_poll_result`, moves completed ones to the `results` map
- **All complete**: Returns `Poll::Ready(())`, allowing the loop to proceed

---

### `PeriphAddr`

Represents a peripheral's address.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeriphAddr(u32);

impl PeriphAddr {
    pub fn raw(&self) -> u32 { self.0 }
}

impl From<Direction> for PeriphAddr { ... }
```

**Value Meanings:**
| Value | Connection Type | Description |
|---|---|---|
| 0–5 | Directly adjacent | `Direction::{Down, Up, North, South, West, East}` |
| 6+ | Wired modem | Peripheral connected via wired network |

**Examples:**
```rust
// Directly adjacent
let monitor = Monitor::from_direction(Direction::South);  // PeriphAddr(3)

// Via wired modem (auto-discovered with find_imm)
let all_radars = find_imm::<Radar>()?;  // PeriphAddr(6), PeriphAddr(7), ...
```

---

### Book-Read Method Pairs

All peripheral operations consist of `book_next_*` and `read_last_*` pairs.

#### `book_next_*(&mut self, ...)`

**Books** a request for the next tick (recorded in a local buffer, not an FFI call).

```rust
pub fn book_next_get_data(&mut self) { ... }
pub fn book_next_set_speed(&mut self, speed: f64) { ... }
```

**Characteristics:**
- Returns immediately (non-blocking)
- Requires `&mut self`
- Multiple bookings for the same method are **overwritten** (last booking wins)
- FFI calls are only issued when `wait_for_next_tick().await` is reached

**Example:**
```rust
// Book multiple requests
radar.book_next_scan_for_entities(100.0);
motor.book_next_set_speed(50.0);

// Wait
wait_for_next_tick().await;

// Both results are now available
let entities = radar.read_last_scan_for_entities()?;
let ack = motor.read_last_set_speed()?;
```

#### `read_last_*(&self) -> Result<T, PeripheralError>`

Reads the **result** of a request from the previous tick.

```rust
pub fn read_last_get_data(&self) -> Result<Data, PeripheralError> { ... }
pub fn read_last_set_speed(&self) -> Result<(), PeripheralError> { ... }
```

**Return Values:**
- `Ok(data)`: Request succeeded, returns the result
- `Err(PeripheralError::NotRequested)`: `book_next_*` was not called beforehand
- `Err(PeripheralError::Bridge(_))`: FFI error
- `Err(PeripheralError::DecodeFailed)`: Failed to parse the result

**Example:**
```rust
// First loop iteration
match radar.read_last_scan_for_entities() {
    Ok(entities) => { /* process */ },
    Err(PeripheralError::NotRequested) => { /* first iteration, ignore */ },
    Err(e) => { eprintln!("Error: {}", e); },
}

// Subsequent iterations
let entities = radar.read_last_scan_for_entities()?;  // Ok expected
```

---

### Peripheral Discovery / Wrapping

#### `find_imm<T>()`

Searches for peripherals of a given type (immediate).

```rust
pub fn find_imm<T: Peripheral>() -> Result<Vec<T>, PeripheralError>
```

**Usage:**
```rust
// Find all monitors
let monitors: Vec<Monitor> = find_imm()?;

// Get a single peripheral
let radar = find_imm::<Radar>()?.into_iter().next().ok_or(...)?;

// Get the first N
let (motor, _rest) = find_imm::<ElectricMotor>()?
    .into_iter()
    .next()
    .map(|m| (m, ()))
    .ok_or(PeripheralError::NotFound)?;
```

**Notes:**
- Includes peripherals connected via wired modem
- Optimized for speed in v0.2.0 (`host_find_peripherals_by_type_imm`)

#### `wrap_imm(periph_addr: PeriphAddr)` / `wrap(periph_addr: PeriphAddr)`

Wraps a peripheral at a known address.

```rust
pub fn wrap_imm(addr: PeriphAddr) -> Result<T, PeripheralError>
pub fn wrap(addr: PeriphAddr) -> T  // unchecked
```

**Examples:**
```rust
// Peripheral via wired modem
let radar_6 = wrap_imm::<Radar>(PeriphAddr(6))?;  // Checks existence
let radar_7 = wrap::<Radar>(PeriphAddr(7));       // No existence check
```

---

## Peripheral Errors

```rust
pub enum PeripheralError {
    Bridge(BridgeError),        // FFI layer error
    NotFound,                   // Peripheral not found
    DecodeFailed,               // Failed to parse result
    Unexpected(String),         // Other
    NotRequested,               // read_last called without prior book_next
}
```

### Handling the NotRequested Error

On the first loop iteration, `book_next_*` has not yet been called, so `read_last_*` returns `NotRequested`.

```rust
loop {
    // First iteration: NotRequested, subsequent iterations: Ok(data)
    let data = match sensor.read_last_get_data() {
        Ok(d) => d,
        Err(PeripheralError::NotRequested) => {
            // First iteration — no result yet
            Default::default()
        },
        Err(e) => return Err(e),
    };

    // Book request
    sensor.book_next_get_data();
    wait_for_next_tick().await;
}
```

**Common Patterns:**
```rust
// Treat as Option
let data = sensor.read_last_get_data().ok();

// Provide a default value with unwrap_or
let data = sensor.read_last_get_data().unwrap_or_default();

// Propagate with ? (NotRequested propagates as Err)
let data = sensor.read_last_get_data()?;
```

---

## Immediate Methods (`_imm`)

Methods with the `_imm` suffix execute immediately within the same tick (no await needed).

```rust
// Returns within the same tick (e.g., for checking peripheral connection)
let kind = radar.get_type_imm()?;

// Write results are also finalized within the same tick
let written = inventory.push_items_imm(
    InventoryRef { side: Direction::South },
    &items,
    &[],
)?;
```

**Characteristics:**
- Intentional exception to the 1-tick delay principle
- Implemented via `host_request_info_imm` / `host_do_action_imm`
- Does not require the book-read pattern

---

## Event-Waiting Methods (async)

Event-driven methods such as `receive_wait_raw` are provided as `async` functions.

```rust
// Wait until an event occurs
let msg = modem.receive_wait_raw().await?;
```

**Internal Implementation:**
- Internally loops through book → `wait_for_next_tick()` → read
- Appears as `async` to the user
- Can wait for multiple events simultaneously with `parallel!`

```rust
let (msg, keystroke) = parallel!(
    modem.receive_wait_raw(),
    keyboard.read_line()
).await?;
```

---

## Complete Example

```rust
use rust_computers_api::prelude::*;

#[entry]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut radar = find_imm::<Radar>()?
        .into_iter()
        .next()
        .ok_or(PeripheralError::NotFound)?;
    
    let mut motor = find_imm::<ElectricMotor>()?
        .into_iter()
        .next()
        .ok_or(PeripheralError::NotFound)?;

    // Initial booking
    radar.book_next_scan_for_entities(100.0);
    wait_for_next_tick().await;

    loop {
        // Retrieve previous tick's results
        let entities = match radar.read_last_scan_for_entities() {
            Ok(e) => e,
            Err(PeripheralError::NotRequested) => Vec::new(),
            Err(e) => return Err(e.into()),
        };

        // Computation
        let target_speed = if entities.is_empty() {
            0.0
        } else {
            entities[0].distance * 10.0
        };

        // Configure
        motor.book_next_set_speed(target_speed);
        
        // Book scan
        radar.book_next_scan_for_entities(100.0);

        // Wait for next tick
        wait_for_next_tick().await;

        // Log output (_imm)
        let speed = motor.get_speed_imm()?;
        println!("Speed: {:.2}", speed);
    }
}
```

---

## Performance Characteristics

| Operation | Cost | Description |
|---|---|---|
| `book_next_*` | O(1) | Records to local buffer |
| `read_last_*` | O(1) | Global map lookup |
| `wait_for_next_tick()` Poll 1 | FFI call | Issues all pending requests |
| `wait_for_next_tick()` Poll 2+ | FFI call | Polls for results |
| `_imm` | FFI call | Immediate FFI call |

**Optimizations:**
- `book_*` involves no FFI calls (purely local)
- The first poll of `wait_for_next_tick()` **batch-issues all requests via FFI** (parallelized)
- `read_last_*` is a memory lookup only

---

## Notes

### Deprecation of Peripheral::new()

As of v0.2.0, `Peripheral::new()` is `#[doc(hidden)]`.  
Use the following instead:

```rust
// ❌ Do not use
let monitor = Monitor::new(PeriphAddr(3));

// ✅ Recommended
let monitors: Vec<Monitor> = find_imm()?;
let singles = find_imm::<Monitor>()?;

let specific = wrap_imm::<Radar>(PeriphAddr(6))?;
```

### Borrowing and &mut self

`book_next_*` requires `&mut self`, so you cannot call multiple `book_next_*` methods on the same peripheral simultaneously.

```rust
// ❌ Error (second call re-borrows)
radar.book_next_scan_for_entities(100.0);
radar.book_next_scan_for_entities(200.0);  // Overwrites

// ✅ OK (each peripheral is independent)
radar.book_next_scan_for_entities(100.0);
motor.book_next_set_speed(50.0);
```

### Result Buffer Location

Currently, result buffers are allocated on the **Rust (WASM) side**.  
They may be moved to the Java side in the future for performance optimization.

---

## Related References

- [docs/spec.md](../spec.md) — Overall specification, 1-tick delay principle, FFI details
- [docs/api/](.) — API documentation for each peripheral
