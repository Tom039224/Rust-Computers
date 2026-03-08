# Station

**Module:** `create`  
**Peripheral Type:** `create:station`

Create Station peripheral. Controls train assembly, scheduling, and monitors train presence at a station.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_assemble` / `read_last_assemble`

Assemble the train.

```rust
pub fn book_next_assemble(&mut self)
pub fn read_last_assemble(&self) -> Result<(), PeripheralError>
```

#### `book_next_disassemble` / `read_last_disassemble`

Disassemble the train.

```rust
pub fn book_next_disassemble(&mut self)
pub fn read_last_disassemble(&self) -> Result<(), PeripheralError>
```

#### `book_next_set_assembly_mode` / `read_last_set_assembly_mode`

Set the assembly mode.

```rust
pub fn book_next_set_assembly_mode(&mut self, mode: bool)
pub fn read_last_set_assembly_mode(&self) -> Result<(), PeripheralError>
```

#### `book_next_is_in_assembly_mode` / `read_last_is_in_assembly_mode`

Check if in assembly mode.

```rust
pub fn book_next_is_in_assembly_mode(&mut self)
pub fn read_last_is_in_assembly_mode(&self) -> Result<bool, PeripheralError>
```

#### `book_next_get_station_name` / `read_last_get_station_name`

Get the station name.

```rust
pub fn book_next_get_station_name(&mut self)
pub fn read_last_get_station_name(&self) -> Result<String, PeripheralError>
```

#### `book_next_set_station_name` / `read_last_set_station_name`

Set the station name.

```rust
pub fn book_next_set_station_name(&mut self, name: &str)
pub fn read_last_set_station_name(&self) -> Result<(), PeripheralError>
```

#### `book_next_is_train_present` / `read_last_is_train_present`

Check if a train is present at the station.

```rust
pub fn book_next_is_train_present(&mut self)
pub fn read_last_is_train_present(&self) -> Result<bool, PeripheralError>
```

#### `book_next_is_train_imminent` / `read_last_is_train_imminent`

Check if a train is about to arrive.

```rust
pub fn book_next_is_train_imminent(&mut self)
pub fn read_last_is_train_imminent(&self) -> Result<bool, PeripheralError>
```

#### `book_next_is_train_enroute` / `read_last_is_train_enroute`

Check if a train is en route to the station.

```rust
pub fn book_next_is_train_enroute(&mut self)
pub fn read_last_is_train_enroute(&self) -> Result<bool, PeripheralError>
```

#### `book_next_get_train_name` / `read_last_get_train_name`

Get the train name.

```rust
pub fn book_next_get_train_name(&mut self)
pub fn read_last_get_train_name(&self) -> Result<String, PeripheralError>
```

#### `book_next_set_train_name` / `read_last_set_train_name`

Set the train name.

```rust
pub fn book_next_set_train_name(&mut self, name: &str)
pub fn read_last_set_train_name(&self) -> Result<(), PeripheralError>
```

#### `book_next_has_schedule` / `read_last_has_schedule`

Check if the train has a schedule.

```rust
pub fn book_next_has_schedule(&mut self)
pub fn read_last_has_schedule(&self) -> Result<bool, PeripheralError>
```

#### `book_next_get_schedule` / `read_last_get_schedule`

Get the train schedule.

```rust
pub fn book_next_get_schedule(&mut self)
pub fn read_last_get_schedule(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
```

#### `book_next_set_schedule` / `read_last_set_schedule`

Set the train schedule.

```rust
pub fn book_next_set_schedule(&mut self, schedule: &BTreeMap<String, Value>) -> Result<(), PeripheralError>
pub fn read_last_set_schedule(&self) -> Result<(), PeripheralError>
```

#### `book_next_can_train_reach` / `read_last_can_train_reach`

Check if the train can reach a destination. Returns `(reachable, reason)`.

```rust
pub fn book_next_can_train_reach(&mut self, dest: &str)
pub fn read_last_can_train_reach(&self) -> Result<(bool, Option<String>), PeripheralError>
```

#### `book_next_distance_to` / `read_last_distance_to`

Get the distance to a destination. Returns `(distance, reason)`.

```rust
pub fn book_next_distance_to(&mut self, dest: &str)
pub fn read_last_distance_to(&self) -> Result<(Option<f64>, Option<String>), PeripheralError>
```

### Immediate Methods

#### `is_in_assembly_mode_imm`

```rust
pub fn is_in_assembly_mode_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_station_name_imm`

```rust
pub fn get_station_name_imm(&self) -> Result<String, PeripheralError>
```

#### `is_train_present_imm`

```rust
pub fn is_train_present_imm(&self) -> Result<bool, PeripheralError>
```

#### `is_train_imminent_imm`

```rust
pub fn is_train_imminent_imm(&self) -> Result<bool, PeripheralError>
```

#### `is_train_enroute_imm`

```rust
pub fn is_train_enroute_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_train_name_imm`

```rust
pub fn get_train_name_imm(&self) -> Result<String, PeripheralError>
```

#### `has_schedule_imm`

```rust
pub fn has_schedule_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_schedule_imm`

```rust
pub fn get_schedule_imm(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
```

#### `can_train_reach_imm`

```rust
pub fn can_train_reach_imm(&self, dest: &str) -> Result<(bool, Option<String>), PeripheralError>
```

#### `distance_to_imm`

```rust
pub fn distance_to_imm(&self, dest: &str) -> Result<(Option<f64>, Option<String>), PeripheralError>
```

### Async Methods (Event)

#### `book_next_try_pull_train_arrive` / `read_last_try_pull_train_arrive`

Try to pull a train arrival event (non-blocking).

```rust
pub fn book_next_try_pull_train_arrive(&mut self)
pub fn read_last_try_pull_train_arrive(&self) -> Result<Option<()>, PeripheralError>
```

#### `pull_train_arrive`

Wait for a train arrival event (blocking async loop).

```rust
pub async fn pull_train_arrive(&self) -> Result<(), PeripheralError>
```

#### `book_next_try_pull_train_depart` / `read_last_try_pull_train_depart`

Try to pull a train departure event (non-blocking).

```rust
pub fn book_next_try_pull_train_depart(&mut self)
pub fn read_last_try_pull_train_depart(&self) -> Result<Option<()>, PeripheralError>
```

#### `pull_train_depart`

Wait for a train departure event (blocking async loop).

```rust
pub async fn pull_train_depart(&self) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::create::station::Station;
use rust_computers_api::peripheral::Peripheral;

let mut station = Station::wrap(addr);

// Wait for train to arrive
station.pull_train_arrive().await?;

// Read train info
let name = station.get_train_name_imm()?;
let has_sched = station.has_schedule_imm()?;

// Dispatch train
station.book_next_set_train_name("Express Line");
wait_for_next_tick().await;
station.read_last_set_train_name()?;
```
