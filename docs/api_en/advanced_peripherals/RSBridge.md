# RSBridge

**Module:** `advanced_peripherals::rs_bridge`  
**Peripheral Type:** `advancedPeripherals:rs_bridge`

AdvancedPeripherals RS Bridge peripheral. Provides full access to Refined Storage networks. Has nearly identical interface to MEBridge. Supports item, fluid, and chemical operations, crafting, and storage monitoring.

## Book-Read Methods

### Item Operations

#### `book_next_list_items` / `read_last_list_items`
List all items in the RS network.
```rust
pub fn book_next_list_items(&mut self)
pub fn read_last_list_items(&self) -> Result<Vec<MEItemEntry>, PeripheralError>
```
**Returns:** `Vec<MEItemEntry>`

---

#### `book_next_get_item` / `read_last_get_item`
Get the first item matching a filter.
```rust
pub fn book_next_get_item(&mut self, filter: &[u8])
pub fn read_last_get_item(&self) -> Result<MEItemEntry, PeripheralError>
```

---

#### `book_next_export_item` / `read_last_export_item`
Export an item to an adjacent inventory.
```rust
pub fn book_next_export_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_item(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_item` / `read_last_import_item`
Import an item from an adjacent inventory.
```rust
pub fn book_next_import_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_item(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_export_item_to_peripheral` / `read_last_export_item_to_peripheral`
Export an item to a named peripheral.
```rust
pub fn book_next_export_item_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_item_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_item_from_peripheral` / `read_last_import_item_from_peripheral`
Import an item from a named peripheral.
```rust
pub fn book_next_import_item_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_item_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### Fluid Operations

#### `book_next_list_fluids` / `read_last_list_fluids`
List all fluids in the RS network.
```rust
pub fn book_next_list_fluids(&mut self)
pub fn read_last_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
```

---

#### `book_next_get_fluid` / `read_last_get_fluid`
Get the first fluid matching a filter.
```rust
pub fn book_next_get_fluid(&mut self, filter: &[u8])
pub fn read_last_get_fluid(&self) -> Result<MEFluidEntry, PeripheralError>
```

---

#### `book_next_export_fluid` / `read_last_export_fluid`
Export fluid to an adjacent tank.
```rust
pub fn book_next_export_fluid(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_fluid(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_fluid` / `read_last_import_fluid`
Import fluid from an adjacent tank.
```rust
pub fn book_next_import_fluid(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_fluid(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_export_fluid_to_peripheral` / `read_last_export_fluid_to_peripheral`
Export fluid to a named peripheral.
```rust
pub fn book_next_export_fluid_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_fluid_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_fluid_from_peripheral` / `read_last_import_fluid_from_peripheral`
Import fluid from a named peripheral.
```rust
pub fn book_next_import_fluid_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_fluid_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### Chemical Operations

#### `book_next_list_chemicals` / `read_last_list_chemicals`
List all chemicals in the RS network.
```rust
pub fn book_next_list_chemicals(&mut self)
pub fn read_last_list_chemicals(&self) -> Result<Vec<MEChemicalEntry>, PeripheralError>
```

---

#### `book_next_get_chemical` / `read_last_get_chemical`
Get the first chemical matching a filter.
```rust
pub fn book_next_get_chemical(&mut self, filter: &[u8])
pub fn read_last_get_chemical(&self) -> Result<MEChemicalEntry, PeripheralError>
```

---

#### `book_next_export_chemical` / `read_last_export_chemical`
Export chemical.
```rust
pub fn book_next_export_chemical(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_chemical(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_chemical` / `read_last_import_chemical`
Import chemical.
```rust
pub fn book_next_import_chemical(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_chemical(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_export_chemical_to_peripheral` / `read_last_export_chemical_to_peripheral`
Export chemical to a named peripheral.
```rust
pub fn book_next_export_chemical_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_chemical_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_chemical_from_peripheral` / `read_last_import_chemical_from_peripheral`
Import chemical from a named peripheral.
```rust
pub fn book_next_import_chemical_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_chemical_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### Crafting

#### `book_next_craft_item` / `read_last_craft_item`
Request item crafting.
```rust
pub fn book_next_craft_item(&mut self, filter: &[u8])
pub fn read_last_craft_item(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_craft_fluid` / `read_last_craft_fluid`
Request fluid crafting.
```rust
pub fn book_next_craft_fluid(&mut self, filter: &[u8])
pub fn read_last_craft_fluid(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_craft_chemical` / `read_last_craft_chemical`
Request chemical crafting.
```rust
pub fn book_next_craft_chemical(&mut self, filter: &[u8])
pub fn read_last_craft_chemical(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_is_item_crafting` / `read_last_is_item_crafting`
Check if an item is currently being crafted.
```rust
pub fn book_next_is_item_crafting(&mut self, filter: &[u8])
pub fn read_last_is_item_crafting(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_is_fluid_crafting` / `read_last_is_fluid_crafting`
Check if a fluid is currently being crafted.
```rust
pub fn book_next_is_fluid_crafting(&mut self, filter: &[u8])
pub fn read_last_is_fluid_crafting(&self) -> Result<bool, PeripheralError>
```

---

### Storage & Energy

#### `book_next_get_energy_storage` / `read_last_get_energy_storage`
Get stored energy.
```rust
pub fn book_next_get_energy_storage(&mut self)
pub fn read_last_get_energy_storage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_max_energy_storage` / `read_last_get_max_energy_storage`
Get maximum energy capacity.
```rust
pub fn book_next_get_max_energy_storage(&mut self)
pub fn read_last_get_max_energy_storage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_avg_power_usage` / `read_last_get_avg_power_usage`
Get average power usage.
```rust
pub fn book_next_get_avg_power_usage(&mut self)
pub fn read_last_get_avg_power_usage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_avg_power_injection` / `read_last_get_avg_power_injection`
Get average power injection.
```rust
pub fn book_next_get_avg_power_injection(&mut self)
pub fn read_last_get_avg_power_injection(&self) -> Result<f64, PeripheralError>
```

---

#### Storage capacity methods
- `book_next_get_total_item_storage` / `read_last_get_total_item_storage` → `i64`
- `book_next_get_used_item_storage` / `read_last_get_used_item_storage` → `i64`
- `book_next_get_available_item_storage` / `read_last_get_available_item_storage` → `i64`
- `book_next_get_total_fluid_storage` / `read_last_get_total_fluid_storage` → `i64`
- `book_next_get_used_fluid_storage` / `read_last_get_used_fluid_storage` → `i64`
- `book_next_get_available_fluid_storage` / `read_last_get_available_fluid_storage` → `i64`
- `book_next_get_total_chemical_storage` / `read_last_get_total_chemical_storage` → `i64`
- `book_next_get_used_chemical_storage` / `read_last_get_used_chemical_storage` → `i64`
- `book_next_get_available_chemical_storage` / `read_last_get_available_chemical_storage` → `i64`

## Immediate Methods

None.

## Types

Types are re-exported from `me_bridge`:

```rust
pub use super::me_bridge::{MEItemEntry, MEFluidEntry, MEChemicalEntry};
```

See [MEBridge](MEBridge.md) for type definitions.

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::RSBridge;
use rust_computers_api::peripheral::Peripheral;

let mut rs = RSBridge::wrap(addr);

loop {
    let items = rs.read_last_list_items();
    let energy = rs.read_last_get_energy_storage();

    rs.book_next_list_items();
    rs.book_next_get_energy_storage();
    wait_for_next_tick().await;
}
```
