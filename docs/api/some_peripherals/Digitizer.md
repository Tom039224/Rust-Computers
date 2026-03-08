# Digitizer

**Module:** `some_peripherals`
**Peripheral Type:** `sp:digitizer` (the NAME constant)

Some-Peripherals Digitizer peripheral. Converts physical items into digital UUIDs and back, with support for merging and splitting digital item stacks.

## Methods

### Async Methods

These methods are async and internally await a response from the game server. They use `do_action` internally (action-style async).

#### `digitize_amount`

Digitize the item in slot 0 and return a UUID representing the digital item.

```rust
pub async fn digitize_amount(
    &self,
    amount: Option<u32>,
) -> Result<String, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| amount | `Option<u32>` | Number of items to digitize (optional, defaults to all) |

#### `rematerialize_amount`

Rematerialize a digital item by UUID back into slot 0.

```rust
pub async fn rematerialize_amount(
    &self,
    uuid: &str,
    amount: Option<u32>,
) -> Result<bool, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| uuid | `&str` | UUID of the digital item |
| amount | `Option<u32>` | Number of items to rematerialize (optional, defaults to all) |

#### `merge_digital_items`

Merge two digital item stacks into one.

```rust
pub async fn merge_digital_items(
    &self,
    into_uuid: &str,
    from_uuid: &str,
    amount: Option<u32>,
) -> Result<bool, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| into_uuid | `&str` | UUID of the destination digital item |
| from_uuid | `&str` | UUID of the source digital item |
| amount | `Option<u32>` | Number of items to merge (optional, defaults to all) |

#### `separate_digital_item`

Split a digital item stack, creating a new UUID for the separated portion.

```rust
pub async fn separate_digital_item(
    &self,
    from_uuid: &str,
    amount: u32,
) -> Result<String, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| from_uuid | `&str` | UUID of the source digital item |
| amount | `u32` | Number of items to split off |

**Returns:** The new UUID for the separated item stack.

#### `check_id`

Check if a UUID exists and get its item data.

```rust
pub async fn check_id(&self, uuid: &str) -> Result<SPItemData, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| uuid | `&str` | UUID to look up |

#### `get_item_in_slot`

Get item information for the item currently in slot 0.

```rust
pub async fn get_item_in_slot(&self) -> Result<SPItemData, PeripheralError>
```

#### `get_item_limit_in_slot`

Get the maximum item count for slot 0.

```rust
pub async fn get_item_limit_in_slot(&self) -> Result<u32, PeripheralError>
```

## Example

```rust
// Digitize items from slot 0
let uuid = digitizer.digitize_amount(Some(16)).await.unwrap();

// Check the digital item
let item_data = digitizer.check_id(&uuid).await.unwrap();

// Split 8 items off into a new stack
let new_uuid = digitizer.separate_digital_item(&uuid, 8).await.unwrap();

// Rematerialize back into slot 0
digitizer.rematerialize_amount(&uuid, None).await.unwrap();
```

## Types

### SPItemData

Digitized item data.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPItemData {
    pub id: String,
    pub count: u32,
    pub tag: msgpack::Value,
}
```

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | Item registry name (e.g. `"minecraft:diamond"`) |
| count | `u32` | Item stack count |
| tag | `msgpack::Value` | NBT/tag data as a msgpack value (default: empty) |

### SPDigitizedItem

Digitized item reference (UUID wrapper).

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPDigitizedItem {
    pub uuid: String,
}
```

| Field | Type | Description |
|-------|------|-------------|
| uuid | `String` | The unique identifier for the digital item |
