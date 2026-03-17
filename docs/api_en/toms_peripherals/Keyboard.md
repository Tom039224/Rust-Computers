# Keyboard

**Module:** `toms_peripherals`  
**Peripheral Type:** `tm:keyboard`

Toms-Peripherals Keyboard peripheral for handling keyboard input events.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### Async Action Methods

#### `set_fire_native_events`

Enable or disable native event firing.

```rust
pub async fn set_fire_native_events(&self, enabled: bool) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::toms_peripherals::keyboard::Keyboard;
use rust_computers_api::peripheral::Peripheral;

let keyboard = Keyboard::wrap(addr);

// Enable native keyboard events
keyboard.set_fire_native_events(true).await?;
```
