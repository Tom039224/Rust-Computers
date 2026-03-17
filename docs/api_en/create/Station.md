```markdown
# Station

**Module:** `create`  
**Peripheral Type:** `create:station`

Create Station peripheral. Manages train station operations including assembly, scheduling, and train tracking.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_assemble` / `read_last_assemble`

Assemble a train at this station.

```rust
pub fn book_next_assemble(&mut self)
pub fn read_last_assemble(&self) -> Result<(), PeripheralError>
```

### `book_next_disassemble` / `read_last_disassemble`

Disassemble the train at this station.

```rust
pub fn book_next_disassemble(&mut self)
pub fn read_last_disassemble(&self) -> Result<(), PeripheralError>
```

### `book_next_set_assembly_mode` / `read_last_set_assembly_mode`

Set the assembly mode.

```rust
pub fn book_next_set_assembly_mode(&mut self, mode: bool)
pub fn read_last_set_assembly_mode(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `mode` | `bool` | `true` to enable assembly mode |

### `book_next_is_in_assembly_mode` / `read_last_is_in_assembly_mode`

Get whether the station is in assembly mode.

```rust
pub fn book_next_is_in_assembly_mode(&mut self)
pub fn read_last_is_in_assembly_mode(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if in assembly mode.

### `book_next_get_station_name` / `read_last_get_station_name`

Get the station name.

```rust
pub fn book_next_get_station_name(&mut self)
pub fn read_last_get_station_name(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the station name.

### `book_next_set_station_name` / `read_last_set_station_name`

Set the station name.

```rust
pub fn book_next_set_station_name(&mut self, name: &str)
pub fn read_last_set_station_name(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `&str` | The new station name |

### `book_next_is_train_present` / `read_last_is_train_present`

Get whether a train is present at the station.

```rust
pub fn book_next_is_train_present(&mut self)
pub fn read_last_is_train_present(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train is present.

### `book_next_is_train_imminent` / `read_last_is_train_imminent`

Get whether a train arrival is imminent.

```rust
pub fn book_next_is_train_imminent(&mut self)
pub fn read_last_is_train_imminent(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train arrival is imminent.

### `book_next_is_train_enroute` / `read_last_is_train_enroute`

Get whether a train is en route to this station.

```rust
pub fn book_next_is_train_enroute(&mut self)
pub fn read_last_is_train_enroute(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train is en route.

### `book_next_get_train_name` / `read_last_get_train_name`

Get the name of the train at this station.

```rust
pub fn book_next_get_train_name(&mut self)
pub fn read_last_get_train_name(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the train name.

### `book_next_set_train_name` / `read_last_set_train_name`

Set the name of the train at this station.

```rust
pub fn book_next_set_train_name(&mut self, name: &str)
pub fn read_last_set_train_name(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `&str` | The new train name |

### `book_next_has_schedule` / `read_last_has_schedule`

Get whether the train has a schedule.

```rust
pub fn book_next_has_schedule(&mut self)
pub fn read_last_has_schedule(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a schedule exists.

### `book_next_get_schedule` / `read_last_get_schedule`

Get the train's schedule.

```rust
pub fn book_next_get_schedule(&mut self)
pub fn read_last_get_schedule(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
```

**Returns:** `BTreeMap<String, Value>` — the schedule data.

### `book_next_set_schedule` / `read_last_set_schedule`

Set the train's schedule.

```rust
pub fn book_next_set_schedule(&mut self, schedule: &BTreeMap<String, Value>) -> Result<(), PeripheralError>
pub fn read_last_set_schedule(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `schedule` | `&BTreeMap<String, Value>` | The schedule data to set |

### `book_next_can_train_reach` / `read_last_can_train_reach`

Check if the train can reach a specified destination station.

```rust
pub fn book_next_can_train_reach(&mut self, dest: &str)
pub fn read_last_can_train_reach(&self) -> Result<(bool, Option<String>), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `dest` | `&str` | Destination station name |

**Returns:** `(bool, Option<String>)` — tuple of (reachable, reason).

### `book_next_distance_to` / `read_last_distance_to`

Get the distance to a specified destination station.

```rust
pub fn book_next_distance_to(&mut self, dest: &str)
pub fn read_last_distance_to(&self) -> Result<(Option<f64>, Option<String>), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `dest` | `&str` | Destination station name |

**Returns:** `(Option<f64>, Option<String>)` — tuple of (distance, reason). Distance is `None` if unreachable.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `is_in_assembly_mode_imm`

Get whether the station is in assembly mode immediately.

```rust
pub fn is_in_assembly_mode_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if in assembly mode.

### `get_station_name_imm`

Get the station name immediately.

```rust
pub fn get_station_name_imm(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the station name.

### `is_train_present_imm`

Get whether a train is present immediately.

```rust
pub fn is_train_present_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train is present.

### `is_train_imminent_imm`

Get whether a train arrival is imminent immediately.

```rust
pub fn is_train_imminent_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train arrival is imminent.

### `is_train_enroute_imm`

Get whether a train is en route immediately.

```rust
pub fn is_train_enroute_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train is en route.

### `get_train_name_imm`

Get the train name immediately.

```rust
pub fn get_train_name_imm(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the train name.

### `has_schedule_imm`

Get whether the train has a schedule immediately.

```rust
pub fn has_schedule_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a schedule exists.

### `get_schedule_imm`

Get the train's schedule immediately.

```rust
pub fn get_schedule_imm(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
```

**Returns:** `BTreeMap<String, Value>` — the schedule data.

### `can_train_reach_imm`

Check if the train can reach a destination immediately.

```rust
pub fn can_train_reach_imm(&self, dest: &str) -> Result<(bool, Option<String>), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `dest` | `&str` | Destination station name |

**Returns:** `(bool, Option<String>)` — tuple of (reachable, reason).

### `distance_to_imm`

Get the distance to a destination immediately.

```rust
pub fn distance_to_imm(&self, dest: &str) -> Result<(Option<f64>, Option<String>), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `dest` | `&str` | Destination station name |

**Returns:** `(Option<f64>, Option<String>)` — tuple of (distance, reason).

## Event-Wait Methods

### `book_next_try_pull_train_arrive` / `read_last_try_pull_train_arrive`

Wait 1 tick for a train arrival event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_train_arrive(&mut self)
pub fn read_last_try_pull_train_arrive(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_arrive`

Async method that waits until a train arrival event is received.

```rust
pub async fn pull_train_arrive(&self) -> Result<(), PeripheralError>
```

### `book_next_try_pull_train_depart` / `read_last_try_pull_train_depart`

Wait 1 tick for a train departure event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_train_depart(&mut self)
pub fn read_last_try_pull_train_depart(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_depart`

Async method that waits until a train departure event is received.

```rust
pub async fn pull_train_depart(&self) -> Result<(), PeripheralError>
```

## Usage Example

```rust
use rust_computers_api::create::station::Station;
use rust_computers_api::peripheral::Peripheral;

let mut station = Station::wrap(addr);

// Get station name
let name = station.get_station_name_imm()?;

// Check if train is present
let present = station.is_train_present_imm()?;

// Set station name
station.book_next_set_station_name("Depot A");
wait_for_next_tick().await;
station.read_last_set_station_name()?;

// Check reachability
station.book_next_can_train_reach("Depot B");
wait_for_next_tick().await;
let (reachable, reason) = station.read_last_can_train_reach()?;

// Wait for train arrival
station.pull_train_arrive().await?;
```

```
