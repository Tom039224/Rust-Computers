# GoggleLinkPort

**Module:** `some_peripherals`
**Peripheral Type:** `sp:goggle_link_port` (the NAME constant)

Some-Peripherals GoggleLinkPort peripheral. Provides access to connected goggle devices.

## Methods

### Async Methods

These methods are async and internally await a response from the game server.

#### `get_connected`

Get a list of currently connected goggles and their information.

```rust
pub async fn get_connected(
    &self,
) -> Result<BTreeMap<String, msgpack::Value>, PeripheralError>
```

**Returns:** A map of connected goggle identifiers to their data (as msgpack values).

## Example

```rust
let connected = goggle_link_port.get_connected().await.unwrap();
for (id, data) in &connected {
    // Process connected goggle data
}
```
