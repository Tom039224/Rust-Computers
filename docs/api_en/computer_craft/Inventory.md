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

```rust
// Rust example to be added
```
## Implementation Status

### ✅ Implemented

- book_next_size / read_last_size / async_size
- book_next_list / read_last_list / async_list
- book_next_get_item_detail / read_last_get_item_detail / async_get_item_detail
- book_next_push_items / read_last_push_items / async_push_items
- book_next_pull_items / read_last_pull_items / async_pull_items

### ✅ Also Implemented

- book_next_get_item_limit / read_last_get_item_limit / async_get_item_limit


## Methods

### `size()` / `book_next_size()` / `read_last_size()` / `async_size()`

Get the number of slots in the inventory.

**Rust Signatures:**
```rust
pub fn book_next_size(&mut self)
pub fn read_last_size(&self) -> Result<u32, PeripheralError>
pub async fn async_size(&self) -> Result<u32, PeripheralError>
```

**Returns:** `number` — Total number of slots

**Example:**
```rust
// Rust example to be added
```
---

### `list()` / `book_next_list()` / `read_last_list()` / `async_list()`

List all items in the inventory with summary information.

**Rust Signatures:**
```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
pub async fn async_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
```

**Returns:** `table` — Sparse table mapping slot numbers to item info

**Item Info Structure:**
```rust
// Rust example to be added
```
**Example:**
```rust
// Rust example to be added
```
---

### `getItemDetail(slot)` / `book_next_get_item_detail(slot)` / `read_last_get_item_detail()` / `async_get_item_detail(slot)`

Get detailed information about an item in a specific slot.

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
```rust
// Rust example to be added
```
**Example:**
```rust
// Rust example to be added
```
**Error Handling:**
- Throws error if slot number is out of range

---

### `getItemLimit(slot)` 🚧 / `book_next_get_item_limit(slot)` / `read_last_get_item_limit()` / `async_get_item_limit(slot)`

Get the maximum number of items that can be stored in a slot.

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
```rust
// Rust example to be added
```
---

### `pushItems(toName, fromSlot, limit?, toSlot?)` / `book_next_push_items(...)` / `read_last_push_items()` / `async_push_items(...)`

Transfer items from this inventory to another connected inventory.

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
```rust
// Rust example to be added
```
**Error Handling:**
- Throws error if destination inventory doesn't exist
- Throws error if slot is out of range

---

### `pullItems(fromName, fromSlot, limit?, toSlot?)` / `book_next_pull_items(...)` / `read_last_pull_items()` / `async_pull_items(...)`

Transfer items from another connected inventory into this one.

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
```rust
// Rust example to be added
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

```rust
// Rust example to be added
```
### Example 2: Find Item by Name

```rust
// Rust example to be added
```
### Example 3: Transfer Items Between Chests

```rust
// Rust example to be added
```
### Example 4: Calculate Total Capacity

```rust
// Rust example to be added
```
### Example 5: Inventory Sorting

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Inventory not found**: Peripheral is disconnected or not accessible
- **Invalid slot**: Slot number is out of range (1 to size)
- **Network error**: Wired network connection is broken
- **Destination not found**: Target inventory doesn't exist or isn't accessible

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### ItemDetail
```rust
// Rust example to be added
```
### SlotInfo
```rust
// Rust example to be added
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
