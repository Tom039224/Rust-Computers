# BlockReader

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:block_reader`  
**Source:** `BlockReaderPeripheral.java`

## Overview

The BlockReader peripheral reads detailed information about the block directly in front of it. This includes the block's registry name, NBT data, block state properties, and whether it's a tile entity (block entity). This is useful for automation systems that need to identify and react to specific blocks or their properties.

## Three-Function Pattern

The BlockReader API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
use rust_computers_api::advanced_peripherals::BlockReader;
use rust_computers_api::peripheral::find_imm;

// Example using the three-function pattern
async fn example_block_reader() -> Result<(), rust_computers_api::error::PeripheralError> {
    // Find the BlockReader peripheral
    let mut block_reader = find_imm::<BlockReader>()?;
    
    // Method 1: Using book_next + read_last pattern (manual control)
    block_reader.book_next_get_block_name();
    rust_computers_api::wait_for_next_tick().await;
    let block_name = block_reader.read_last_get_block_name()?;
    println!("Block name: {}", block_name);
    
    // Method 2: Using async_* convenience method
    let block_data = block_reader.async_get_block_data().await?;
    println!("Block data: {:?}", block_data);
    
    // Method 3: Batch multiple operations
    block_reader.book_next_get_block_states();
    block_reader.book_next_is_tile_entity();
    rust_computers_api::wait_for_next_tick().await;
    
    let block_states = block_reader.read_last_get_block_states()?;
    let is_tile_entity = block_reader.read_last_is_tile_entity()?;
    
    println!("Block states: {:?}", block_states);
    println!("Is tile entity: {}", is_tile_entity);
    
    Ok(())
}
```
## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### Block Information

#### `getBlockName()` / `book_next_get_block_name()` / `read_last_get_block_name()` / `async_get_block_name()`

Get the registry name of the block in front of the peripheral.

**Rust Signatures:**
```rust
pub fn book_next_get_block_name(&mut self)
pub fn read_last_get_block_name(&self) -> Result<String, PeripheralError>
pub async fn async_get_block_name(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — Block registry name (e.g., `"minecraft:stone"`, `"minecraft:chest"`)

**Example:**
```rust
use rust_computers_api::advanced_peripherals::BlockReader;
use rust_computers_api::peripheral::find_imm;

async fn example() -> Result<(), rust_computers_api::error::PeripheralError> {
    let mut block_reader = find_imm::<BlockReader>()?;
    
    // Using the async convenience method
    let block_name = block_reader.async_get_block_name().await?;
    println!("Block name: {}", block_name);
    
    // Or using the manual pattern
    block_reader.book_next_get_block_name();
    rust_computers_api::wait_for_next_tick().await;
    let block_name = block_reader.read_last_get_block_name()?;
    println!("Block name: {}", block_name);
    
    Ok(())
}
```
---

#### `getBlockState()` / `book_next_get_block_state()` / `read_last_get_block_state()` / `async_get_block_state()`

Get the block state properties of the block in front.

**Rust Signatures:**
```rust
pub fn book_next_get_block_state(&mut self)
pub fn read_last_get_block_state(&self) -> Result<BlockState, PeripheralError>
pub async fn async_get_block_state(&self) -> Result<BlockState, PeripheralError>
```

**Returns:** `table` — Block state properties (e.g., `{facing: "north", powered: true}`)

**Example:**
```rust
use rust_computers_api::advanced_peripherals::BlockReader;
use rust_computers_api::peripheral::find_imm;

async fn example() -> Result<(), rust_computers_api::error::PeripheralError> {
    let mut block_reader = find_imm::<BlockReader>()?;
    
    // Get block state properties
    let block_state = block_reader.async_get_block_state().await?;
    
    // BlockState is a msgpack::Value that can be inspected
    if let Some(facing) = block_state.get("facing") {
        println!("Block is facing: {}", facing);
    }
    
    if let Some(powered) = block_state.get("powered") {
        println!("Block is powered: {}", powered);
    }
    
    Ok(())
}
```
---

#### `getBlockTags()` / `book_next_get_block_tags()` / `read_last_get_block_tags()` / `async_get_block_tags()`

Get the tags associated with the block.

**Rust Signatures:**
```rust
pub fn book_next_get_block_tags(&mut self)
pub fn read_last_get_block_tags(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_block_tags(&self) -> Result<Vec<String>, PeripheralError>
```

**Returns:** `table` — Array of tag strings

**Example:**
```rust
// Rust example to be added
```
---

#### `getBlockNBT()` / `book_next_get_block_nbt()` / `read_last_get_block_nbt()` / `async_get_block_nbt()`

Get the NBT data of the block (if it's a tile entity).

**Rust Signatures:**
```rust
pub fn book_next_get_block_nbt(&mut self)
pub fn read_last_get_block_nbt(&self) -> Result<Option<NBTData>, PeripheralError>
pub async fn async_get_block_nbt(&self) -> Result<Option<NBTData>, PeripheralError>
```

**Returns:** `table | nil` — NBT data or `nil` if not a tile entity

**Example:**
```rust
// Rust example to be added
```
---

#### `isTileEntity()` / `book_next_is_tile_entity()` / `read_last_is_tile_entity()` / `async_is_tile_entity()`

Check if the block is a tile entity (block entity).

**Rust Signatures:**
```rust
pub fn book_next_is_tile_entity(&mut self)
pub fn read_last_is_tile_entity(&self) -> Result<bool, PeripheralError>
pub async fn async_is_tile_entity(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if the block is a tile entity

**Example:**
```rust
// Rust example to be added
```
---

#### `getBlockInfo()` / `book_next_get_block_info()` / `read_last_get_block_info()` / `async_get_block_info()`

Get complete information about the block.

**Rust Signatures:**
```rust
pub fn book_next_get_block_info(&mut self)
pub fn read_last_get_block_info(&self) -> Result<BlockInfo, PeripheralError>
pub async fn async_get_block_info(&self) -> Result<BlockInfo, PeripheralError>
```

**Returns:** `table` — Complete block information

**Example:**
```rust
// Rust example to be added
```
---

## Events

The BlockReader peripheral does not generate events.

---

## Usage Examples

### Example 1: Identify Block Type

```rust
// Rust example to be added
```
### Example 2: Check for Specific Block

```rust
// Rust example to be added
```
### Example 3: Read Chest NBT Data

```rust
// Rust example to be added
```
### Example 4: Monitor Block State Changes

```rust
// Rust example to be added
```
### Example 5: Automated Block Detection System

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **No block in front**: There is no block directly in front of the reader
- **Peripheral disconnected**: The BlockReader is no longer accessible
- **Invalid NBT data**: NBT data cannot be parsed

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### BlockInfo
```rust
// Rust example to be added
```
### BlockState
```rust
// Rust example to be added
```
---

## Notes

- The BlockReader reads the block directly in front of it
- Tile entities include chests, furnaces, hoppers, etc.
- NBT data is only available for tile entities
- Block state properties vary by block type
- Tags help identify block categories
- The three-function pattern allows for efficient batch operations
- BlockReader requires line of sight to the block

---

## Related

- [GeoScanner](./GeoScanner.md) — For scanning multiple blocks
- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation

**Example:**
```rust
// Rust example to be added
```
---

### `getBlockData()` / `book_next_get_block_data()` / `read_last_get_block_data()` / `async_get_block_data()`

Get the NBT data of the block in front of the peripheral.

**Rust Signatures:**
```rust
pub fn book_next_get_block_data(&mut self)
pub fn read_last_get_block_data(&self) -> Result<Value, PeripheralError>
pub async fn async_get_block_data(&self) -> Result<Value, PeripheralError>
```

**Returns:** `table` — NBT data as a dynamic table

**Returns:** The NBT data structure varies by block type. For example:
- Chest: `{Items = {...}, id = "minecraft:chest"}`
- Furnace: `{Items = {...}, BurnTime = 0, CookTime = 0, ...}`
- Custom blocks may have additional properties

**Example:**
```rust
// Rust example to be added
```
---

### `getBlockStates()` / `book_next_get_block_states()` / `read_last_get_block_states()` / `async_get_block_states()`

Get the block state properties of the block in front of the peripheral.

**Rust Signatures:**
```rust
pub fn book_next_get_block_states(&mut self)
pub fn read_last_get_block_states(&self) -> Result<Value, PeripheralError>
pub async fn async_get_block_states(&self) -> Result<Value, PeripheralError>
```

**Returns:** `table` — Block state properties

**Block states vary by block type. Common examples:**
- Logs: `{axis = "y", natural = true}`
- Stairs: `{facing = "north", half = "bottom", shape = "straight"}`
- Redstone: `{north = "up", south = "side", east = "none", west = "up", power = 15}`
- Doors: `{facing = "north", half = "lower", hinge = "left", open = false, powered = false}`

**Example:**
```rust
// Rust example to be added
```
---

### `isTileEntity()` / `book_next_is_tile_entity()` / `read_last_is_tile_entity()` / `async_is_tile_entity()`

Check if the block in front of the peripheral is a tile entity (block entity).

**Rust Signatures:**
```rust
pub fn book_next_is_tile_entity(&mut self)
pub fn read_last_is_tile_entity(&self) -> Result<bool, PeripheralError>
pub async fn async_is_tile_entity(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if the block is a tile entity, `false` otherwise

**Tile entities include:**
- Chests, barrels, hoppers
- Furnaces, blast furnaces, smokers
- Brewing stands, cauldrons
- Enchanting tables, anvils
- Beacons, conduits
- Custom mod blocks with tile entities

**Example:**
```rust
// Rust example to be added
```
---

## Events

The BlockReader peripheral does not generate events.

---

## Usage Examples

### Example 1: Identify Block Type

```rust
use rust_computers_api::advanced_peripherals::BlockReader;
use rust_computers_api::peripheral::find_imm;

async fn identify_block_type() -> Result<(), rust_computers_api::error::PeripheralError> {
    let mut block_reader = find_imm::<BlockReader>()?;
    
    // Get block name
    let block_name = block_reader.async_get_block_name().await?;
    println!("Block type: {}", block_name);
    
    // Check if it's a tile entity
    let is_tile_entity = block_reader.async_is_tile_entity().await?;
    println!("Is tile entity: {}", is_tile_entity);
    
    // Get block tags
    let tags = block_reader.async_get_block_tags().await?;
    println!("Block tags: {:?}", tags);
    
    // Check for specific block types
    if block_name == "minecraft:chest" {
        println!("Found a chest!");
    } else if block_name == "minecraft:furnace" {
        println!("Found a furnace!");
    } else if tags.contains(&"minecraft:logs".to_string()) {
        println!("Found a log!");
    }
    
    Ok(())
}
```

### Example 2: Check Chest Contents

```rust
use rust_computers_api::advanced_peripherals::BlockReader;
use rust_computers_api::peripheral::find_imm;

async fn check_chest_contents() -> Result<(), rust_computers_api::error::PeripheralError> {
    let mut block_reader = find_imm::<BlockReader>()?;
    
    // First check if it's a chest
    let block_name = block_reader.async_get_block_name().await?;
    
    if block_name == "minecraft:chest" {
        println!("Found a chest, checking contents...");
        
        // Get NBT data
        let nbt_data = block_reader.async_get_block_data().await?;
        
        // Check for items in the chest
        if let Some(items) = nbt_data.get("Items") {
            if let Some(items_array) = items.as_array() {
                println!("Chest contains {} items:", items_array.len());
                
                for (i, item) in items_array.iter().enumerate() {
                    if let Some(item_map) = item.as_map() {
                        let slot = item_map.get("Slot").and_then(|v| v.as_u64());
                        let id = item_map.get("id").and_then(|v| v.as_str());
                        let count = item_map.get("Count").and_then(|v| v.as_u64());
                        
                        if let (Some(slot), Some(id), Some(count)) = (slot, id, count) {
                            println!("  Slot {}: {} x{}", slot, id, count);
                        }
                    }
                }
            }
        } else {
            println!("Chest is empty");
        }
    } else {
        println!("Not a chest, found: {}", block_name);
    }
    
    Ok(())
}
```
### Example 3: Monitor Block State Changes

```rust
// Rust example to be added
```
### Example 4: Detect Redstone Signal

```rust
// Rust example to be added
```
### Example 5: Inventory Monitoring

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **No block in front**: The peripheral is not facing a block (e.g., facing air)
- **Peripheral disconnected**: The peripheral is no longer accessible
- **Invalid state**: The block state cannot be read

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### BlockState
```rust
// BlockState is represented as a msgpack::Value (dynamic map)
use rust_computers_api::msgpack;

// Example of working with BlockState
fn process_block_state(state: msgpack::Value) {
    // Check if block has specific properties
    if let Some(facing) = state.get("facing") {
        println!("Facing direction: {}", facing);
    }
    
    if let Some(powered) = state.get("powered") {
        let is_powered: bool = powered.as_bool().unwrap_or(false);
        println!("Powered: {}", is_powered);
    }
    
    // Common block state properties:
    // - facing: "north", "south", "east", "west", "up", "down"
    // - powered: true/false
    // - open: true/false (doors, gates)
    // - half: "top", "bottom" (stairs, slabs)
    // - axis: "x", "y", "z" (logs, pillars)
    // - waterlogged: true/false
}
```

### NBTData
```rust
// NBTData is also represented as msgpack::Value
// The structure varies by block type

// Example for a chest
fn process_chest_nbt(nbt: msgpack::Value) {
    if let Some(items) = nbt.get("Items") {
        println!("Chest contains items");
        
        // Items is typically an array of item entries
        if let Some(items_array) = items.as_array() {
            for item in items_array {
                if let Some(item_map) = item.as_map() {
                    let slot = item_map.get("Slot").and_then(|v| v.as_u64());
                    let id = item_map.get("id").and_then(|v| v.as_str());
                    let count = item_map.get("Count").and_then(|v| v.as_u64());
                    
                    if let (Some(slot), Some(id), Some(count)) = (slot, id, count) {
                        println!("  Slot {}: {} x{}", slot, id, count);
                    }
                }
            }
        }
    }
}

// Example for a furnace
fn process_furnace_nbt(nbt: msgpack::Value) {
    if let Some(burn_time) = nbt.get("BurnTime") {
        println!("Burn time remaining: {}", burn_time);
    }
    
    if let Some(cook_time) = nbt.get("CookTime") {
        println!("Cook time: {}", cook_time);
    }
}
```
---

## Notes

- The BlockReader reads the block directly in front of it (the direction it's facing)
- Block states are always strings or numbers, even for boolean-like values
- NBT data structure depends on the block type
- Tile entities have additional NBT data beyond simple blocks
- The three-function pattern allows for efficient batch operations

---

## Related

- [GeoScanner](./GeoScanner.md) — For scanning multiple blocks in an area
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
