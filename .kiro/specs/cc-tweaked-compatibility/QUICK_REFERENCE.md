# Quick Reference: API Design Pattern

> **⚠️ Implementation Status Note**: This document describes the intended API design pattern. Currently, most peripherals have `book_next_*` and `read_last_*` methods implemented, but `async_*` variants are only partially implemented. See the [Implementation Status](#implementation-status) section below for details.

## Three-Function Pair Pattern

### Pattern Structure

```rust
pub struct SomePeripheral {
    addr: PeriphAddr,
}

impl SomePeripheral {
    // ===== Information Retrieval (Query) =====
    
    /// Book a request for the next tick
    pub fn book_next_get_data(&mut self) { }
    
    /// Read the result from the last tick
    pub fn read_last_get_data(&self) -> Result<Data, PeripheralError> { }
    
    /// Async version: book → wait → read
    pub async fn async_get_data(&self) -> Result<Data, PeripheralError> { }
    
    // ===== World Interaction (Action) =====
    
    /// Book an action for the next tick
    pub fn book_next_set_value(&mut self, value: i32) { }
    
    /// Read results from all booked actions
    pub fn read_last_set_value(&self) -> Vec<Result<(), PeripheralError>> { }
    
    /// Async version: book → wait → read
    pub async fn async_set_value(&mut self, value: i32) -> Result<(), PeripheralError> { }
    
    // ===== Event Handling =====
    
    /// Book an event listener for the next tick
    pub fn book_next_receive_event(&mut self) { }
    
    /// Read events (multiple requests, multiple responses)
    pub fn read_last_receive_event(&self) -> Vec<Option<Event>> { }
    
    /// Async version: wait until event occurs
    pub async fn async_receive_event(&self) -> Result<Event, PeripheralError> { }
}
```

## Usage Examples

### Example 1: Information Retrieval (book-read pattern)

```rust
let mut sensor = find_imm::<Sensor>().unwrap();

// Initial booking
sensor.book_next_get_data();
wait_for_next_tick().await;

loop {
    // Read result from last tick
    let data = sensor.read_last_get_data()?;
    
    // Process
    process(&data);
    
    // Book next request
    sensor.book_next_get_data();
    wait_for_next_tick().await;
}
```

### Example 2: Information Retrieval (async pattern)

```rust
let sensor = find_imm::<Sensor>().unwrap();

loop {
    // Async version handles book → wait → read internally
    let data = sensor.async_get_data().await?;
    
    // Process
    process(&data);
}
```

### Example 3: World Interaction (multiple actions)

```rust
let mut motor = find_imm::<Motor>().unwrap();

loop {
    // Book multiple actions
    motor.book_next_set_speed(100);
    motor.book_next_set_direction(Direction::Forward);
    motor.book_next_enable_brake(true);
    
    wait_for_next_tick().await;
    
    // Read results from all actions
    let results = motor.read_last_set_speed()?;  // Vec<Result<(), PeripheralError>>
    
    // Check if all succeeded
    for result in results {
        result?;  // Propagate errors
    }
}
```

### Example 4: Event Handling (async pattern - recommended)

```rust
let modem = find_imm::<Modem>().unwrap();

loop {
    // Wait for event (blocks until event occurs)
    let msg = modem.async_receive_raw().await?;
    
    // Process message
    process_message(&msg);
}
```

### Example 5: Event Handling (book-read pattern)

```rust
let mut modem = find_imm::<Modem>().unwrap();

modem.book_next_receive_raw();
wait_for_next_tick().await;

loop {
    // Check for events (may be empty)
    let events = modem.read_last_receive_raw()?;  // Vec<Option<Message>>
    
    for event in events.into_iter().flatten() {
        process_message(&event);
    }
    
    // Book next event
    modem.book_next_receive_raw();
    wait_for_next_tick().await;
}
```

### Example 6: Parallel Operations

```rust
let mut sensor = find_imm::<Sensor>().unwrap();
let mut motor = find_imm::<Motor>().unwrap();

loop {
    // Book multiple operations
    sensor.book_next_get_data();
    motor.book_next_get_status();
    
    wait_for_next_tick().await;
    
    // Read results in parallel
    let (data, status) = parallel!(
        sensor.async_get_data(),
        motor.async_get_status(),
    ).await;
    
    let data = data?;
    let status = status?;
    
    // Process
    process(&data, &status);
}
```

## Implementation Checklist

### For Each Peripheral Method

- [ ] Identify method type: Query / Action / Event
- [ ] Create `book_next_*` method
- [ ] Create `read_last_*` method
- [ ] Create `async_*` method
- [ ] Add proper error handling
- [ ] Add documentation with examples
- [ ] Add unit tests
- [ ] Add property-based tests

### For Query Methods

```rust
pub fn book_next_get_data(&mut self) {
    peripheral::book_request(self.addr, "getData", &[]);
}

pub fn read_last_get_data(&self) -> Result<Data, PeripheralError> {
    let bytes = peripheral::read_result(self.addr, "getData")?;
    decode::<Data>(&bytes)
}

pub async fn async_get_data(&self) -> Result<Data, PeripheralError> {
    self.book_next_get_data();
    wait_for_next_tick().await;
    self.read_last_get_data()
}
```

### For Action Methods

```rust
pub fn book_next_set_value(&mut self, value: i32) {
    let args = encode(&value)?;
    peripheral::book_action(self.addr, "setValue", &args);
}

pub fn read_last_set_value(&self) -> Vec<Result<(), PeripheralError>> {
    peripheral::read_action_results(self.addr, "setValue")
}

pub async fn async_set_value(&mut self, value: i32) -> Result<(), PeripheralError> {
    self.book_next_set_value(value);
    wait_for_next_tick().await;
    self.read_last_set_value()
        .into_iter()
        .next()
        .unwrap_or(Ok(()))
}
```

### For Event Methods

```rust
pub fn book_next_receive_event(&mut self) {
    peripheral::book_request(self.addr, "receiveEvent", &[]);
}

pub fn read_last_receive_event(&self) -> Vec<Option<Event>> {
    peripheral::read_event_results(self.addr, "receiveEvent")
}

pub async fn async_receive_event(&self) -> Result<Event, PeripheralError> {
    loop {
        self.book_next_receive_event();
        wait_for_next_tick().await;
        
        if let Some(event) = self.read_last_receive_event()?
            .into_iter()
            .next()
            .flatten()
        {
            return Ok(event);
        }
    }
}
```

## Common Patterns

### Pattern 1: Single Query Loop

```rust
loop {
    let value = peripheral.async_get_value().await?;
    process(value);
}
```

### Pattern 2: Multiple Actions

```rust
peripheral.book_next_action1(arg1);
peripheral.book_next_action2(arg2);
peripheral.book_next_action3(arg3);
wait_for_next_tick().await;

let results = peripheral.read_last_action1()?;
for result in results {
    result?;
}
```

### Pattern 3: Event Waiting

```rust
let event = peripheral.async_receive_event().await?;
process(event);
```

### Pattern 4: Parallel Queries

```rust
let (a, b, c) = parallel!(
    periph1.async_get_value(),
    periph2.async_get_value(),
    periph3.async_get_value(),
).await;
```

### Pattern 5: Mixed Operations

```rust
// Book multiple operations
sensor.book_next_get_data();
motor.book_next_set_speed(100);
wait_for_next_tick().await;

// Read results
let data = sensor.read_last_get_data()?;
let results = motor.read_last_set_speed()?;

// Process
process(&data);
for result in results {
    result?;
}
```

## Error Handling

### Common Errors

```rust
pub enum PeripheralError {
    NotFound,                    // Peripheral not found
    MethodNotFound,              // Method not found
    InvalidArgument,             // Invalid argument
    ExecutionFailed(String),     // Execution failed
    Disconnected,                // Disconnected
    Timeout,                     // Timeout
    SerializationError,          // Serialization error
}
```

### Error Recovery

```rust
// Retry on failure
loop {
    match peripheral.async_get_value().await {
        Ok(value) => {
            process(value);
            break;
        }
        Err(PeripheralError::Timeout) => {
            // Retry
            continue;
        }
        Err(e) => {
            // Fatal error
            eprintln!("Error: {:?}", e);
            break;
        }
    }
}
```

## Testing

### Unit Test Template

```rust
#[test]
fn test_book_next_get_data() {
    let mut periph = SomePeripheral::new(PeriphAddr::from_raw(0));
    periph.book_next_get_data();
    // Verify request was booked
}

#[test]
fn test_read_last_get_data() {
    let periph = SomePeripheral::new(PeriphAddr::from_raw(0));
    let result = periph.read_last_get_data();
    // Verify result format
}

#[tokio::test]
async fn test_async_get_data() {
    let periph = SomePeripheral::new(PeriphAddr::from_raw(0));
    let result = periph.async_get_data().await;
    // Verify async behavior
}
```

### Property Test Template

```rust
#[test]
fn prop_request_ordering() {
    // Property: Booked requests are executed in order
    // ∀ requests: book_next_*(requests[i]) → read_last_*() returns result[i]
}

#[test]
fn prop_action_accumulation() {
    // Property: All booked actions are executed
    // ∀ actions: book_next_action_*(actions[i]) → read_last_action_*() returns all results
}

#[test]
fn prop_event_termination() {
    // Property: Event waiting terminates when event occurs
    // ∀ event: async_receive_event().await → eventually returns event
}
```

## Documentation Template

```rust
/// Gets the current data from the sensor.
///
/// # Book-Read Pattern
///
/// ```rust,no_run
/// let mut sensor = find_imm::<Sensor>().unwrap();
/// sensor.book_next_get_data();
/// wait_for_next_tick().await;
/// let data = sensor.read_last_get_data()?;
/// ```
///
/// # Async Pattern
///
/// ```rust,no_run
/// let sensor = find_imm::<Sensor>().unwrap();
/// let data = sensor.async_get_data().await?;
/// ```
///
/// # Errors
///
/// Returns `PeripheralError` if:
/// - The sensor is not found
/// - The method is not available
/// - The sensor is disconnected
pub async fn async_get_data(&self) -> Result<Data, PeripheralError> {
    // Implementation
}
```

---

## Implementation Status

### ✅ Fully Implemented Peripherals

The following peripherals have all three function variants (`book_next_*`, `read_last_*`, and `async_*`) implemented:

**CC:Tweaked - Inventory**:
- `size()`, `list()`, `get_item_detail()`, `push_items()`, `pull_items()`
- ⚠️ Missing: `get_item_limit()`

### 🟡 Partially Implemented Peripherals

The following peripherals have `book_next_*` and `read_last_*` implemented, but are missing `async_*` variants:

**CC:Tweaked**:
- Modem (also missing: `is_wireless()`, `get_names_remote()`, event system)
- Monitor (also missing: `monitor_resize` event)
- Speaker (also missing: `speaker_audio_empty` event)

**AdvancedPeripherals** (all have `book_next_*` / `read_last_*` only):
- MEBridge (~60 methods)
- PlayerDetector (also missing: `playerJoin`/`playerLeave` events)
- ChatBox (also missing: `chat` event)
- BlockReader
- GeoScanner
- EnvironmentDetector
- EnergyDetector
- And others...

**Other Mods** (all have `book_next_*` / `read_last_*` only):
- Create peripherals
- Control-Craft peripherals
- Clockwork CC Compat peripherals
- Some-Peripherals
- Toms-Peripherals
- CBC CC Control

### 🚧 Event System Status

Event handling is currently **not fully implemented**. The following events are documented but not yet functional:
- `modem_message` (Modem)
- `monitor_resize` (Monitor)
- `monitor_touch` (Monitor) - partially implemented via `poll_touch()`
- `chat` (ChatBox)
- `playerJoin` / `playerLeave` (PlayerDetector)
- `speaker_audio_empty` (Speaker)

### Recommended Usage

Until `async_*` variants are fully implemented, use the `book_next_*` / `read_last_*` pattern:

```rust
// Current recommended pattern
let mut sensor = find_imm::<Sensor>().unwrap();

sensor.book_next_get_data();
wait_for_next_tick().await;

loop {
    let data = sensor.read_last_get_data()?;
    process(&data);
    
    sensor.book_next_get_data();
    wait_for_next_tick().await;
}
```

For peripherals with `async_*` implemented (like Inventory), you can use:

```rust
// For fully implemented peripherals
let inventory = find_imm::<Inventory>().unwrap();
let items = inventory.async_list().await?;
```

---

**Last Updated**: 2025-03-15
**Version**: 1.0
