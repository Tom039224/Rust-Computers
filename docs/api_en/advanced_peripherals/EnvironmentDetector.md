# EnvironmentDetector

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:environment_detector`  
**Source:** `EnvironmentDetectorPeripheral.java`

## Overview

The EnvironmentDetector peripheral provides comprehensive access to environmental data including biome information, dimension details, weather conditions, light levels, time, moon phases, terrain properties, sleep status, and entity scanning. This is useful for creating weather-aware systems, time-based automation, entity detection systems, and environmental monitoring.

## Three-Function Pattern

The EnvironmentDetector API uses the three-function pattern for all methods:

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

### Environment & Weather

### Environment & Weather

#### `getBiome()` / `book_next_get_biome()` / `read_last_get_biome()` / `async_get_biome()`

Get the biome ID at the peripheral's location.

**Rust Signatures:**
```rust
pub fn book_next_get_biome(&mut self)
pub fn read_last_get_biome(&self) -> Result<String, PeripheralError>
pub async fn async_get_biome(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — Biome registry name (e.g., "minecraft:plains", "minecraft:forest")

**Example:**
```rust
// Rust example to be added
```
---

#### `getDimension()` / `book_next_get_dimension()` / `read_last_get_dimension()` / `async_get_dimension()`

Get the dimension ID.

**Rust Signatures:**
```rust
pub fn book_next_get_dimension(&mut self)
pub fn read_last_get_dimension(&self) -> Result<String, PeripheralError>
pub async fn async_get_dimension(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — Dimension ID (e.g., "minecraft:overworld", "minecraft:the_nether")

---

#### `isDimension(dim)` / `book_next_is_dimension(dim)` / `read_last_is_dimension()` / `async_is_dimension(dim)`

Check if the peripheral is in a specific dimension.

**Rust Signatures:**
```rust
pub fn book_next_is_dimension(&mut self, dim: &str)
pub fn read_last_is_dimension(&self) -> Result<bool, PeripheralError>
pub async fn async_is_dimension(&self, dim: &str) -> Result<bool, PeripheralError>
```

**Parameters:**
- `dim: string` — Dimension ID to check

**Returns:** `boolean` — `true` if in the specified dimension

---

#### `listDimensions()` / `book_next_list_dimensions()` / `read_last_list_dimensions()` / `async_list_dimensions()`

List all dimension IDs available on the server.

**Rust Signatures:**
```rust
pub fn book_next_list_dimensions(&mut self)
pub fn read_last_list_dimensions(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_list_dimensions(&self) -> Result<Vec<String>, PeripheralError>
```

**Returns:** `table` — Array of dimension IDs

---

#### `isRaining()` / `book_next_is_raining()` / `read_last_is_raining()` / `async_is_raining()`

Check if it is currently raining.

**Rust Signatures:**
```rust
pub fn book_next_is_raining(&mut self)
pub fn read_last_is_raining(&self) -> Result<bool, PeripheralError>
pub async fn async_is_raining(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if raining

---

#### `isThunder()` / `book_next_is_thunder()` / `read_last_is_thunder()` / `async_is_thunder()`

Check if there is a thunderstorm.

**Rust Signatures:**
```rust
pub fn book_next_is_thunder(&mut self)
pub fn read_last_is_thunder(&self) -> Result<bool, PeripheralError>
pub async fn async_is_thunder(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if thundering

---

#### `isSunny()` / `book_next_is_sunny()` / `read_last_is_sunny()` / `async_is_sunny()`

Check if it is sunny.

**Rust Signatures:**
```rust
pub fn book_next_is_sunny(&mut self)
pub fn read_last_is_sunny(&self) -> Result<bool, PeripheralError>
pub async fn async_is_sunny(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if sunny

---

### Light & Time

#### `getSkyLightLevel()` / `book_next_get_sky_light_level()` / `read_last_get_sky_light_level()` / `async_get_sky_light_level()`

Get the sky light level (0–15).

**Rust Signatures:**
```rust
pub fn book_next_get_sky_light_level(&mut self)
pub fn read_last_get_sky_light_level(&self) -> Result<i32, PeripheralError>
pub async fn async_get_sky_light_level(&self) -> Result<i32, PeripheralError>
```

**Returns:** `number` — Sky light level (0-15)

---

#### `getBlockLightLevel()` / `book_next_get_block_light_level()` / `read_last_get_block_light_level()` / `async_get_block_light_level()`

Get the block light level (0–15).

**Rust Signatures:**
```rust
pub fn book_next_get_block_light_level(&mut self)
pub fn read_last_get_block_light_level(&self) -> Result<i32, PeripheralError>
pub async fn async_get_block_light_level(&self) -> Result<i32, PeripheralError>
```

**Returns:** `number` — Block light level (0-15)

---

#### `getDayLightLevel()` / `book_next_get_day_light_level()` / `read_last_get_day_light_level()` / `async_get_day_light_level()`

Get the daylight level (0–15).

**Rust Signatures:**
```rust
pub fn book_next_get_day_light_level(&mut self)
pub fn read_last_get_day_light_level(&self) -> Result<i32, PeripheralError>
pub async fn async_get_day_light_level(&self) -> Result<i32, PeripheralError>
```

**Returns:** `number` — Daylight level (0-15)

---

#### `getTime()` / `book_next_get_time()` / `read_last_get_time()` / `async_get_time()`

Get the world time in ticks.

**Rust Signatures:**
```rust
pub fn book_next_get_time(&mut self)
pub fn read_last_get_time(&self) -> Result<i64, PeripheralError>
pub async fn async_get_time(&self) -> Result<i64, PeripheralError>
```

**Returns:** `number` — World time in ticks (0-24000 per day)

**Example:**
```rust
// Rust example to be added
```
---

### Moon Phase

#### `getMoonId()` / `book_next_get_moon_id()` / `read_last_get_moon_id()` / `async_get_moon_id()`

Get the current moon phase ID (0–7).

**Rust Signatures:**
```rust
pub fn book_next_get_moon_id(&mut self)
pub fn read_last_get_moon_id(&self) -> Result<i32, PeripheralError>
pub async fn async_get_moon_id(&self) -> Result<i32, PeripheralError>
```

**Returns:** `number` — Moon phase ID (0-7)

---

#### `getMoonName()` / `book_next_get_moon_name()` / `read_last_get_moon_name()` / `async_get_moon_name()`

Get the moon phase name.

**Rust Signatures:**
```rust
pub fn book_next_get_moon_name(&mut self)
pub fn read_last_get_moon_name(&self) -> Result<String, PeripheralError>
pub async fn async_get_moon_name(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — Moon phase name (e.g., "Full Moon", "New Moon")

---

#### `isMoon(phase)` / `book_next_is_moon(phase)` / `read_last_is_moon()` / `async_is_moon(phase)`

Check if the current moon phase matches the given name.

**Rust Signatures:**
```rust
pub fn book_next_is_moon(&mut self, phase: &str)
pub fn read_last_is_moon(&self) -> Result<bool, PeripheralError>
pub async fn async_is_moon(&self, phase: &str) -> Result<bool, PeripheralError>
```

**Parameters:**
- `phase: string` — Moon phase name to check

**Returns:** `boolean` — `true` if current phase matches

---

### Terrain

#### `isSlimeChunk()` / `book_next_is_slime_chunk()` / `read_last_is_slime_chunk()` / `async_is_slime_chunk()`

Check if the current chunk is a slime chunk.

**Rust Signatures:**
```rust
pub fn book_next_is_slime_chunk(&mut self)
pub fn read_last_is_slime_chunk(&self) -> Result<bool, PeripheralError>
pub async fn async_is_slime_chunk(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if current chunk is a slime chunk

---

### Sleep

#### `canSleepHere()` / `book_next_can_sleep_here()` / `read_last_can_sleep_here()` / `async_can_sleep_here()`

Check if sleeping is possible in this dimension.

**Rust Signatures:**
```rust
pub fn book_next_can_sleep_here(&mut self)
pub fn read_last_can_sleep_here(&self) -> Result<bool, PeripheralError>
pub async fn async_can_sleep_here(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if sleeping is possible

---

#### `canSleepPlayer(name)` / `book_next_can_sleep_player(name)` / `read_last_can_sleep_player()` / `async_can_sleep_player(name)`

Check if a specific player can sleep.

**Rust Signatures:**
```rust
pub fn book_next_can_sleep_player(&mut self, name: &str)
pub fn read_last_can_sleep_player(&self) -> Result<bool, PeripheralError>
pub async fn async_can_sleep_player(&self, name: &str) -> Result<bool, PeripheralError>
```

**Parameters:**
- `name: string` — Player name

**Returns:** `boolean` — `true` if player can sleep

---

### Entity Scanning

#### `scanEntities(radius)` / `book_next_scan_entities(radius)` / `read_last_scan_entities()` / `async_scan_entities(radius)`

Scan for entities within a given radius.

**Rust Signatures:**
```rust
pub fn book_next_scan_entities(&mut self, radius: f64)
pub fn read_last_scan_entities(&self) -> Result<Vec<EntityInfo>, PeripheralError>
pub async fn async_scan_entities(&self, radius: f64) -> Result<Vec<EntityInfo>, PeripheralError>
```

**Parameters:**
- `radius: number` — Scan radius in blocks

**Returns:** `table` — Array of entity information

**Entity Structure:**
```rust
// Rust example to be added
```
**Example:**
```rust
// Rust example to be added
```
---

#### `scanCost(radius)` / `book_next_scan_cost(radius)` / `read_last_scan_cost()` / `scan_cost_imm(radius)`

Get the fuel cost for a scan operation.

**Rust Signatures:**
```rust
pub fn book_next_scan_cost(&mut self, radius: f64)
pub fn read_last_scan_cost(&self) -> Result<f64, PeripheralError>
pub fn scan_cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```

**Parameters:**
- `radius: number` — Scan radius in blocks

**Returns:** `number` — Fuel cost

**Note:** The `scan_cost_imm` method returns the result immediately without waiting for the next tick.

---

## Events

The EnvironmentDetector peripheral does not generate events.

---

## Usage Examples

### Example 1: Weather Monitoring

```rust
// Rust example to be added
```
### Example 2: Time-Based Automation

```rust
// Rust example to be added
```
### Example 3: Entity Detection

```rust
// Rust example to be added
```
### Example 4: Biome Detection

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Peripheral disconnected**: The EnvironmentDetector is no longer accessible
- **Invalid radius**: Scan radius is negative or exceeds maximum
- **Invalid dimension**: Dimension ID does not exist
- **Invalid player**: Player name is not found

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### EntityInfo
```rust
// Rust example to be added
```
---

## Notes

- Light levels range from 0 (dark) to 15 (bright)
- World time is measured in ticks (0-24000 per day)
- Moon phases cycle from 0 (full moon) to 7 (new moon)
- Entity scanning has a fuel cost that increases with radius
- The three-function pattern allows for efficient batch operations
- Slime chunks are determined by world seed and chunk coordinates

---

## Related

- [BlockReader](./BlockReader.md) — For reading block information
- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
