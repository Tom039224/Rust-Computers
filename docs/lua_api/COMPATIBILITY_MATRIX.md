# CC:Tweaked Compatibility Matrix

## Overview

This document provides a comprehensive compatibility matrix for RustComputers with CC:Tweaked and compatible mods. It tracks the implementation status of peripherals and their methods.

## Legend

- ✅ **Fully Implemented**: All methods implemented with book_next/read_last/async patterns
- 🟡 **Partially Implemented**: Some methods implemented, missing event support or some APIs
- ❌ **Not Implemented**: Peripheral not yet implemented
- 📝 **Documented**: Lua API specification documented

## Compatibility Matrix

| Mod | Peripheral Count | Implementation Status | Documentation | Priority |
|-----|-----------------|----------------------|---------------|----------|
| CC:Tweaked | 4 | 🟡 Partial | ✅ Complete | High |
| AdvancedPeripherals | 12 | 🟡 Partial | ✅ Complete | High |
| Create | 18 | 🟡 Partial | ✅ Complete | Medium |
| Create Additions | 5 | 🟡 Partial | ✅ Complete | Medium |
| Control-Craft | 14 | ❌ Not Implemented | ✅ Complete | Medium |
| Clockwork CC Compat | 13 | ❌ Not Implemented | ✅ Complete | Low |
| CC-VS | 3 | 🟡 Minimal | ✅ Complete | Low |
| Some-Peripherals | 6 | 🟡 Partial | ✅ Complete | Medium |
| Toms-Peripherals | 4 | 🟡 Partial | ✅ Complete | Low |
| CBC CC Control | 1 | 🟡 Minimal | ✅ Complete | Low |
| VS-Addition | 0 | ❌ Not Implemented | ❌ None | Low |

## CC:Tweaked Core Peripherals

### Inventory

**Type**: `inventory`  
**Status**: 🟡 Partial  
**Missing**: Multiple action result handling

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `list` | ✅ | ✅ | ✅ | |
| `getItemDetail` | ✅ | ✅ | ✅ | |
| `getItemLimit` | ✅ | ✅ | ✅ | |
| `pushItems` | 🟡 | 🟡 | 🟡 | Single action only |
| `pullItems` | 🟡 | 🟡 | 🟡 | Single action only |
| `size` | ✅ | ✅ | ✅ | |

### Modem

**Type**: `modem`  
**Status**: 🟡 Partial  
**Missing**: Event support (receive, transmit events)

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `open` | ✅ | ✅ | ✅ | |
| `close` | ✅ | ✅ | ✅ | |
| `closeAll` | ✅ | ✅ | ✅ | |
| `isOpen` | ✅ | ✅ | ✅ | |
| `transmit` | ✅ | ✅ | ✅ | |
| `isWireless` | ✅ | ✅ | ✅ | |
| `receive` (event) | ❌ | ❌ | ❌ | Event not implemented |

### Monitor

**Type**: `monitor`  
**Status**: 🟡 Partial  
**Missing**: Touch event support

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `write` | ✅ | ✅ | ✅ | |
| `clear` | ✅ | ✅ | ✅ | |
| `setCursorPos` | ✅ | ✅ | ✅ | |
| `getCursorPos` | ✅ | ✅ | ✅ | |
| `getSize` | ✅ | ✅ | ✅ | |
| `setTextScale` | ✅ | ✅ | ✅ | |
| `setTextColor` | ✅ | ✅ | ✅ | |
| `setBackgroundColor` | ✅ | ✅ | ✅ | |
| `monitor_touch` (event) | ❌ | ❌ | ❌ | Event not implemented |

### Speaker

**Type**: `speaker`  
**Status**: 🟡 Partial  
**Missing**: Some audio methods

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `playNote` | ✅ | ✅ | ✅ | |
| `playSound` | ✅ | ✅ | ✅ | |
| `stop` | ✅ | ✅ | ✅ | |

## AdvancedPeripherals

### BlockReader

**Type**: `block_reader`  
**Status**: ✅ Fully Implemented

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `getBlockName` | ✅ | ✅ | ✅ | |
| `getBlockData` | ✅ | ✅ | ✅ | |
| `getBlockStates` | ✅ | ✅ | ✅ | |
| `isTileEntity` | ✅ | ✅ | ✅ | |

### ChatBox

**Type**: `chat_box`  
**Status**: 🟡 Partial  
**Missing**: Event support

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `sendMessage` | ✅ | ✅ | ✅ | |
| `sendMessageToPlayer` | ✅ | ✅ | ✅ | |
| `chat` (event) | ❌ | ❌ | ❌ | Event not implemented |

### MEBridge

**Type**: `me_bridge`  
**Status**: 🟡 Partial  
**Missing**: Some advanced methods

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `listItems` | ✅ | ✅ | ✅ | |
| `getItem` | ✅ | ✅ | ✅ | |
| `exportItem` | ✅ | ✅ | ✅ | |
| `importItem` | ✅ | ✅ | ✅ | |
| `listFluids` | ✅ | ✅ | ✅ | |
| `getFluid` | ✅ | ✅ | ✅ | |
| `craftItem` | 🟡 | 🟡 | 🟡 | Needs testing |
| `isItemCrafting` | 🟡 | 🟡 | 🟡 | Needs testing |

### PlayerDetector

**Type**: `player_detector`  
**Status**: 🟡 Partial  
**Missing**: Event support

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `getPlayers` | ✅ | ✅ | ✅ | |
| `getPlayerPos` | ✅ | ✅ | ✅ | |
| `isPlayerInRange` | ✅ | ✅ | ✅ | |
| `playerJoin` (event) | ❌ | ❌ | ❌ | Event not implemented |
| `playerLeave` (event) | ❌ | ❌ | ❌ | Event not implemented |

### GeoScanner

**Type**: `geo_scanner`  
**Status**: 🟡 Partial

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `scan` | ✅ | ✅ | ✅ | |
| `chunkAnalyze` | 🟡 | 🟡 | 🟡 | Needs testing |

### Other AdvancedPeripherals

- **ColonyIntegrator**: 🟡 Partial (basic methods implemented)
- **Compass**: ✅ Fully Implemented
- **EnergyDetector**: ✅ Fully Implemented
- **EnvironmentDetector**: ✅ Fully Implemented
- **InventoryManager**: 🟡 Partial (missing some methods)
- **NBTStorage**: ✅ Fully Implemented
- **RSBridge**: ✅ Fully Implemented

## Create

### DisplayLink

**Type**: `display_link`  
**Status**: 🟡 Partial

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `update` | ✅ | ✅ | ✅ | |
| `getTargetLine` | ✅ | ✅ | ✅ | |

### Station

**Type**: `station`  
**Status**: 🟡 Partial

| Method | book_next | read_last | async | Notes |
|--------|-----------|-----------|-------|-------|
| `assemble` | ✅ | ✅ | ✅ | |
| `disassemble` | ✅ | ✅ | ✅ | |
| `getStationName` | ✅ | ✅ | ✅ | |

### Other Create Peripherals

- **CreativeMotor**: ✅ Fully Implemented
- **Speedometer**: ✅ Fully Implemented
- **Stressometer**: ✅ Fully Implemented
- **RotationSpeedController**: ✅ Fully Implemented
- **SequencedGearshift**: 🟡 Partial
- **NixieTube**: ✅ Fully Implemented
- **Signal**: ✅ Fully Implemented
- **TrackObserver**: 🟡 Partial (event support missing)
- **Frogport**: 🟡 Partial
- **Packager**: 🟡 Partial
- **Postbox**: 🟡 Partial
- **RedstoneRequester**: ✅ Fully Implemented
- **Repackager**: 🟡 Partial
- **Sticker**: ✅ Fully Implemented
- **StockTicker**: 🟡 Partial
- **TableclothShop**: 🟡 Partial

## Create Additions

- **DigitalAdapter**: ✅ Fully Implemented
- **ElectricMotor**: ✅ Fully Implemented
- **ModularAccumulator**: ✅ Fully Implemented
- **PortableEnergyInterface**: 🟡 Partial
- **RedstoneRelay**: ✅ Fully Implemented

## Control-Craft

**Status**: ❌ Not Implemented (all peripherals)

Peripherals documented but not yet implemented:
- Camera
- CannonMount
- CompactFlap
- DynamicMotor
- FlapBearing
- Jet
- KinematicMotor
- KineticResistor
- LinkBridge
- PropellerController
- Slider
- SpatialAnchor
- Spinalyzer
- Transmitter

## Clockwork CC Compat

**Status**: ❌ Not Implemented (all peripherals)

Peripherals documented but not yet implemented:
- AirCompressor
- Boiler
- CoalBurner
- DuctTank
- Exhaust
- GasEngine
- GasNetwork
- GasNozzle
- GasPump
- GasThruster
- GasValve
- Radiator
- RedstoneDuct

## CC-VS

**Status**: 🟡 Minimal

- **Aerodynamics**: 🟡 Minimal
- **Drag**: 🟡 Minimal
- **Ship**: 🟡 Minimal

## Some-Peripherals

- **BallisticAccelerator**: 🟡 Partial
- **Digitizer**: 🟡 Partial
- **GoggleLinkPort**: 🟡 Partial
- **Radar**: 🟡 Partial
- **Raycaster**: 🟡 Partial
- **WorldScanner**: 🟡 Partial

## Toms-Peripherals

- **GPU**: 🟡 Partial
- **Keyboard**: 🟡 Partial (event support missing)
- **RedstonePort**: ✅ Fully Implemented
- **WatchDogTimer**: ✅ Fully Implemented

## CBC CC Control

- **CompactCannonMount**: 🟡 Minimal

## VS-Addition

**Status**: ❌ Not Implemented

No peripherals documented or implemented yet.

## Missing Features Summary

### High Priority

1. **Event System Implementation**
   - Modem: `receive`, `transmit` events
   - Monitor: `monitor_touch` event
   - ChatBox: `chat` event
   - PlayerDetector: `playerJoin`, `playerLeave` events
   - Keyboard: `key`, `key_up` events

2. **Multiple Action Results**
   - Inventory: `pushItems`, `pullItems` batch operations
   - All action methods need Vec<Result<T>> support

3. **book_next/read_last/async Pattern**
   - Ensure all implemented methods follow the three-function pattern
   - Add proper error handling for all methods

### Medium Priority

1. **Control-Craft Implementation**
   - All 14 peripherals need implementation
   - Focus on Camera, CannonMount, DynamicMotor first

2. **Advanced Create Features**
   - Complete partial implementations
   - Add missing methods to existing peripherals

3. **AdvancedPeripherals Completion**
   - Complete MEBridge crafting methods
   - Add missing InventoryManager methods

### Low Priority

1. **Clockwork CC Compat Implementation**
   - All 13 peripherals need implementation
   - Lower priority due to niche use case

2. **CC-VS Enhancement**
   - Expand minimal implementations
   - Add more ship control methods

3. **VS-Addition**
   - Research and document peripherals
   - Implement if peripherals are available

## Implementation Roadmap

### Phase 1: Core Event System (Weeks 1-2)
- Implement Java-side event monitoring framework
- Add Rust-side event booking/reading
- Implement Modem events
- Implement Monitor touch events

### Phase 2: Action Result Handling (Week 3)
- Extend request buffer for multiple actions
- Implement Vec<Result<T>> return types
- Update Inventory methods

### Phase 3: High-Priority Peripherals (Weeks 4-5)
- Complete CC:Tweaked core peripherals
- Complete AdvancedPeripherals event support
- Add missing ChatBox and PlayerDetector events

### Phase 4: Control-Craft (Weeks 6-7)
- Implement Camera, CannonMount, DynamicMotor
- Implement remaining Control-Craft peripherals
- Add comprehensive tests

### Phase 5: Polish & Documentation (Week 8)
- Complete all documentation
- Add usage examples
- Performance optimization
- Final testing

## Testing Coverage

| Category | Unit Tests | Integration Tests | Property Tests |
|----------|-----------|-------------------|----------------|
| CC:Tweaked Core | 🟡 Partial | ❌ None | ❌ None |
| AdvancedPeripherals | 🟡 Partial | ❌ None | ❌ None |
| Create | 🟡 Partial | ❌ None | ❌ None |
| Event System | ❌ None | ❌ None | ❌ None |
| Action Results | ❌ None | ❌ None | ❌ None |

## Notes

- All peripherals with ✅ status have been tested in Minecraft environment
- 🟡 Partial status indicates working implementation but missing features
- Event system is the highest priority missing feature
- Multiple action result handling is critical for batch operations
- Property-based testing should be added for all core functionality

---

**Last Updated**: 2025-01-XX  
**Version**: 1.0  
**Maintainer**: RustComputers Team
