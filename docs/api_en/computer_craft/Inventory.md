# Inventory

**Mod:** CC:Tweaked  
**Peripheral Type:** `inventory`  
**Source:** `AbstractInventoryMethods.java`

## Overview

The Inventory peripheral provides access to block inventories (chests, barrels, furnaces, etc.) through a wired network. It allows you to query inventory contents, get detailed item information, and transfer items between connected inventories.

## Three-Function Pattern

The Inventory API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```lua
-- Method 1: book_next / read_last pattern
inventory.book_next_list()
wait_for_next_tick()
local items = inventory.read_last_list()

-- Method 2: async pattern (recommended)
local items = inventory.async_list()
```

## Methods

### `size()` / `book_next_size()` / `read_last_size()` / `async_size()`

Get the number of slots in the inventory.

**Lua Signature:**
```lua
function size() -> number
```

**Rust Signatures:**
```rust
pub fn book_next_size(&mut self)
pub fn read_last_size(&self) -> Result<u32, PeripheralError>
pub async fn async_size(&self) -> Result<u32, PeripheralError>
```

**Returns:** `number` — Total number of slots

**Example:**
```lua
local inventory = peripheral.find("inventory")
local slot_count = inventory.async_size()
print("Inventory has " .. slot_count .. " slots")
```

---

### `list()` / `book_next_list()` / `read_last_list()` / `async_list()`

List all items in the inventory with summary information.

**Lua Signature:**
```lua
function list() -> table
```

**Rust Signatures:**
```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
pub async fn async_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
```

**Returns:** `table` — Sparse table mapping slot numbers to item info

**Item Info Structure:**
```lua
{
  name = "minecraft:diamond",  -- Item registry name
  count = 5,                    -- Stack size
}
```

**Example:**
```lua
local inventory = peripheral.find("inventory")
local items = inventory.async_list()

for slot, item in pairs(items) do
  print(("Slot %d: %d x %s"):format(slot, item.count, item.name))
end
```

---

### `getItemDetail(slot)` / `book_next_get_item_detail(slot)` / `read_last_get_item_detail()` / `async_get_item_detail(slot)`

Get detailed information about an item in a specific slot.

**Lua Signature:**
```lua
function getItemDetail(slot: number) -> table | nil
```

**Rust Signatures:**
```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<ItemDetail>, PeripheralError>
pub async fn async_get_item_detail(&self, slot: u32) -> Result<Option<ItemDetail>, PeripheralError>
```

**Parameters:**
- `slot: number` — Slot index (1-based)

**Returns:** `table | nil` — Item detail table or `nil` if slot is empty

**Item Detail Structure:**
```lua
{
  name = "minecraft:diamond",      -- Item registry name
  count = 5,                        -- Stack size
  displayName = "Diamond",          -- Display name
  maxCount = 64,                    -- Maximum stack size
  damage = 10,                      -- Durability damage (optional)
  maxDamage = 100,                  -- Maximum durability (optional)
  tags = {                          -- Item tags
    ["minecraft:gems"] = true,
  },
}
```

**Example:**
```lua
local inventory = peripheral.find("inventory")
local item = inventory.async_get_item_detail(1)

if item then
  print(("Slot 1: %d x %s"):format(item.count, item.displayName))
  if item.damage then
    print(("Durability: %d/%d"):format(item.damage, item.maxDamage))
  end
else
  print("Slot 1 is empty")
end
```

**Error Handling:**
- Throws error if slot number is out of range

---

### `getItemLimit(slot)` / `book_next_get_item_limit(slot)` / `read_last_get_item_limit()` / `async_get_item_limit(slot)`

Get the maximum number of items that can be stored in a slot.

**Lua Signature:**
```lua
function getItemLimit(slot: number) -> number
```

**Rust Signatures:**
```rust
pub fn book_next_get_item_limit(&mut self, slot: u32)
pub fn read_last_get_item_limit(&self) -> Result<u32, PeripheralError>
pub async fn async_get_item_limit(&self, slot: u32) -> Result<u32, PeripheralError>
```

**Parameters:**
- `slot: number` — Slot index (1-based)

**Returns:** `number` — Maximum item count for this slot (usually 64, but can be higher for special inventories)

**Example:**
```lua
local inventory = peripheral.find("inventory")
local limit = inventory.async_get_item_limit(1)
print("Slot 1 can hold up to " .. limit .. " items")
```

---

### `pushItems(toName, fromSlot, limit?, toSlot?)` / `book_next_push_items(...)` / `read_last_push_items()` / `async_push_items(...)`

Transfer items from this inventory to another connected inventory.

**Lua Signature:**
```lua
function pushItems(toName: string, fromSlot: number, limit?: number, toSlot?: number) -> number
```

**Rust Signatures:**
```rust
pub fn book_next_push_items(&mut self, to_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>)
pub fn read_last_push_items(&self) -> Result<u32, PeripheralError>
pub async fn async_push_items(&self, to_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters:**
- `toName: string` — Name of destination inventory (from `peripheral.getNamesRemote()`)
- `fromSlot: number` — Source slot index (1-based)
- `limit?: number` — Maximum items to transfer (optional, defaults to stack limit)
- `toSlot?: number` — Destination slot index (optional, auto-selects if omitted)

**Returns:** `number` — Number of items actually transferred

**Requirements:**
- Both inventories must be connected via wired modem and network cable
- Destination inventory must exist and be accessible

**Example:**
```lua
local source = peripheral.find("minecraft:chest_0")
local dest_name = "minecraft:chest_1"

-- Transfer 32 diamonds from slot 1 to destination
local moved = source.async_push_items(dest_name, 1, 32)
print("Moved " .. moved .. " items")
```

**Error Handling:**
- Throws error if destination inventory doesn't exist
- Throws error if slot is out of range

---

### `pullItems(fromName, fromSlot, limit?, toSlot?)` / `book_next_pull_items(...)` / `read_last_pull_items()` / `async_pull_items(...)`

Transfer items from another connected inventory into this one.

**Lua Signature:**
```lua
function pullItems(fromName: string, fromSlot: number, limit?: number, toSlot?: number) -> number
```

**Rust Signatures:**
```rust
pub fn book_next_pull_items(&mut self, from_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>)
pub fn read_last_pull_items(&self) -> Result<u32, PeripheralError>
pub async fn async_pull_items(&self, from_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters:**
- `fromName: string` — Name of source inventory (from `peripheral.getNamesRemote()`)
- `fromSlot: number` — Source slot index (1-based)
- `limit?: number` — Maximum items to transfer (optional, defaults to stack limit)
- `toSlot?: number` — Destination slot index (optional, auto-selects if omitted)

**Returns:** `number` — Number of items actually transferred

**Requirements:**
- Both inventories must be connected via wired modem and network cable
- Source inventory must exist and be accessible

**Example:**
```lua
local dest = peripheral.find("minecraft:chest_0")
local source_name = "minecraft:chest_1"

-- Pull 32 diamonds from source slot 1
local moved = dest.async_pull_items(source_name, 1, 32)
print("Pulled " .. moved .. " items")
```

**Error Handling:**
- Throws error if source inventory doesn't exist
- Throws error if slot is out of range

---

## Events

The Inventory peripheral does not generate events.

---

## Usage Examples

### Example 1: List All Items

```lua
local inventory = peripheral.find("minecraft:chest")

local items = inventory.async_list()
for slot, item in pairs(items) do
  print(("Slot %d: %d x %s"):format(slot, item.count, item.name))
end
```

### Example 2: Find Item by Name

```lua
local inventory = peripheral.find("minecraft:chest")

local function find_item(name)
  local items = inventory.async_list()
  for slot, item in pairs(items) do
    if item.name == name then
      return slot, item
    end
  end
  return nil
end

local slot, item = find_item("minecraft:diamond")
if slot then
  print(("Found %d diamonds in slot %d"):format(item.count, slot))
end
```

### Example 3: Transfer Items Between Chests

```lua
local source = peripheral.find("minecraft:chest_0")
local dest_name = "minecraft:chest_1"

-- Transfer all items from source to destination
local items = source.async_list()
for slot, item in pairs(items) do
  local moved = source.async_push_items(dest_name, slot)
  print(("Moved %d items from slot %d"):format(moved, slot))
end
```

### Example 4: Calculate Total Capacity

```lua
local inventory = peripheral.find("minecraft:chest")

local total_capacity = 0
for i = 1, inventory.async_size() do
  total_capacity = total_capacity + inventory.async_get_item_limit(i)
end

print("Total capacity: " .. total_capacity)
```

### Example 5: Inventory Sorting

```lua
local inventory = peripheral.find("minecraft:chest")

-- Sort items by name
local items = inventory.async_list()
local sorted = {}
for slot, item in pairs(items) do
  table.insert(sorted, {slot = slot, item = item})
end

table.sort(sorted, function(a, b)
  return a.item.name < b.item.name
end)

for _, entry in ipairs(sorted) do
  print(("Slot %d: %s"):format(entry.slot, entry.item.name))
end
```

---

## Error Handling

All methods may throw errors in the following cases:

- **Inventory not found**: Peripheral is disconnected or not accessible
- **Invalid slot**: Slot number is out of range (1 to size)
- **Network error**: Wired network connection is broken
- **Destination not found**: Target inventory doesn't exist or isn't accessible

**Example Error Handling:**
```lua
local inventory = peripheral.find("minecraft:chest")
if not inventory then
  error("No inventory found")
end

local success, result = pcall(function()
  return inventory.async_list()
end)

if not success then
  print("Error: " .. result)
else
  print("Got " .. #result .. " items")
end
```

---

## Type Definitions

### ItemDetail
```lua
{
  name: string,           -- Registry name (e.g., "minecraft:diamond")
  count: number,          -- Stack size
  displayName: string,    -- Display name
  maxCount: number,       -- Maximum stack size
  damage?: number,        -- Durability damage (optional)
  maxDamage?: number,     -- Maximum durability (optional)
  tags?: table,           -- Item tags (optional)
}
```

### SlotInfo
```lua
{
  name: string,   -- Registry name
  count: number,  -- Stack size
}
```

---

## Notes

- All slot indices are 1-based (first slot is 1, not 0)
- `pushItems` and `pullItems` require wired network connection
- Inventory names come from `peripheral.getNamesRemote()` on a wired modem
- Empty slots are not included in the `list()` result (sparse table)
- The three-function pattern allows for efficient batch operations

---

## Related

- [Modem](./Modem.md) — Required for wired network communication
- [CC:Tweaked Documentation](https://tweaked.cc/) — Official documentation
