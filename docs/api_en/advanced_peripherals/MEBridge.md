# MEBridge

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:me_bridge`  
**Source:** `MEBridgePeripheral.java`

## Overview

The MEBridge peripheral provides full access to Applied Energistics 2 (AE2) ME networks. It allows you to query and manipulate items, fluids, and chemicals in the ME system, request crafting operations, monitor energy levels, and manage storage. This is essential for creating automated systems that interact with AE2 networks.

## Three-Function Pattern

The MEBridge API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
// Rust example to be added
```
## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods (~60 methods)

### 🚧 Not Yet Implemented

- async_* variants for all methods (~60 methods)


## Methods

### Item Operations

#### `listItems()` / `book_next_list_items()` / `read_last_list_items()` / `async_list_items()`

List all items currently stored in the ME network.

**Rust Signatures:**
```rust
pub fn book_next_list_items(&mut self)
pub fn read_last_list_items(&self) -> Result<Vec<MEItemEntry>, PeripheralError>
pub async fn async_list_items(&self) -> Result<Vec<MEItemEntry>, PeripheralError>
```

**Returns:** `table` — Array of item entries

**Example:**
```rust
// Rust example to be added
```
---

#### `getItem(filter)` / `book_next_get_item(filter)` / `read_last_get_item()` / `async_get_item(filter)`

Get the first item matching the specified filter.

**Rust Signatures:**
```rust
pub fn book_next_get_item(&mut self, filter: &[u8])
pub fn read_last_get_item(&self) -> Result<MEItemEntry, PeripheralError>
pub async fn async_get_item(&self, filter: &[u8]) -> Result<MEItemEntry, PeripheralError>
```

**Parameters:**
- `filter: table` — Filter table with properties like `name`, `displayName`, `tags`, etc.

**Filter Examples:**
```rust
// Rust example to be added
```
**Returns:** `table | nil` — Item entry or `nil` if not found

**Example:**
```rust
// Rust example to be added
```
---

#### `exportItem(filter, side, amount?)` / `book_next_export_item(...)` / `read_last_export_item()` / `async_export_item(...)`

Export items from the ME network to an adjacent inventory.

**Rust Signatures:**
```rust
pub fn book_next_export_item(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_export_item(&self) -> Result<u32, PeripheralError>
pub async fn async_export_item(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters:**
- `filter: table` — Item filter
- `side: string` — Direction ("north", "south", "east", "west", "up", "down")
- `amount?: number` — Maximum items to export (optional)

**Returns:** `number` — Number of items actually exported

**Example:**
```rust
// Rust example to be added
```
---

#### `importItem(filter, side, amount?)` / `book_next_import_item(...)` / `read_last_import_item()` / `async_import_item(...)`

Import items from an adjacent inventory into the ME network.

**Rust Signatures:**
```rust
pub fn book_next_import_item(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_import_item(&self) -> Result<u32, PeripheralError>
pub async fn async_import_item(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters / Returns:** Same as `exportItem`

**Example:**
```rust
// Rust example to be added
```
---

#### `exportItemToPeripheral(filter, targetName, amount?)` / `book_next_export_item_to_peripheral(...)` / `read_last_export_item_to_peripheral()` / `async_export_item_to_peripheral(...)`

Export items from ME to a named peripheral (via modem network).

**Rust Signatures:**
```rust
pub fn book_next_export_item_to_peripheral(&mut self, filter: &[u8], target_name: &str, amount: Option<u32>)
pub fn read_last_export_item_to_peripheral(&self) -> Result<u32, PeripheralError>
pub async fn async_export_item_to_peripheral(&self, filter: &[u8], target_name: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters:**
- `filter: table` — Item filter
- `targetName: string` — Name of target peripheral
- `amount?: number` — Maximum items to export

**Returns:** `number` — Items exported

---

#### `importItemFromPeripheral(filter, targetName, amount?)` / `book_next_import_item_from_peripheral(...)` / `read_last_import_item_from_peripheral()` / `async_import_item_from_peripheral(...)`

Import items from a named peripheral into ME.

**Rust Signatures:**
```rust
pub fn book_next_import_item_from_peripheral(&mut self, filter: &[u8], target_name: &str, amount: Option<u32>)
pub fn read_last_import_item_from_peripheral(&self) -> Result<u32, PeripheralError>
pub async fn async_import_item_from_peripheral(&self, filter: &[u8], target_name: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters / Returns:** Same as `exportItemToPeripheral`

---

### Fluid Operations

#### `listFluids()` / `book_next_list_fluids()` / `read_last_list_fluids()` / `async_list_fluids()`

List all fluids currently stored in the ME network.

**Rust Signatures:**
```rust
pub fn book_next_list_fluids(&mut self)
pub fn read_last_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
pub async fn async_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
```

**Returns:** `table` — Array of fluid entries

**Example:**
```rust
// Rust example to be added
```
---

#### `getFluid(filter)` / `book_next_get_fluid(filter)` / `read_last_get_fluid()` / `async_get_fluid(filter)`

Get the first fluid matching the specified filter.

**Rust Signatures:**
```rust
pub fn book_next_get_fluid(&mut self, filter: &[u8])
pub fn read_last_get_fluid(&self) -> Result<MEFluidEntry, PeripheralError>
pub async fn async_get_fluid(&self, filter: &[u8]) -> Result<MEFluidEntry, PeripheralError>
```

**Parameters / Returns:** Same as `getItem`

---

#### `exportFluid(filter, side, amount?)` / `book_next_export_fluid(...)` / `read_last_export_fluid()` / `async_export_fluid(...)`

Export fluids from ME to an adjacent tank.

**Rust Signatures:**
```rust
pub fn book_next_export_fluid(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_export_fluid(&self) -> Result<u32, PeripheralError>
pub async fn async_export_fluid(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters / Returns:** Same as `exportItem` (amount in mB)

---

#### `importFluid(filter, side, amount?)` / `book_next_import_fluid(...)` / `read_last_import_fluid()` / `async_import_fluid(...)`

Import fluids from an adjacent tank into ME.

**Rust Signatures:**
```rust
pub fn book_next_import_fluid(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_import_fluid(&self) -> Result<u32, PeripheralError>
pub async fn async_import_fluid(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters / Returns:** Same as `importItem`

---

#### `exportFluidToPeripheral(filter, targetName, amount?)` / `book_next_export_fluid_to_peripheral(...)` / `read_last_export_fluid_to_peripheral()` / `async_export_fluid_to_peripheral(...)`

Export fluids to a named peripheral.

**Rust Signatures:**
```rust
pub fn book_next_export_fluid_to_peripheral(&mut self, filter: &[u8], target_name: &str, amount: Option<u32>)
pub fn read_last_export_fluid_to_peripheral(&self) -> Result<u32, PeripheralError>
pub async fn async_export_fluid_to_peripheral(&self, filter: &[u8], target_name: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters / Returns:** Same as `exportItemToPeripheral`

---

#### `importFluidFromPeripheral(filter, targetName, amount?)` / `book_next_import_fluid_from_peripheral(...)` / `read_last_import_fluid_from_peripheral()` / `async_import_fluid_from_peripheral(...)`

Import fluids from a named peripheral.

**Rust Signatures:**
```rust
pub fn book_next_import_fluid_from_peripheral(&mut self, filter: &[u8], target_name: &str, amount: Option<u32>)
pub fn read_last_import_fluid_from_peripheral(&self) -> Result<u32, PeripheralError>
pub async fn async_import_fluid_from_peripheral(&self, filter: &[u8], target_name: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**Parameters / Returns:** Same as `importItemToPeripheral`

---

### Crafting Operations

#### `craftItem(filter, amount?)` / `book_next_craft_item(...)` / `read_last_craft_item()` / `async_craft_item(...)`

Request crafting of an item from the ME network.

**Rust Signatures:**
```rust
pub fn book_next_craft_item(&mut self, filter: &[u8], amount: Option<u32>)
pub fn read_last_craft_item(&self) -> Result<bool, PeripheralError>
pub async fn async_craft_item(&self, filter: &[u8], amount: Option<u32>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `filter: table` — Item filter
- `amount?: number` — Amount to craft

**Returns:** `boolean` — `true` if craft request was accepted

**Example:**
```rust
// Rust example to be added
```
---

#### `craftFluid(filter, amount?)` / `book_next_craft_fluid(...)` / `read_last_craft_fluid()` / `async_craft_fluid(...)`

Request crafting of a fluid.

**Rust Signatures:**
```rust
pub fn book_next_craft_fluid(&mut self, filter: &[u8], amount: Option<u32>)
pub fn read_last_craft_fluid(&self) -> Result<bool, PeripheralError>
pub async fn async_craft_fluid(&self, filter: &[u8], amount: Option<u32>) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `craftItem`

---

#### `isItemCrafting(filter)` / `book_next_is_item_crafting(filter)` / `read_last_is_item_crafting()` / `async_is_item_crafting(filter)`

Check if an item is currently being crafted.

**Rust Signatures:**
```rust
pub fn book_next_is_item_crafting(&mut self, filter: &[u8])
pub fn read_last_is_item_crafting(&self) -> Result<bool, PeripheralError>
pub async fn async_is_item_crafting(&self, filter: &[u8]) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `craftItem`

---

#### `isFluidCrafting(filter)` / `book_next_is_fluid_crafting(filter)` / `read_last_is_fluid_crafting()` / `async_is_fluid_crafting(filter)`

Check if a fluid is currently being crafted.

**Rust Signatures:**
```rust
pub fn book_next_is_fluid_crafting(&mut self, filter: &[u8])
pub fn read_last_is_fluid_crafting(&self) -> Result<bool, PeripheralError>
pub async fn async_is_fluid_crafting(&self, filter: &[u8]) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `isItemCrafting`

---

### Storage & Energy Monitoring

#### `getEnergyStorage()` / `book_next_get_energy_storage()` / `read_last_get_energy_storage()` / `async_get_energy_storage()`

Get the current energy stored in the ME network.

**Rust Signatures:**
```rust
pub fn book_next_get_energy_storage(&mut self)
pub fn read_last_get_energy_storage(&self) -> Result<f64, PeripheralError>
pub async fn async_get_energy_storage(&self) -> Result<f64, PeripheralError>
```

**Returns:** `number` — Energy in AE units

**Example:**
```rust
// Rust example to be added
```
---

#### `getMaxEnergyStorage()` / `book_next_get_max_energy_storage()` / `read_last_get_max_energy_storage()` / `async_get_max_energy_storage()`

Get the maximum energy capacity of the ME network.

**Rust Signatures:**
```rust
pub fn book_next_get_max_energy_storage(&mut self)
pub fn read_last_get_max_energy_storage(&self) -> Result<f64, PeripheralError>
pub async fn async_get_max_energy_storage(&self) -> Result<f64, PeripheralError>
```

**Returns:** `number` — Maximum energy capacity

---

#### `getAvgPowerUsage()` / `book_next_get_avg_power_usage()` / `read_last_get_avg_power_usage()` / `async_get_avg_power_usage()`

Get the average power usage of the ME network.

**Rust Signatures:**
```rust
pub fn book_next_get_avg_power_usage(&mut self)
pub fn read_last_get_avg_power_usage(&self) -> Result<f64, PeripheralError>
pub async fn async_get_avg_power_usage(&self) -> Result<f64, PeripheralError>
```

**Returns:** `number` — Average power usage in AE/tick

---

#### `getAvgPowerInjection()` / `book_next_get_avg_power_injection()` / `read_last_get_avg_power_injection()` / `async_get_avg_power_injection()`

Get the average power injection into the ME network.

**Rust Signatures:**
```rust
pub fn book_next_get_avg_power_injection(&mut self)
pub fn read_last_get_avg_power_injection(&self) -> Result<f64, PeripheralError>
pub async fn async_get_avg_power_injection(&self) -> Result<f64, PeripheralError>
```

**Returns:** `number` — Average power injection in AE/tick

---

#### `getTotalItemStorage()` / `book_next_get_total_item_storage()` / `read_last_get_total_item_storage()` / `async_get_total_item_storage()`

Get the total item storage capacity.

**Rust Signatures:**
```rust
pub fn book_next_get_total_item_storage(&mut self)
pub fn read_last_get_total_item_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_total_item_storage(&self) -> Result<i64, PeripheralError>
```

**Returns:** `number` — Total item slots

---

#### `getUsedItemStorage()` / `book_next_get_used_item_storage()` / `read_last_get_used_item_storage()` / `async_get_used_item_storage()`

Get the used item storage.

**Rust Signatures:**
```rust
pub fn book_next_get_used_item_storage(&mut self)
pub fn read_last_get_used_item_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_used_item_storage(&self) -> Result<i64, PeripheralError>
```

**Returns:** `number` — Used item slots

---

#### `getAvailableItemStorage()` / `book_next_get_available_item_storage()` / `read_last_get_available_item_storage()` / `async_get_available_item_storage()`

Get the available item storage space.

**Rust Signatures:**
```rust
pub fn book_next_get_available_item_storage(&mut self)
pub fn read_last_get_available_item_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_available_item_storage(&self) -> Result<i64, PeripheralError>
```

**Returns:** `number` — Available item slots

---

#### `getTotalFluidStorage()` / `book_next_get_total_fluid_storage()` / `read_last_get_total_fluid_storage()` / `async_get_total_fluid_storage()`

Get the total fluid storage capacity in mB.

**Rust Signatures:**
```rust
pub fn book_next_get_total_fluid_storage(&mut self)
pub fn read_last_get_total_fluid_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_total_fluid_storage(&self) -> Result<i64, PeripheralError>
```

**Returns:** `number` — Total fluid capacity in mB

---

#### `getUsedFluidStorage()` / `book_next_get_used_fluid_storage()` / `read_last_get_used_fluid_storage()` / `async_get_used_fluid_storage()`

Get the used fluid storage.

**Rust Signatures:**
```rust
pub fn book_next_get_used_fluid_storage(&mut self)
pub fn read_last_get_used_fluid_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_used_fluid_storage(&self) -> Result<i64, PeripheralError>
```

**Returns:** `number` — Used fluid storage in mB

---

#### `getAvailableFluidStorage()` / `book_next_get_available_fluid_storage()` / `read_last_get_available_fluid_storage()` / `async_get_available_fluid_storage()`

Get the available fluid storage space.

**Rust Signatures:**
```rust
pub fn book_next_get_available_fluid_storage(&mut self)
pub fn read_last_get_available_fluid_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_available_fluid_storage(&self) -> Result<i64, PeripheralError>
```

**Returns:** `number` — Available fluid storage in mB

---

## Events

The MEBridge peripheral does not generate events.

---

## Usage Examples

### Example 1: List All Items and Fluids

```rust
// Rust example to be added
```
### Example 2: Export Items to Adjacent Inventory

```rust
// Rust example to be added
```
### Example 3: Monitor Energy Levels

```rust
// Rust example to be added
```
### Example 4: Craft Items on Demand

```rust
// Rust example to be added
```
### Example 5: Storage Monitoring

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **ME network not found**: MEBridge is not connected to an ME network
- **Peripheral disconnected**: The MEBridge is no longer accessible
- **Invalid filter**: Filter table is malformed
- **Craft not possible**: Item cannot be crafted (no recipe)
- **Insufficient storage**: Not enough space to import items

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### MEItemEntry
```rust
// Rust example to be added
```
### MEFluidEntry
```rust
// Rust example to be added
```
### ItemFilter
```rust
// Rust example to be added
```
---

## Notes

- All amounts are in items (stacks) for items and mB (millibuckets) for fluids
- Filters use partial matching for names and display names
- Crafting requests are asynchronous and may take time to complete
- The three-function pattern allows for efficient batch operations
- MEBridge requires an ME Controller and network to function

---

## Related

- [BlockReader](./BlockReader.md) — For reading block information
- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
