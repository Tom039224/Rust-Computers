# MEBridge

**Module:** `advanced_peripherals::me_bridge`  
**Peripheral Type:** `advancedPeripherals:me_bridge`

AdvancedPeripherals ME Bridge peripheral. Provides full access to Applied Energistics 2 ME networks including item, fluid, and chemical operations, crafting, and storage monitoring.

## Book-Read Methods

### Item Operations

#### `book_next_list_items` / `read_last_list_items`
List all items in the ME network.
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
**Parameters:**
- `filter: &[u8]` — MsgPack-encoded filter table

**Returns:** `MEItemEntry`

---

#### `book_next_export_item` / `read_last_export_item`
Export an item from ME to an adjacent inventory.
```rust
pub fn book_next_export_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_item(&self) -> Result<i64, PeripheralError>
```
**Parameters:**
- `filter: &[u8]` — MsgPack-encoded filter table
- `side: &str` — Direction (e.g. `"north"`, `"up"`)

**Returns:** `i64` — Number of items transferred

---

#### `book_next_import_item` / `read_last_import_item`
Import an item from an adjacent inventory into ME.
```rust
pub fn book_next_import_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_item(&self) -> Result<i64, PeripheralError>
```
**Parameters / Returns:** Same as `export_item`

---

#### `book_next_export_item_to_peripheral` / `read_last_export_item_to_peripheral`
Export an item from ME to a named peripheral.
```rust
pub fn book_next_export_item_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_item_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_item_from_peripheral` / `read_last_import_item_from_peripheral`
Import an item from a named peripheral into ME.
```rust
pub fn book_next_import_item_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_item_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### Fluid Operations

#### `book_next_list_fluids` / `read_last_list_fluids`
List all fluids in the ME network.
```rust
pub fn book_next_list_fluids(&mut self)
pub fn read_last_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
```
**Returns:** `Vec<MEFluidEntry>`

---

#### `book_next_get_fluid` / `read_last_get_fluid`
Get the first fluid matching a filter.
```rust
pub fn book_next_get_fluid(&mut self, filter: &[u8])
pub fn read_last_get_fluid(&self) -> Result<MEFluidEntry, PeripheralError>
```

---

#### `book_next_export_fluid` / `read_last_export_fluid`
Export fluid from ME to an adjacent tank.
```rust
pub fn book_next_export_fluid(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_fluid(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_fluid` / `read_last_import_fluid`
Import fluid from an adjacent tank into ME.
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
List all chemicals in the ME network.
```rust
pub fn book_next_list_chemicals(&mut self)
pub fn read_last_list_chemicals(&self) -> Result<Vec<MEChemicalEntry>, PeripheralError>
```
**Returns:** `Vec<MEChemicalEntry>`

---

#### `book_next_get_chemical` / `read_last_get_chemical`
Get the first chemical matching a filter.
```rust
pub fn book_next_get_chemical(&mut self, filter: &[u8])
pub fn read_last_get_chemical(&self) -> Result<MEChemicalEntry, PeripheralError>
```

---

#### `book_next_export_chemical` / `read_last_export_chemical`
Export chemical from ME.
```rust
pub fn book_next_export_chemical(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_chemical(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_chemical` / `read_last_import_chemical`
Import chemical into ME.
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
Request crafting of an item.
```rust
pub fn book_next_craft_item(&mut self, filter: &[u8])
pub fn read_last_craft_item(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

---

#### `book_next_craft_fluid` / `read_last_craft_fluid`
Request crafting of a fluid.
```rust
pub fn book_next_craft_fluid(&mut self, filter: &[u8])
pub fn read_last_craft_fluid(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_craft_chemical` / `read_last_craft_chemical`
Request crafting of a chemical.
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
Get the stored energy in the ME network.
```rust
pub fn book_next_get_energy_storage(&mut self)
pub fn read_last_get_energy_storage(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

#### `book_next_get_max_energy_storage` / `read_last_get_max_energy_storage`
Get the maximum energy capacity.
```rust
pub fn book_next_get_max_energy_storage(&mut self)
pub fn read_last_get_max_energy_storage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_avg_power_usage` / `read_last_get_avg_power_usage`
Get the average power usage.
```rust
pub fn book_next_get_avg_power_usage(&mut self)
pub fn read_last_get_avg_power_usage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_avg_power_injection` / `read_last_get_avg_power_injection`
Get the average power injection.
```rust
pub fn book_next_get_avg_power_injection(&mut self)
pub fn read_last_get_avg_power_injection(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_total_item_storage` / `read_last_get_total_item_storage`
Get total item storage capacity.
```rust
pub fn book_next_get_total_item_storage(&mut self)
pub fn read_last_get_total_item_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_used_item_storage` / `read_last_get_used_item_storage`
Get used item storage.
```rust
pub fn book_next_get_used_item_storage(&mut self)
pub fn read_last_get_used_item_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_available_item_storage` / `read_last_get_available_item_storage`
Get available item storage.
```rust
pub fn book_next_get_available_item_storage(&mut self)
pub fn read_last_get_available_item_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_total_fluid_storage` / `read_last_get_total_fluid_storage`
Get total fluid storage capacity.
```rust
pub fn book_next_get_total_fluid_storage(&mut self)
pub fn read_last_get_total_fluid_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_used_fluid_storage` / `read_last_get_used_fluid_storage`
Get used fluid storage.
```rust
pub fn book_next_get_used_fluid_storage(&mut self)
pub fn read_last_get_used_fluid_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_available_fluid_storage` / `read_last_get_available_fluid_storage`
Get available fluid storage.
```rust
pub fn book_next_get_available_fluid_storage(&mut self)
pub fn read_last_get_available_fluid_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_total_chemical_storage` / `read_last_get_total_chemical_storage`
Get total chemical storage capacity.
```rust
pub fn book_next_get_total_chemical_storage(&mut self)
pub fn read_last_get_total_chemical_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_used_chemical_storage` / `read_last_get_used_chemical_storage`
Get used chemical storage.
```rust
pub fn book_next_get_used_chemical_storage(&mut self)
pub fn read_last_get_used_chemical_storage(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_get_available_chemical_storage` / `read_last_get_available_chemical_storage`
Get available chemical storage.
```rust
pub fn book_next_get_available_chemical_storage(&mut self)
pub fn read_last_get_available_chemical_storage(&self) -> Result<i64, PeripheralError>
```

## Immediate Methods

None.

## Types

```rust
pub struct MEItemEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub count: i64,
    pub display_name: String,       // serde: "displayName"
    pub max_stack_size: Option<i32>, // serde: "maxStackSize"
    pub components: Value,
    pub fingerprint: String,
}

pub struct MEFluidEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub count: i64,
    pub display_name: String,       // serde: "displayName"
    pub fluid_type: Value,          // serde: "fluidType"
    pub components: Value,
    pub fingerprint: String,
}

pub struct MEChemicalEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub is_gaseous: Option<bool>,   // serde: "isGaseous"
    pub radioactivity: Option<f64>,
    pub count: i64,
    pub display_name: String,       // serde: "displayName"
    pub fingerprint: String,
}
```

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::MEBridge;
use rust_computers_api::peripheral::Peripheral;

let mut me = MEBridge::wrap(addr);

loop {
    let items = me.read_last_list_items();
    let energy = me.read_last_get_energy_storage();

    me.book_next_list_items();
    me.book_next_get_energy_storage();
    wait_for_next_tick().await;
}
```
