# GeoScanner

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:geo_scanner`  
**Source:** `GeoScannerPeripheral.java`

## Overview

The GeoScanner peripheral scans surrounding blocks within a specified radius and analyzes ore distribution in the current chunk. It provides detailed information about block types, their positions, and tags. This is useful for mining automation, resource location, and terrain analysis systems.

## Three-Function Pattern

The GeoScanner API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```lua
-- Method 1: book_next / read_last pattern
scanner.book_next_scan(16)
wait_for_next_tick()
local blocks = scanner.read_last_scan()

-- Method 2: async pattern (recommended)
local blocks = scanner.async_scan(16)
```

## Methods

### `cost(radius)` / `book_next_cost(radius)` / `read_last_cost()` / `cost_imm(radius)`

Get the fuel cost for a scan at the given radius.

**Lua Signature:**
```lua
function cost(radius: number) -> number
```

**Rust Signatures:**
```rust
pub fn book_next_cost(&mut self, radius: f64)
pub fn read_last_cost(&self) -> Result<f64, PeripheralError>
pub fn cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```

**Parameters:**
- `radius: number` — Scan radius in blocks

**Returns:** `number` — Fuel cost (in AE units or similar)

**Note:** The `cost_imm` method returns the result immediately without waiting for the next tick.

**Example:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

-- Check cost before scanning
local cost = scanner.cost_imm(16)
print("Scan cost: " .. cost)

if cost < 100 then
  local blocks = scanner.async_scan(16)
end
```

---

### `scan(radius)` / `book_next_scan(radius)` / `read_last_scan()` / `async_scan(radius)`

Scan blocks within the given radius.

**Lua Signature:**
```lua
function scan(radius: number) -> table
```

**Rust Signatures:**
```rust
pub fn book_next_scan(&mut self, radius: f64)
pub fn read_last_scan(&self) -> Result<Vec<GeoBlockEntry>, PeripheralError>
pub async fn async_scan(&self, radius: f64) -> Result<Vec<GeoBlockEntry>, PeripheralError>
```

**Parameters:**
- `radius: number` — Scan radius in blocks

**Returns:** `table` — Array of block entries

**Block Entry Structure:**
```lua
{
  x: number,           -- X coordinate (relative to scanner)
  y: number,           -- Y coordinate (relative to scanner)
  z: number,           -- Z coordinate (relative to scanner)
  name: string,        -- Block registry name (e.g., "minecraft:diamond_ore")
  tags: table,         -- Block tags (e.g., {"minecraft:ores", "minecraft:mineable/pickaxe"})
}
```

**Example:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local blocks = scanner.async_scan(16)
print("Found " .. #blocks .. " blocks")

for _, block in ipairs(blocks) do
  print(("Block at (%d, %d, %d): %s"):format(block.x, block.y, block.z, block.name))
end
```

---

### `chunkAnalyze()` / `book_next_chunk_analyze()` / `read_last_chunk_analyze()` / `async_chunk_analyze()`

Analyze ore distribution in the current chunk.

**Lua Signature:**
```lua
function chunkAnalyze() -> table
```

**Rust Signatures:**
```rust
pub fn book_next_chunk_analyze(&mut self)
pub fn read_last_chunk_analyze(&self) -> Result<Value, PeripheralError>
pub async fn async_chunk_analyze(&self) -> Result<Value, PeripheralError>
```

**Returns:** `table` — Map of ore names to counts

**Return Structure:**
```lua
{
  ["minecraft:diamond_ore"] = 5,
  ["minecraft:iron_ore"] = 12,
  ["minecraft:gold_ore"] = 3,
  ["minecraft:coal_ore"] = 8,
  -- ... other ores
}
```

**Example:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local ores = scanner.async_chunk_analyze()
print("Ore distribution in chunk:")

for ore_name, count in pairs(ores) do
  print(("  %s: %d"):format(ore_name, count))
end
```

---

## Events

The GeoScanner peripheral does not generate events.

---

## Usage Examples

### Example 1: Find Specific Ore

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function find_ore(ore_name, radius)
  local blocks = scanner.async_scan(radius)
  
  for _, block in ipairs(blocks) do
    if block.name == ore_name then
      return block
    end
  end
  
  return nil
end

local diamond = find_ore("minecraft:diamond_ore", 32)
if diamond then
  print(("Found diamond ore at (%d, %d, %d)"):format(diamond.x, diamond.y, diamond.z))
else
  print("No diamond ore found")
end
```

### Example 2: Scan and Filter by Tag

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function find_ores(radius)
  local blocks = scanner.async_scan(radius)
  local ores = {}
  
  for _, block in ipairs(blocks) do
    -- Check if block has the "minecraft:ores" tag
    for _, tag in ipairs(block.tags) do
      if tag == "minecraft:ores" then
        table.insert(ores, block)
        break
      end
    end
  end
  
  return ores
end

local ores = find_ores(16)
print("Found " .. #ores .. " ore blocks")
```

### Example 3: Chunk Analysis Report

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function analyze_chunk()
  local ores = scanner.async_chunk_analyze()
  
  local total = 0
  for _, count in pairs(ores) do
    total = total + count
  end
  
  print("Chunk Analysis Report:")
  print("Total ore blocks: " .. total)
  print("")
  
  -- Sort by count
  local sorted = {}
  for ore_name, count in pairs(ores) do
    table.insert(sorted, {name = ore_name, count = count})
  end
  
  table.sort(sorted, function(a, b)
    return a.count > b.count
  end)
  
  for _, entry in ipairs(sorted) do
    print(("  %s: %d"):format(entry.name, entry.count))
  end
end

analyze_chunk()
```

### Example 4: Cost-Aware Scanning

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function smart_scan(max_cost)
  -- Find the largest radius we can afford
  local radius = 1
  while true do
    local cost = scanner.cost_imm(radius + 1)
    if cost > max_cost then
      break
    end
    radius = radius + 1
  end
  
  print("Scanning with radius: " .. radius)
  local blocks = scanner.async_scan(radius)
  
  return blocks
end

local blocks = smart_scan(500)
print("Found " .. #blocks .. " blocks")
```

### Example 5: Mining Automation

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")
local robot = peripheral.find("robot")  -- Hypothetical robot peripheral

local function mine_nearest_ore(ore_name, radius)
  local blocks = scanner.async_scan(radius)
  
  -- Find nearest ore
  local nearest = nil
  local min_distance = math.huge
  
  for _, block in ipairs(blocks) do
    if block.name == ore_name then
      local distance = math.sqrt(block.x^2 + block.y^2 + block.z^2)
      if distance < min_distance then
        min_distance = distance
        nearest = block
      end
    end
  end
  
  if nearest then
    print(("Mining %s at (%d, %d, %d)"):format(ore_name, nearest.x, nearest.y, nearest.z))
    -- Move robot to ore and mine it
    robot.move_to(nearest.x, nearest.y, nearest.z)
    robot.mine()
  else
    print("No ore found")
  end
end

mine_nearest_ore("minecraft:diamond_ore", 32)
```

---

## Error Handling

All methods may throw errors in the following cases:

- **Insufficient fuel**: Not enough energy to perform the scan
- **Peripheral disconnected**: The GeoScanner is no longer accessible
- **Invalid radius**: Radius is negative or exceeds maximum

**Example Error Handling:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")
if not scanner then
  error("GeoScanner not found")
end

local success, result = pcall(function()
  return scanner.async_scan(16)
end)

if not success then
  print("Error: " .. result)
else
  print("Found " .. #result .. " blocks")
end
```

---

## Type Definitions

### GeoBlockEntry
```lua
{
  x: number,           -- X coordinate (relative to scanner)
  y: number,           -- Y coordinate (relative to scanner)
  z: number,           -- Z coordinate (relative to scanner)
  name: string,        -- Block registry name
  tags: table,         -- Array of block tags
}
```

### ChunkAnalysisResult
```lua
{
  [ore_name: string]: number,  -- Ore name to count mapping
  -- Examples:
  -- ["minecraft:diamond_ore"] = 5
  -- ["minecraft:iron_ore"] = 12
}
```

---

## Notes

- Coordinates returned by `scan()` are relative to the scanner's position
- The `cost_imm()` method is the only immediate method (returns without waiting)
- Chunk analysis scans the entire current chunk, not just the radius
- Block tags can be used to filter results (e.g., "minecraft:ores", "minecraft:mineable/pickaxe")
- The three-function pattern allows for efficient batch operations
- Larger scan radii consume more fuel/energy

---

## Related

- [BlockReader](./BlockReader.md) — For reading detailed block information
- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
