# WatchdogTimer

**Module:** `toms_peripherals`  
**Peripheral Type:** `tm:watchdog_timer`

Toms-Peripherals Watchdog Timer peripheral. Emits a redstone signal if not reset within the configured timeout.

## Methods

### Async Methods (with imm variants)

#### `is_enabled` / `is_enabled_imm`

Check if the watchdog timer is enabled.

```rust
pub async fn is_enabled(&self) -> Result<bool, PeripheralError>
pub fn is_enabled_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_timeout` / `get_timeout_imm`

Get the timeout value in ticks.

```rust
pub async fn get_timeout(&self) -> Result<u32, PeripheralError>
pub fn get_timeout_imm(&self) -> Result<u32, PeripheralError>
```

### Async Action Methods

#### `set_enabled`

Enable or disable the watchdog timer.

```rust
pub async fn set_enabled(&self, enabled: bool) -> Result<(), PeripheralError>
```

#### `set_timeout`

Set the timeout in ticks.

```rust
pub async fn set_timeout(&self, ticks: u32) -> Result<(), PeripheralError>
```

#### `reset`

Reset the watchdog timer counter.

```rust
pub async fn reset(&self) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::toms_peripherals::watchdog_timer::WatchDogTimer;
use rust_computers_api::peripheral::Peripheral;

let wdt = WatchDogTimer::wrap(addr);

// Configure: 100 ticks timeout
wdt.set_timeout(100).await?;
wdt.set_enabled(true).await?;

// In main loop, periodically reset
loop {
    // ... do work ...
    wdt.reset().await?;
    wait_for_next_tick().await;
}
```
