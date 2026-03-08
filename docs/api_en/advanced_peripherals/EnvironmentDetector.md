# EnvironmentDetector

**Module:** `advanced_peripherals::environment_detector`  
**Peripheral Type:** `advancedPeripherals:environment_detector`

AdvancedPeripherals Environment Detector peripheral. Provides comprehensive access to environmental data including biome, dimension, weather, light levels, time, moon phase, terrain, sleep status, and entity scanning.

## Book-Read Methods

### Environment & Weather

#### `book_next_get_biome` / `read_last_get_biome`
Get the biome ID at the peripheral's location.
```rust
pub fn book_next_get_biome(&mut self)
pub fn read_last_get_biome(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

#### `book_next_get_dimension` / `read_last_get_dimension`
Get the dimension ID.
```rust
pub fn book_next_get_dimension(&mut self)
pub fn read_last_get_dimension(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

#### `book_next_is_dimension` / `read_last_is_dimension`
Check if the peripheral is in a specific dimension.
```rust
pub fn book_next_is_dimension(&mut self, dim: &str)
pub fn read_last_is_dimension(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `dim: &str` — Dimension ID to check

**Returns:** `bool`

---

#### `book_next_list_dimensions` / `read_last_list_dimensions`
List all dimension IDs.
```rust
pub fn book_next_list_dimensions(&mut self)
pub fn read_last_list_dimensions(&self) -> Result<Vec<String>, PeripheralError>
```
**Returns:** `Vec<String>`

---

#### `book_next_is_raining` / `read_last_is_raining`
Check if it is raining.
```rust
pub fn book_next_is_raining(&mut self)
pub fn read_last_is_raining(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

#### `book_next_is_thunder` / `read_last_is_thunder`
Check if there is a thunderstorm.
```rust
pub fn book_next_is_thunder(&mut self)
pub fn read_last_is_thunder(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

#### `book_next_is_sunny` / `read_last_is_sunny`
Check if it is sunny.
```rust
pub fn book_next_is_sunny(&mut self)
pub fn read_last_is_sunny(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

### Light & Time

#### `book_next_get_sky_light_level` / `read_last_get_sky_light_level`
Get the sky light level (0–15).
```rust
pub fn book_next_get_sky_light_level(&mut self)
pub fn read_last_get_sky_light_level(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_block_light_level` / `read_last_get_block_light_level`
Get the block light level (0–15).
```rust
pub fn book_next_get_block_light_level(&mut self)
pub fn read_last_get_block_light_level(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_day_light_level` / `read_last_get_day_light_level`
Get the daylight level (0–15).
```rust
pub fn book_next_get_day_light_level(&mut self)
pub fn read_last_get_day_light_level(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_time` / `read_last_get_time`
Get the world time in ticks.
```rust
pub fn book_next_get_time(&mut self)
pub fn read_last_get_time(&self) -> Result<i64, PeripheralError>
```
**Returns:** `i64`

---

### Moon Phase

#### `book_next_get_moon_id` / `read_last_get_moon_id`
Get the current moon phase ID (0–7).
```rust
pub fn book_next_get_moon_id(&mut self)
pub fn read_last_get_moon_id(&self) -> Result<i32, PeripheralError>
```
**Returns:** `i32`

---

#### `book_next_get_moon_name` / `read_last_get_moon_name`
Get the moon phase name.
```rust
pub fn book_next_get_moon_name(&mut self)
pub fn read_last_get_moon_name(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

#### `book_next_is_moon` / `read_last_is_moon`
Check if the current moon phase matches the given name.
```rust
pub fn book_next_is_moon(&mut self, phase: &str)
pub fn read_last_is_moon(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `phase: &str` — Moon phase name to check

**Returns:** `bool`

---

### Terrain

#### `book_next_is_slime_chunk` / `read_last_is_slime_chunk`
Check if the current chunk is a slime chunk.
```rust
pub fn book_next_is_slime_chunk(&mut self)
pub fn read_last_is_slime_chunk(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

### Sleep

#### `book_next_can_sleep_here` / `read_last_can_sleep_here`
Check if sleeping is possible in this dimension.
```rust
pub fn book_next_can_sleep_here(&mut self)
pub fn read_last_can_sleep_here(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

#### `book_next_can_sleep_player` / `read_last_can_sleep_player`
Check if a specific player can sleep.
```rust
pub fn book_next_can_sleep_player(&mut self, name: &str)
pub fn read_last_can_sleep_player(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `name: &str` — Player name

**Returns:** `bool`

---

### Entity Scanning

#### `book_next_scan_entities` / `read_last_scan_entities`
Scan for entities within a given radius.
```rust
pub fn book_next_scan_entities(&mut self, radius: f64)
pub fn read_last_scan_entities(&self) -> Result<Vec<EntityInfo>, PeripheralError>
```
**Parameters:**
- `radius: f64` — Scan radius

**Returns:** `Vec<EntityInfo>`

---

#### `book_next_scan_cost` / `read_last_scan_cost` / `scan_cost_imm`
Get the fuel cost for a scan operation.
```rust
pub fn book_next_scan_cost(&mut self, radius: f64)
pub fn read_last_scan_cost(&self) -> Result<f64, PeripheralError>
pub fn scan_cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```
**Parameters:**
- `radius: f64` — Scan radius

**Returns:** `f64`

## Immediate Methods

- `scan_cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>`

## Types

```rust
pub struct EntityInfo {
    pub id: String,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub tags: Vec<String>,
    pub can_freeze: Option<bool>,       // serde: "canFreeze"
    pub is_glowing: Option<bool>,       // serde: "isGlowing"
    pub is_in_wall: Option<bool>,       // serde: "isInWall"
    pub health: Option<f64>,
    pub max_health: Option<f64>,        // serde: "maxHealth"
    pub last_damage_source: Option<String>, // serde: "lastDamageSource"
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}
```

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::EnvironmentDetector;
use rust_computers_api::peripheral::Peripheral;

let mut env = EnvironmentDetector::wrap(addr);

loop {
    let biome = env.read_last_get_biome();
    let raining = env.read_last_is_raining();
    let entities = env.read_last_scan_entities();

    env.book_next_get_biome();
    env.book_next_is_raining();
    env.book_next_scan_entities(16.0);
    wait_for_next_tick().await;
}
```
