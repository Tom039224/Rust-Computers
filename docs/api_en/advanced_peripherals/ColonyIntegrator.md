# ColonyIntegrator

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:colony_integrator`  
**Source:** `ColonyIntegratorPeripheral.java`

## Overview

The ColonyIntegrator peripheral provides comprehensive access to MineColonies colony data. It allows you to query colony information, manage citizens, monitor buildings, track work orders and requests, and check attack status. This is useful for creating automated colony management systems and monitoring tools.

## Three-Function Pattern

The ColonyIntegrator API uses the three-function pattern for all methods:

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

### Colony Information

#### `isInColony()` / `book_next_is_in_colony()` / `read_last_is_in_colony()` / `async_is_in_colony()`

Check if the peripheral is inside a colony.

**Rust Signatures:**
```rust
pub fn book_next_is_in_colony(&mut self)
pub fn read_last_is_in_colony(&self) -> Result<bool, PeripheralError>
pub async fn async_is_in_colony(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if inside a colony

---

#### `getColonyId()` / `book_next_get_colony_id()` / `read_last_get_colony_id()` / `async_get_colony_id()`

Get the colony ID.

**Rust Signatures:**
```rust
pub fn book_next_get_colony_id(&mut self)
pub fn read_last_get_colony_id(&self) -> Result<i32, PeripheralError>
pub async fn async_get_colony_id(&self) -> Result<i32, PeripheralError>
```

**Returns:** `number` — Colony ID

---

#### `getColonyName()` / `book_next_get_colony_name()` / `read_last_get_colony_name()` / `async_get_colony_name()`

Get the colony name.

**Rust Signatures:**
```rust
pub fn book_next_get_colony_name(&mut self)
pub fn read_last_get_colony_name(&self) -> Result<String, PeripheralError>
pub async fn async_get_colony_name(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — Colony name

---

#### `getColonyStyle()` / `book_next_get_colony_style()` / `read_last_get_colony_style()` / `async_get_colony_style()`

Get the colony build style.

**Rust Signatures:**
```rust
pub fn book_next_get_colony_style(&mut self)
pub fn read_last_get_colony_style(&self) -> Result<String, PeripheralError>
pub async fn async_get_colony_style(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — Build style name

---

#### `isActive()` / `book_next_is_active()` / `read_last_is_active()` / `async_is_active()`

Check if the colony is currently active.

**Rust Signatures:**
```rust
pub fn book_next_is_active(&mut self)
pub fn read_last_is_active(&self) -> Result<bool, PeripheralError>
pub async fn async_is_active(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if colony is active

---

#### `getAmountOfCitizens()` / `book_next_get_amount_of_citizens()` / `read_last_get_amount_of_citizens()` / `async_get_amount_of_citizens()`

Get the current number of citizens.

**Rust Signatures:**
```rust
pub fn book_next_get_amount_of_citizens(&mut self)
pub fn read_last_get_amount_of_citizens(&self) -> Result<i32, PeripheralError>
pub async fn async_get_amount_of_citizens(&self) -> Result<i32, PeripheralError>
```

**Returns:** `number` — Current citizen count

---

#### `getMaxCitizens()` / `book_next_get_max_citizens()` / `read_last_get_max_citizens()` / `async_get_max_citizens()`

Get the maximum number of citizens.

**Rust Signatures:**
```rust
pub fn book_next_get_max_citizens(&mut self)
pub fn read_last_get_max_citizens(&self) -> Result<i32, PeripheralError>
pub async fn async_get_max_citizens(&self) -> Result<i32, PeripheralError>
```

**Returns:** `number` — Maximum citizen capacity

---

#### `getHappiness()` / `book_next_get_happiness()` / `read_last_get_happiness()` / `async_get_happiness()`

Get the colony happiness level.

**Rust Signatures:**
```rust
pub fn book_next_get_happiness(&mut self)
pub fn read_last_get_happiness(&self) -> Result<f64, PeripheralError>
pub async fn async_get_happiness(&self) -> Result<f64, PeripheralError>
```

**Returns:** `number` — Happiness level

---

#### `getPosition()` / `book_next_get_position()` / `read_last_get_position()` / `async_get_position()`

Get the town hall position.

**Rust Signatures:**
```rust
pub fn book_next_get_position(&mut self)
pub fn read_last_get_position(&self) -> Result<ColonyPosition, PeripheralError>
pub async fn async_get_position(&self) -> Result<ColonyPosition, PeripheralError>
```

**Returns:** `table` — Position table with x, y, z coordinates

---

### Citizens

#### `getCitizens()` / `book_next_get_citizens()` / `read_last_get_citizens()` / `async_get_citizens()`

Get the list of all citizens.

**Rust Signatures:**
```rust
pub fn book_next_get_citizens(&mut self)
pub fn read_last_get_citizens(&self) -> Result<Vec<CitizenInfo>, PeripheralError>
pub async fn async_get_citizens(&self) -> Result<Vec<CitizenInfo>, PeripheralError>
```

**Returns:** `table` — Array of citizen information

---

#### `getCitizenInfo(id)` / `book_next_get_citizen_info(id)` / `read_last_get_citizen_info()` / `async_get_citizen_info(id)`

Get detailed info for a specific citizen by ID.

**Rust Signatures:**
```rust
pub fn book_next_get_citizen_info(&mut self, id: i32)
pub fn read_last_get_citizen_info(&self) -> Result<CitizenInfo, PeripheralError>
pub async fn async_get_citizen_info(&self, id: i32) -> Result<CitizenInfo, PeripheralError>
```

**Parameters:**
- `id: number` — Citizen ID

**Returns:** `table` — Citizen information

---

### Buildings

#### `getBuildings()` / `book_next_get_buildings()` / `read_last_get_buildings()` / `async_get_buildings()`

Get the list of all buildings.

**Rust Signatures:**
```rust
pub fn book_next_get_buildings(&mut self)
pub fn read_last_get_buildings(&self) -> Result<Vec<BuildingInfo>, PeripheralError>
pub async fn async_get_buildings(&self) -> Result<Vec<BuildingInfo>, PeripheralError>
```

**Returns:** `table` — Array of building information

---

#### `getBuildingInfo(x, y, z)` / `book_next_get_building_info(x, y, z)` / `read_last_get_building_info()` / `async_get_building_info(x, y, z)`

Get building info at a specific position.

**Rust Signatures:**
```rust
pub fn book_next_get_building_info(&mut self, x: f64, y: f64, z: f64)
pub fn read_last_get_building_info(&self) -> Result<BuildingInfo, PeripheralError>
pub async fn async_get_building_info(&self, x: f64, y: f64, z: f64) -> Result<BuildingInfo, PeripheralError>
```

**Parameters:**
- `x, y, z: number` — Block coordinates

**Returns:** `table` — Building information

---

### Work Orders & Requests

#### `getWorkOrders()` / `book_next_get_work_orders()` / `read_last_get_work_orders()` / `async_get_work_orders()`

Get the list of work orders.

**Rust Signatures:**
```rust
pub fn book_next_get_work_orders(&mut self)
pub fn read_last_get_work_orders(&self) -> Result<Vec<WorkOrderInfo>, PeripheralError>
pub async fn async_get_work_orders(&self) -> Result<Vec<WorkOrderInfo>, PeripheralError>
```

**Returns:** `table` — Array of work order information

---

#### `getRequests()` / `book_next_get_requests()` / `read_last_get_requests()` / `async_get_requests()`

Get the list of unresolved requests.

**Rust Signatures:**
```rust
pub fn book_next_get_requests(&mut self)
pub fn read_last_get_requests(&self) -> Result<Value, PeripheralError>
pub async fn async_get_requests(&self) -> Result<Value, PeripheralError>
```

**Returns:** `table` — Dynamic table of requests

---

#### `getBuilderResources(x, y, z)` / `book_next_get_builder_resources(x, y, z)` / `read_last_get_builder_resources()` / `async_get_builder_resources(x, y, z)`

Get the resource list needed by a builder at a given position.

**Rust Signatures:**
```rust
pub fn book_next_get_builder_resources(&mut self, x: f64, y: f64, z: f64)
pub fn read_last_get_builder_resources(&self) -> Result<Value, PeripheralError>
pub async fn async_get_builder_resources(&self, x: f64, y: f64, z: f64) -> Result<Value, PeripheralError>
```

**Parameters:**
- `x, y, z: number` — Builder position

**Returns:** `table` — Dynamic table of required resources

---

### Attack Status

#### `isUnderAttack()` / `book_next_is_under_attack()` / `read_last_is_under_attack()` / `async_is_under_attack()`

Check if the colony is under attack.

**Rust Signatures:**
```rust
pub fn book_next_is_under_attack(&mut self)
pub fn read_last_is_under_attack(&self) -> Result<bool, PeripheralError>
pub async fn async_is_under_attack(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if colony is under attack

---

## Events

The ColonyIntegrator peripheral does not generate events.

---

## Usage Examples

### Example 1: Monitor Colony Status

```rust
// Rust example to be added
```
### Example 2: List All Citizens

```rust
// Rust example to be added
```
### Example 3: Monitor Work Orders

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Not in colony**: Peripheral is not inside a colony
- **Peripheral disconnected**: The ColonyIntegrator is no longer accessible
- **Invalid citizen ID**: Citizen ID does not exist
- **Invalid coordinates**: Coordinates are invalid or out of range

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### ColonyPosition
```rust
// Rust example to be added
```
### CitizenInfo
```rust
// Rust example to be added
```
### BuildingInfo
```rust
// Rust example to be added
```
### WorkOrderInfo
```rust
// Rust example to be added
```
---

## Notes

- The ColonyIntegrator requires MineColonies mod to be installed
- Colony data is updated periodically, not in real-time
- The three-function pattern allows for efficient batch operations
- Happiness affects citizen productivity and breeding
- Work orders are assigned to builders automatically

---

## Related

- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [BlockReader](./BlockReader.md) — For reading block information
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
