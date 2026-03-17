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

```lua
-- Method 1: book_next / read_last pattern
reader.book_next_get_block_name()
wait_for_next_tick()
local name = reader.read_last_get_block_name()

-- Method 2: async pattern (recommended)
local name = reader.async_get_block_name()
```

## Methods

### `getBlockName()` / `book_next_get_block_name()` / `read_last_get_block_name()` / `async_get_block_name()`

Get the registry name of the block in front of the peripheral.

**Lua Signature:**
```lua
function getBlockName() -> string
```

**Rust Signatures:**
```rust
pub fn book_next_get_block_name(&mut self)
pub fn read_last_get_block_name(&self) -> Result<String, PeripheralError>
pub async fn async_get_block_name(&self) -> Result<String, PeripheralError>
```

**Returns:** `string` — Block registry name (e.g., `"minecraft:stone"`, `"minecraft:chest"`)

**Example:**
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local name = reader.async_get_block_name()
print("Block: " .. name)
```

---

### `getBlockData()` / `book_next_get_block_data()` / `read_last_get_block_data()` / `async_get_block_data()`

Get the NBT data of the block in front of the peripheral.

**Lua Signature:**
```lua
function getBlockData() -> table
```

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
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local data = reader.async_get_block_data()

-- Check if it's a chest with items
if data.Items then
  print("Found " .. #data.Items .. " items in chest")
end
```

---

### `getBlockStates()` / `book_next_get_block_states()` / `read_last_get_block_states()` / `async_get_block_states()`

Get the block state properties of the block in front of the peripheral.

**Lua Signature:**
```lua
function getBlockStates() -> table
```

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
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local states = reader.async_get_block_states()

-- Check if a door is open
if states.open then
  print("Door is open")
else
  print("Door is closed")
end
```

---

### `isTileEntity()` / `book_next_is_tile_entity()` / `read_last_is_tile_entity()` / `async_is_tile_entity()`

Check if the block in front of the peripheral is a tile entity (block entity).

**Lua Signature:**
```lua
function isTileEntity() -> boolean
```

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
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local is_te = reader.async_is_tile_entity()

if is_te then
  print("Block has tile entity data")
  local data = reader.async_get_block_data()
  print("NBT: " .. textutils.serialize(data))
else
  print("Block is a simple block")
end
```

---

## Events

The BlockReader peripheral does not generate events.

---

## Usage Examples

### Example 1: Identify Block Type

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local name = reader.async_get_block_name()
local states = reader.async_get_block_states()

print("Block: " .. name)
print("States: " .. textutils.serialize(states))
```

### Example 2: Check Chest Contents

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local name = reader.async_get_block_name()
if name == "minecraft:chest" then
  local data = reader.async_get_block_data()
  if data.Items then
    print("Chest contains " .. #data.Items .. " stacks")
    for i, item in ipairs(data.Items) do
      print(("  Slot %d: %d x %s"):format(item.Slot, item.Count, item.id))
    end
  else
    print("Chest is empty")
  end
else
  print("Not a chest: " .. name)
end
```

### Example 3: Monitor Block State Changes

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local last_state = nil

while true do
  local states = reader.async_get_block_states()
  
  if last_state and last_state.open ~= states.open then
    if states.open then
      print("Door opened!")
    else
      print("Door closed!")
    end
  end
  
  last_state = states
  sleep(0.5)
end
```

### Example 4: Detect Redstone Signal

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local name = reader.async_get_block_name()
if name == "minecraft:redstone_wire" then
  local states = reader.async_get_block_states()
  local power = tonumber(states.power) or 0
  print("Redstone power level: " .. power)
end
```

### Example 5: Inventory Monitoring

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local function get_inventory_size()
  local data = reader.async_get_block_data()
  if data.Items then
    local max_slot = 0
    for _, item in ipairs(data.Items) do
      if item.Slot > max_slot then
        max_slot = item.Slot
      end
    end
    return max_slot
  end
  return 0
end

local size = get_inventory_size()
print("Inventory has " .. size .. " slots")
```

---

## Error Handling

All methods may throw errors in the following cases:

- **No block in front**: The peripheral is not facing a block (e.g., facing air)
- **Peripheral disconnected**: The peripheral is no longer accessible
- **Invalid state**: The block state cannot be read

**Example Error Handling:**
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
if not reader then
  error("BlockReader not found")
end

local success, result = pcall(function()
  return reader.async_get_block_name()
end)

if not success then
  print("Error: " .. result)
else
  print("Block: " .. result)
end
```

---

## Type Definitions

### BlockState
```lua
{
  [key: string]: string | number | boolean,
  -- Examples:
  -- facing = "north"
  -- half = "bottom"
  -- open = false
  -- power = 15
}
```

### NBTData
```lua
{
  -- Varies by block type
  -- Common properties:
  id?: string,           -- Block ID
  Items?: table,         -- Inventory items (if applicable)
  BurnTime?: number,     -- Furnace burn time (if applicable)
  CookTime?: number,     -- Furnace cook time (if applicable)
  -- ... other block-specific properties
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
