# InventoryManager

**Module:** `advanced_peripherals`  
**Peripheral Type:** `advancedPeripherals:inventory_manager`

Advanced Peripherals Inventory Manager. Manages a player's inventory including armor, items, and chest interactions.

## Types

```rust
pub struct ADItemEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub count: u32,
    pub display_name: String,
    pub max_stack_size: u32,
    pub components: Value,
    pub fingerprint: String,
    pub slot: Option<u32>,
}
```

## Methods

### Async Methods (with imm variant)

#### `get_owner` / `get_owner_imm`

Get the owner player name.

```rust
pub async fn get_owner(&self) -> Result<String, PeripheralError>
pub fn get_owner_imm(&self) -> Result<String, PeripheralError>
```

### Async Action Methods

#### `add_item_to_player`

Add an item from a slot to the player's inventory. Optionally specify count. Returns the number of items transferred.

```rust
pub async fn add_item_to_player(&self, slot: u32, count: Option<u32>) -> Result<u32, PeripheralError>
```

#### `remove_item_from_player`

Remove an item from the player's inventory slot. Optionally specify count. Returns the number of items removed.

```rust
pub async fn remove_item_from_player(&self, slot: u32, count: Option<u32>) -> Result<u32, PeripheralError>
```

### Async Read Methods

#### `list`

List all items in the player's inventory.

```rust
pub async fn list(&self) -> Result<Vec<ADItemEntry>, PeripheralError>
```

#### `get_armor`

List the player's equipped armor.

```rust
pub async fn get_armor(&self) -> Result<Vec<ADItemEntry>, PeripheralError>
```

#### `is_player_equipped`

Check if the player has anything equipped.

```rust
pub async fn is_player_equipped(&self) -> Result<bool, PeripheralError>
```

#### `is_wearing`

Check if the player is wearing armor in a specific slot.

```rust
pub async fn is_wearing(&self, slot: u32) -> Result<bool, PeripheralError>
```

#### `get_item_in_hand`

Get the item in the player's main hand.

```rust
pub async fn get_item_in_hand(&self) -> Result<ADItemEntry, PeripheralError>
```

#### `get_item_in_off_hand`

Get the item in the player's off hand.

```rust
pub async fn get_item_in_off_hand(&self) -> Result<ADItemEntry, PeripheralError>
```

#### `get_empty_space`

Get the number of empty inventory slots.

```rust
pub async fn get_empty_space(&self) -> Result<u32, PeripheralError>
```

#### `is_space_available`

Check if there is any empty space in the inventory.

```rust
pub async fn is_space_available(&self) -> Result<bool, PeripheralError>
```

#### `get_free_slot`

Get the first free inventory slot (-1 if none).

```rust
pub async fn get_free_slot(&self) -> Result<i32, PeripheralError>
```

#### `list_chest`

List items in the connected chest inventory.

```rust
pub async fn list_chest(&self) -> Result<Vec<ADItemEntry>, PeripheralError>
```

## Example

```rust
use rust_computers_api::advanced_peripherals::inventory_manager::InventoryManager;
use rust_computers_api::peripheral::Peripheral;

let inv = InventoryManager::wrap(addr);

// Get owner
let owner = inv.get_owner().await?;

// List inventory
let items = inv.list().await?;

// Check hand item
let hand = inv.get_item_in_hand().await?;

// Transfer item to player from slot 1
let transferred = inv.add_item_to_player(1, Some(16)).await?;
```
