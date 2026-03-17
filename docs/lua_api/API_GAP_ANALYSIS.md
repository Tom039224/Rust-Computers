# API Gap Analysis

## Overview

This document identifies missing APIs, incomplete implementations, and gaps between CC:Tweaked Lua API and RustComputers implementation.

## Critical Gaps (High Priority)

### 1. Event System

**Impact**: High - Blocks many use cases  
**Effort**: Medium - Requires Java and Rust implementation

#### Missing Events

| Peripheral | Event | Status | Priority |
|------------|-------|--------|----------|
| Modem | `modem_message` | ❌ Not Implemented | Critical |
| Monitor | `monitor_touch` | ❌ Not Implemented | High |
| ChatBox | `chat` | ❌ Not Implemented | High |
| PlayerDetector | `playerJoin` | ❌ Not Implemented | Medium |
| PlayerDetector | `playerLeave` | ❌ Not Implemented | Medium |
| Keyboard | `key` | ❌ Not Implemented | Medium |
| Keyboard | `key_up` | ❌ Not Implemented | Medium |
| TrackObserver | `train_pass` | ❌ Not Implemented | Low |

#### Implementation Requirements

1. **Java Side**:
   - Event monitoring framework
   - Event queue management
   - Tick-based event polling
   - Event serialization to MessagePack

2. **Rust Side**:
   - `book_next_*_event()` methods
   - `read_last_*_event()` → `Vec<Option<Event>>`
   - `async_*_event()` → wait until event occurs
   - Event deserialization

3. **Example Implementation**:
```rust
// Modem event
pub fn book_next_receive(&mut self) {
    peripheral::book_event(self.addr, "modem_message");
}

pub fn read_last_receive(&self) -> Vec<Option<ModemMessage>> {
    peripheral::read_events(self.addr, "modem_message")
        .into_iter()
        .map(|opt| opt.and_then(|bytes| decode(&bytes).ok()))
        .collect()
}

pub async fn async_receive(&self) -> Result<ModemMessage, PeripheralError> {
    loop {
        self.book_next_receive();
        wait_for_next_tick().await;
        
        if let Some(msg) = self.read_last_receive()
            .into_iter()
            .next()
            .flatten()
        {
            return Ok(msg);
        }
    }
}
```

### 2. Multiple Action Results

**Impact**: High - Limits batch operations  
**Effort**: Medium - Requires buffer management changes

#### Affected Methods

| Peripheral | Method | Current | Required |
|------------|--------|---------|----------|
| Inventory | `pushItems` | Single result | `Vec<Result<u32>>` |
| Inventory | `pullItems` | Single result | `Vec<Result<u32>>` |
| MEBridge | `exportItem` | Single result | `Vec<Result<u32>>` |
| MEBridge | `importItem` | Single result | `Vec<Result<u32>>` |
| All | Action methods | Single result | `Vec<Result<T>>` |

#### Implementation Requirements

1. **Java Side**:
   - Separate action buffer from query buffer
   - Accumulate action results (not overwrite)
   - Return list of results

2. **Rust Side**:
   - `read_last_*()` returns `Vec<Result<T>>`
   - Handle multiple results in async methods

3. **Example**:
```rust
// Book multiple actions
inventory.book_next_push_items("minecraft:chest", 1, 64);
inventory.book_next_push_items("minecraft:chest", 2, 32);
inventory.book_next_push_items("minecraft:chest", 3, 16);
wait_for_next_tick().await;

// Read all results
let results = inventory.read_last_push_items()?;
// results: Vec<Result<u32, PeripheralError>>
// [Ok(64), Ok(32), Ok(16)]
```

### 3. book_next/read_last/async Pattern Consistency

**Impact**: Medium - API inconsistency  
**Effort**: Low - Refactoring existing code

#### Inconsistent Implementations

| Peripheral | Issue | Fix Required |
|------------|-------|--------------|
| Some methods | Missing `book_next_*` | Add booking method |
| Some methods | Missing `read_last_*` | Add reading method |
| Some methods | Missing `async_*` | Add async wrapper |
| Some methods | Direct FFI call | Refactor to use pattern |

#### Pattern Template

```rust
// Query method (information retrieval)
pub fn book_next_get_data(&mut self) {
    peripheral::book_request(self.addr, "getData", &[]);
}

pub fn read_last_get_data(&self) -> Result<Data, PeripheralError> {
    let bytes = peripheral::read_result(self.addr, "getData")?;
    decode::<Data>(&bytes)
}

pub async fn async_get_data(&self) -> Result<Data, PeripheralError> {
    self.book_next_get_data();
    wait_for_next_tick().await;
    self.read_last_get_data()
}

// Action method (world interaction)
pub fn book_next_set_value(&mut self, value: i32) {
    let args = encode(&value)?;
    peripheral::book_action(self.addr, "setValue", &args);
}

pub fn read_last_set_value(&self) -> Vec<Result<(), PeripheralError>> {
    peripheral::read_action_results(self.addr, "setValue")
}

pub async fn async_set_value(&mut self, value: i32) -> Result<(), PeripheralError> {
    self.book_next_set_value(value);
    wait_for_next_tick().await;
    self.read_last_set_value()
        .into_iter()
        .next()
        .unwrap_or(Ok(()))
}
```

## Major Gaps (Medium Priority)

### 4. Control-Craft Peripherals

**Impact**: Medium - Missing entire mod support  
**Effort**: High - 14 peripherals to implement

#### Missing Peripherals

1. **Camera** - Ship camera control
2. **CannonMount** - Weapon control
3. **CompactFlap** - Aerodynamic control
4. **DynamicMotor** - Motor control
5. **FlapBearing** - Bearing control
6. **Jet** - Jet engine control
7. **KinematicMotor** - Kinematic motor
8. **KineticResistor** - Resistor control
9. **LinkBridge** - Link management
10. **PropellerController** - Propeller control
11. **Slider** - Slider control
12. **SpatialAnchor** - Anchor management
13. **Spinalyzer** - Ship analysis
14. **Transmitter** - Signal transmission

#### Implementation Priority

1. **High**: Camera, CannonMount, DynamicMotor (most commonly used)
2. **Medium**: Jet, PropellerController, FlapBearing
3. **Low**: Others

### 5. Clockwork CC Compat Peripherals

**Impact**: Low - Niche mod  
**Effort**: High - 13 peripherals to implement

#### Missing Peripherals

All 13 peripherals are documented but not implemented:
- AirCompressor, Boiler, CoalBurner, DuctTank, Exhaust
- GasEngine, GasNetwork, GasNozzle, GasPump, GasThruster
- GasValve, Radiator, RedstoneDuct

#### Recommendation

Implement only if user demand exists. Focus on higher-priority mods first.

### 6. AdvancedPeripherals Incomplete Methods

**Impact**: Medium - Limits ME system automation  
**Effort**: Low - Extend existing implementations

#### Missing/Incomplete Methods

| Peripheral | Method | Status | Issue |
|------------|--------|--------|-------|
| MEBridge | `craftItem` | 🟡 Partial | Needs testing |
| MEBridge | `isItemCrafting` | 🟡 Partial | Needs testing |
| MEBridge | `craftFluid` | 🟡 Partial | Needs testing |
| MEBridge | `isFluidCrafting` | 🟡 Partial | Needs testing |
| InventoryManager | `addItemToPlayer` | ❌ Missing | Not implemented |
| InventoryManager | `removeItemFromPlayer` | ❌ Missing | Not implemented |
| GeoScanner | `chunkAnalyze` | 🟡 Partial | Needs testing |

## Minor Gaps (Low Priority)

### 7. CC-VS Enhancement

**Impact**: Low - Minimal implementation exists  
**Effort**: Medium

Current status: Minimal implementations for Aerodynamics, Drag, Ship peripherals.

#### Missing Features

- Advanced ship control methods
- Physics calculation methods
- Ship state monitoring

### 8. Some-Peripherals Completion

**Impact**: Low - Partial implementations exist  
**Effort**: Low

#### Incomplete Peripherals

- BallisticAccelerator: Missing some methods
- Digitizer: Missing some methods
- GoggleLinkPort: Missing some methods
- Radar: Missing some methods
- Raycaster: Missing some methods
- WorldScanner: Missing some methods

### 9. VS-Addition

**Impact**: Low - Mod not yet researched  
**Effort**: Unknown

Status: No peripherals documented or implemented.

Action: Research mod to determine if peripherals exist.

## API Design Gaps

### 10. Error Handling Consistency

**Impact**: Medium - User experience  
**Effort**: Low

#### Issues

1. Inconsistent error types across peripherals
2. Missing error context in some methods
3. No error recovery guidance

#### Recommendations

1. Standardize error types:
```rust
pub enum PeripheralError {
    NotFound,
    MethodNotFound,
    InvalidArgument(String),
    ExecutionFailed(String),
    Disconnected,
    Timeout,
    SerializationError(String),
}
```

2. Add error context:
```rust
Err(PeripheralError::ExecutionFailed(
    format!("Failed to push items: {}", reason)
))
```

3. Document error recovery patterns

### 11. Documentation Gaps

**Impact**: Low - User experience  
**Effort**: Low

#### Missing Documentation

1. **Usage Examples**: Many methods lack examples
2. **Error Cases**: Error conditions not documented
3. **Performance Notes**: No guidance on performance implications
4. **Best Practices**: Missing best practice guides

#### Recommendations

1. Add examples to all public methods
2. Document all error cases
3. Add performance notes for expensive operations
4. Create best practices guide

### 12. Testing Gaps

**Impact**: Medium - Code quality  
**Effort**: Medium

#### Missing Tests

| Category | Unit Tests | Integration Tests | Property Tests |
|----------|-----------|-------------------|----------------|
| Event System | ❌ None | ❌ None | ❌ None |
| Action Results | ❌ None | ❌ None | ❌ None |
| CC:Tweaked Core | 🟡 Partial | ❌ None | ❌ None |
| AdvancedPeripherals | 🟡 Partial | ❌ None | ❌ None |
| Create | 🟡 Partial | ❌ None | ❌ None |

#### Required Tests

1. **Unit Tests**: All peripheral methods
2. **Integration Tests**: Multi-peripheral scenarios
3. **Property Tests**: 
   - Request ordering
   - Action accumulation
   - Event polling termination
   - Tick boundary consistency

## Implementation Roadmap

### Phase 1: Critical Gaps (Weeks 1-3)

1. **Week 1-2**: Event System
   - Java event monitoring framework
   - Rust event booking/reading
   - Modem events
   - Monitor touch events

2. **Week 3**: Multiple Action Results
   - Extend request buffer
   - Update Inventory methods
   - Update MEBridge methods

### Phase 2: Major Gaps (Weeks 4-6)

1. **Week 4**: Pattern Consistency
   - Audit all peripherals
   - Refactor inconsistent methods
   - Add missing book_next/read_last/async

2. **Week 5-6**: Control-Craft
   - Implement Camera, CannonMount, DynamicMotor
   - Implement Jet, PropellerController
   - Implement remaining peripherals

### Phase 3: Minor Gaps (Weeks 7-8)

1. **Week 7**: AdvancedPeripherals Completion
   - Complete MEBridge methods
   - Add InventoryManager methods
   - Test GeoScanner

2. **Week 8**: Polish
   - Error handling consistency
   - Documentation completion
   - Testing coverage

## Success Metrics

### Coverage Targets

- **Event Support**: 100% of documented events
- **Action Results**: 100% of action methods
- **Pattern Consistency**: 100% of methods follow pattern
- **Control-Craft**: 80% of peripherals implemented
- **Test Coverage**: 80% code coverage
- **Documentation**: 100% of public APIs documented

### Quality Targets

- **Error Handling**: All errors have context
- **Performance**: <1ms FFI overhead per call
- **Reliability**: <0.1% failure rate in tests
- **Usability**: All common use cases have examples

## Conclusion

The most critical gaps are:

1. **Event System** - Blocks many use cases
2. **Multiple Action Results** - Limits batch operations
3. **Pattern Consistency** - API usability

Addressing these three gaps will significantly improve RustComputers' CC:Tweaked compatibility and user experience.

---

**Last Updated**: 2025-01-XX  
**Version**: 1.0  
**Maintainer**: RustComputers Team
