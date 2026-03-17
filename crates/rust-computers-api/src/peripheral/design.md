# Rust Peripheral Wrapper Framework Design

## Overview

This document outlines the design for the Rust-side peripheral wrapper framework that provides a user-friendly async interface to Minecraft peripherals.

## Core Principles

1. **Type Safety**: Strongly typed interfaces for all peripheral methods
2. **Async-First**: All operations are async by default
3. **Zero-Cost Abstractions**: Minimal runtime overhead
4. **Ergonomic API**: Natural Rust patterns and idioms
5. **Error Handling**: Comprehensive error types and recovery

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Code                                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Peripheral Wrappers                         │   │
│  │  - Inventory, Modem, Monitor, Speaker, etc.         │   │
│  │  - Type-safe methods with proper return types       │   │
│  │  - Async/await interface                            │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Core Peripheral Traits                      │   │
│  │  - Peripheral trait (common interface)              │   │
│  │  - BookRead trait (book-read pattern)               │   │
│  │  - Immediate trait (immediate methods)              │   │
│  │  - Event trait (event handling)                     │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         FFI Layer                                   │   │
│  │  - book_request(), read_result()                    │   │
│  │  - book_event(), read_events()                      │   │
│  │  - MessagePack encoding/decoding                    │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            ↑
                            │ FFI (MessagePack)
                            │
┌─────────────────────────────────────────────────────────────┐
│                    Java Bridge Layer                        │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         PeripheralProvider                          │   │
│  │  - Peripheral discovery & registration             │   │
│  │  - Method invocation                                │   │
│  │  - Event monitoring                                 │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Core Traits

### 1. Peripheral Trait

```rust
/// Base trait for all peripherals
pub trait Peripheral {
    /// Get the peripheral address
    fn addr(&self) -> PeriphAddr;
    
    /// Get the peripheral type name
    fn type_name(&self) -> &'static str;
    
    /// Wrap a peripheral address into a typed wrapper
    fn wrap(addr: PeriphAddr) -> Self;
}
```

### 2. BookRead Trait

```rust
/// Trait for peripherals that support the book-read pattern
pub trait BookRead: Peripheral {
    /// Book a request (non-blocking)
    fn book_next(&mut self, method: &str, args: &[u8]);
    
    /// Read the result from the previous tick
    fn read_last(&self, method: &str) -> Result<Vec<u8>, PeripheralError>;
    
    /// Async method that books, waits, and reads
    async fn call(&mut self, method: &str, args: &[u8]) -> Result<Vec<u8>, PeripheralError> {
        self.book_next(method, args);
        wait_for_next_tick().await;
        self.read_last(method)
    }
}
```

### 3. Immediate Trait

```rust
/// Trait for peripherals that support immediate methods
pub trait Immediate: Peripheral {
    /// Call an immediate method (blocking)
    fn call_imm(&self, method: &str, args: &[u8]) -> Result<Vec<u8>, PeripheralError>;
}
```

### 4. Event Trait

```rust
/// Trait for peripherals that support events
pub trait Event: Peripheral {
    /// Book an event listener
    fn book_event(&mut self, event: &str);
    
    /// Read events from the previous tick
    fn read_events(&self, event: &str) -> Vec<Option<Vec<u8>>>;
    
    /// Async method that waits for an event
    async fn wait_event(&mut self, event: &str) -> Result<Vec<u8>, PeripheralError> {
        loop {
            self.book_event(event);
            wait_for_next_tick().await;
            
            if let Some(event_data) = self.read_events(event)
                .into_iter()
                .next()
                .flatten()
            {
                return Ok(event_data);
            }
        }
    }
}
```

## Peripheral Wrapper Structure

Each peripheral type will have a dedicated wrapper struct:

```rust
pub struct Inventory {
    addr: PeriphAddr,
}

impl Peripheral for Inventory {
    fn addr(&self) -> PeriphAddr { self.addr }
    fn type_name(&self) -> &'static str { "inventory" }
    fn wrap(addr: PeriphAddr) -> Self { Self { addr } }
}

impl BookRead for Inventory {
    fn book_next(&mut self, method: &str, args: &[u8]) {
        peripheral::book_request(self.addr, method, args);
    }
    
    fn read_last(&self, method: &str) -> Result<Vec<u8>, PeripheralError> {
        peripheral::read_result(self.addr, method)
    }
}

impl Inventory {
    // Type-safe methods
    pub async fn size(&mut self) -> Result<u32, PeripheralError> {
        let bytes = self.call("size", &[]).await?;
        decode::<u32>(&bytes)
    }
    
    pub async fn list(&mut self) -> Result<HashMap<u32, Item>, PeripheralError> {
        let bytes = self.call("list", &[]).await?;
        decode::<HashMap<u32, Item>>(&bytes)
    }
    
    pub async fn get_item_detail(&mut self, slot: u32) -> Result<Option<ItemDetail>, PeripheralError> {
        let args = encode_args(slot)?;
        let bytes = self.call("getItemDetail", &args).await?;
        decode::<Option<ItemDetail>>(&bytes)
    }
    
    // Immediate methods
    pub fn size_imm(&self) -> Result<u32, PeripheralError> {
        let bytes = peripheral::call_imm(self.addr, "size", &[])?;
        decode::<u32>(&bytes)
    }
}
```

## MessagePack Encoding/Decoding

### Encoding Strategy

```rust
/// Encode arguments for a peripheral call
pub fn encode_args<T: Serialize>(args: T) -> Result<Vec<u8>, PeripheralError> {
    let mut buf = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(&mut buf);
    args.serialize(&mut serializer)?;
    Ok(buf)
}

/// Decode result from a peripheral call
pub fn decode_result<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PeripheralError> {
    let mut deserializer = rmp_serde::Deserializer::new(bytes);
    Ok(T::deserialize(&mut deserializer)?)
}
```

### Type Mapping

| Lua Type | Rust Type | Notes |
|----------|-----------|-------|
| `nil` | `Option<T>` or `()` | Use `Option` for nullable values |
| `boolean` | `bool` | |
| `number` | `f64` or `i32`/`u32` | Use appropriate numeric type |
| `string` | `String` | |
| `table` (array) | `Vec<T>` | |
| `table` (map) | `HashMap<K, V>` | |
| `table` (mixed) | `Value` (serde_json) | For complex nested structures |

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum PeripheralError {
    #[error("Peripheral not found")]
    NotFound,
    
    #[error("Method not found: {0}")]
    MethodNotFound(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Encoding error: {0}")]
    Encoding(#[from] rmp_serde::encode::Error),
    
    #[error("Decoding error: {0}")]
    Decoding(#[from] rmp_serde::decode::Error),
    
    #[error("FFI error: {0}")]
    Ffi(String),
}
```

## Async Pattern Implementation

```rust
/// Wait for the next Minecraft tick
pub async fn wait_for_next_tick() {
    // Implementation uses FFI to wait for tick boundary
    unsafe {
        ffi::wait_for_next_tick();
    }
}

/// Macro for generating async methods
macro_rules! async_method {
    ($name:ident, $method:expr, $args:expr, $ret:ty) => {
        pub async fn $name(&mut self) -> Result<$ret, PeripheralError> {
            let args = encode_args($args)?;
            let bytes = self.call($method, &args).await?;
            decode_result(&bytes)
        }
    };
}

/// Macro for generating immediate methods
macro_rules! imm_method {
    ($name:ident, $method:expr, $args:expr, $ret:ty) => {
        pub fn $name(&self) -> Result<$ret, PeripheralError> {
            let args = encode_args($args)?;
            let bytes = peripheral::call_imm(self.addr, $method, &args)?;
            decode_result(&bytes)
        }
    };
}
```

## Code Generation Strategy

We'll use a combination of:

1. **Macros**: For common patterns (book-read, immediate, event)
2. **Manual Implementation**: For complex peripherals with special logic
3. **Build Scripts**: For generating boilerplate from API specifications

### Example Build Script

```rust
// build.rs
fn main() {
    // Read API specifications
    let specs = read_api_specs();
    
    // Generate Rust code for each peripheral
    for spec in specs {
        generate_peripheral_code(&spec);
    }
}
```

## Directory Structure

```
crates/rust-computers-api/
├── src/
│   ├── lib.rs
│   ├── peripheral/
│   │   ├── mod.rs          # Core traits and types
│   │   ├── ffi.rs          # FFI bindings
│   │   ├── error.rs        # Error types
│   │   ├── encode.rs       # MessagePack encoding
│   │   └── decode.rs       # MessagePack decoding
│   ├── computercraft/
│   │   ├── mod.rs
│   │   ├── inventory.rs    # Inventory wrapper
│   │   ├── modem.rs        # Modem wrapper
│   │   ├── monitor.rs      # Monitor wrapper
│   │   └── speaker.rs      # Speaker wrapper
│   ├── advanced_peripherals/
│   │   ├── mod.rs
│   │   ├── geo_scanner.rs  # GeoScanner wrapper
│   │   └── ...
│   ├── create/
│   │   ├── mod.rs
│   │   ├── display_link.rs # DisplayLink wrapper
│   │   ├── station.rs      # Station wrapper
│   │   └── ...
│   └── macros/
│       ├── mod.rs
│       ├── book_read.rs    # Book-read pattern macro
│       └── peripheral.rs   # Peripheral wrapper macro
└── build.rs                # Code generation script
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inventory_size() {
        // Mock FFI for testing
        let inventory = Inventory::wrap(PeriphAddr::from_raw(0));
        // Test immediate method
        assert!(inventory.size_imm().is_ok());
    }
    
    #[tokio::test]
    async fn test_inventory_list_async() {
        let mut inventory = Inventory::wrap(PeriphAddr::from_raw(0));
        // Test async method
        let result = inventory.list().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration {
    // Tests that require actual Minecraft/Java bridge
    // These would run in a test environment with the mod loaded
}
```

## Performance Considerations

1. **MessagePack Overhead**: Use efficient encoding strategies
2. **FFI Calls**: Batch requests when possible
3. **Memory Allocation**: Reuse buffers for encoding/decoding
4. **Async Runtime**: Use lightweight async runtime (tokio or async-std)

## Next Steps

1. Implement core traits and FFI layer
2. Create code generation utilities
3. Implement first peripheral (Inventory) as reference
4. Add comprehensive error handling
5. Write documentation and examples
6. Create test suite

## References

- [CC:Tweaked API Documentation](https://tweaked.cc/)
- [MessagePack Specification](https://msgpack.org/)
- [Serde Documentation](https://serde.rs/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
```
