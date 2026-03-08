# ColonyIntegrator

**Module:** `advanced_peripherals::colony_integrator`  
**Peripheral Type:** `advancedPeripherals:colony_integrator`

AdvancedPeripherals Colony Integrator peripheral. Provides comprehensive access to MineColonies colony data including colony info, citizens, buildings, work orders, requests, and attack status.

## Book-Read Methods

### Colony Information

#### `book_next_is_in_colony` / `read_last_is_in_colony`
Check if the peripheral is inside a colony.
```rust
pub fn book_next_is_in_colony(&mut self)
pub fn read_last_is_in_colony(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

#### `book_next_get_colony_id` / `read_last_get_colony_id`
Get the colony ID.
```rust
pub fn book_next_get_colony_id(&mut self)
pub fn read_last_get_colony_id(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_colony_name` / `read_last_get_colony_name`
Get the colony name.
```rust
pub fn book_next_get_colony_name(&mut self)
pub fn read_last_get_colony_name(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

#### `book_next_get_colony_style` / `read_last_get_colony_style`
Get the colony build style.
```rust
pub fn book_next_get_colony_style(&mut self)
pub fn read_last_get_colony_style(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

#### `book_next_is_active` / `read_last_is_active`
Check if the colony is currently active.
```rust
pub fn book_next_is_active(&mut self)
pub fn read_last_is_active(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

#### `book_next_get_amount_of_citizens` / `read_last_get_amount_of_citizens`
Get the current number of citizens.
```rust
pub fn book_next_get_amount_of_citizens(&mut self)
pub fn read_last_get_amount_of_citizens(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_max_citizens` / `read_last_get_max_citizens`
Get the maximum number of citizens.
```rust
pub fn book_next_get_max_citizens(&mut self)
pub fn read_last_get_max_citizens(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_happiness` / `read_last_get_happiness`
Get the colony happiness level.
```rust
pub fn book_next_get_happiness(&mut self)
pub fn read_last_get_happiness(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_position` / `read_last_get_position`
Get the town hall position.
```rust
pub fn book_next_get_position(&mut self)
pub fn read_last_get_position(&self) -> Result<ColonyPosition, PeripheralError>
```
**Returns:** `ColonyPosition`

---

### Citizens

#### `book_next_get_citizens` / `read_last_get_citizens`
Get the list of all citizens.
```rust
pub fn book_next_get_citizens(&mut self)
pub fn read_last_get_citizens(&self) -> Result<Vec<CitizenInfo>, PeripheralError>
```
**Returns:** `Vec<CitizenInfo>`

---

#### `book_next_get_citizen_info` / `read_last_get_citizen_info`
Get detailed info for a specific citizen by ID.
```rust
pub fn book_next_get_citizen_info(&mut self, id: i32)
pub fn read_last_get_citizen_info(&self) -> Result<CitizenInfo, PeripheralError>
```
**Parameters:**
- `id: i32` — Citizen ID

**Returns:** `CitizenInfo`

---

### Buildings

#### `book_next_get_buildings` / `read_last_get_buildings`
Get the list of all buildings.
```rust
pub fn book_next_get_buildings(&mut self)
pub fn read_last_get_buildings(&self) -> Result<Vec<BuildingInfo>, PeripheralError>
```
**Returns:** `Vec<BuildingInfo>`

---

#### `book_next_get_building_info` / `read_last_get_building_info`
Get building info at a specific position.
```rust
pub fn book_next_get_building_info(&mut self, x: f64, y: f64, z: f64)
pub fn read_last_get_building_info(&self) -> Result<BuildingInfo, PeripheralError>
```
**Parameters:**
- `x: f64`, `y: f64`, `z: f64` — Block coordinates

**Returns:** `BuildingInfo`

---

### Work Orders & Requests

#### `book_next_get_work_orders` / `read_last_get_work_orders`
Get the list of work orders.
```rust
pub fn book_next_get_work_orders(&mut self)
pub fn read_last_get_work_orders(&self) -> Result<Vec<WorkOrderInfo>, PeripheralError>
```
**Returns:** `Vec<WorkOrderInfo>`

---

#### `book_next_get_requests` / `read_last_get_requests`
Get the list of unresolved requests.
```rust
pub fn book_next_get_requests(&mut self)
pub fn read_last_get_requests(&self) -> Result<Value, PeripheralError>
```
**Returns:** `Value` (dynamic table)

---

#### `book_next_get_builder_resources` / `read_last_get_builder_resources`
Get the resource list needed by a builder at a given position.
```rust
pub fn book_next_get_builder_resources(&mut self, x: f64, y: f64, z: f64)
pub fn read_last_get_builder_resources(&self) -> Result<Value, PeripheralError>
```
**Parameters:**
- `x: f64`, `y: f64`, `z: f64` — Builder position

**Returns:** `Value` (dynamic table)

---

### Attack

#### `book_next_is_under_attack` / `read_last_is_under_attack`
Check if the colony is under attack.
```rust
pub fn book_next_is_under_attack(&mut self)
pub fn read_last_is_under_attack(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

## Immediate Methods

None.

## Types

```rust
pub struct ColonyPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct CitizenInfo {
    pub id: i32,
    pub name: String,
    pub job: Option<String>,
    pub level: Option<i32>,
    pub health: Option<f64>,
    pub max_health: Option<f64>,   // serde: "maxHealth"
    pub happiness: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub bed_pos: Option<ColonyPosition>, // serde: "bedPos"
}

pub struct BuildingInfo {
    pub building_type: Option<String>, // serde: "type"
    pub location: Option<ColonyPosition>,
    pub level: Option<i32>,
    pub max_level: Option<i32>,        // serde: "maxLevel"
    pub style: Option<String>,
}

pub struct WorkOrderInfo {
    pub id: i32,
    pub order_type: Option<String>,    // serde: "type"
    pub builder: Option<ColonyPosition>,
    pub location: Option<ColonyPosition>,
    pub priority: Option<i32>,
}
```

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::ColonyIntegrator;
use rust_computers_api::peripheral::Peripheral;

let mut colony = ColonyIntegrator::wrap(addr);

loop {
    let citizens = colony.read_last_get_citizens();
    let under_attack = colony.read_last_is_under_attack();

    colony.book_next_get_citizens();
    colony.book_next_is_under_attack();
    wait_for_next_tick().await;
}
```
