# Chunky Turtle

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `chunky` (turtle upgrade)  
**Source:** `ChunkyPeripheral.java`, `TurtleChunkyUpgrade.java`

## Overview

The Chunky Turtle upgrade is a turtle equipment item that keeps the chunk the turtle is in loaded at all times. It does **not** expose any Lua-callable methods — it operates automatically when equipped.

## Behavior

- When the turtle is equipped with the Chunky upgrade, the chunk containing the turtle is force-loaded by the server.
- The chunk remains loaded as long as the turtle is active and the upgrade is equipped.
- If the turtle moves to a different chunk, the previously loaded chunk is unloaded and the new chunk is loaded.
- The chunk loading state is persisted via a UUID stored in the turtle's data components, so it survives server restarts.

## Rust Implementation

No Rust struct is needed for this peripheral. The Chunky upgrade has no Lua API surface — there are no methods to call from a CC:Tweaked program.

## Configuration

Chunk loading can be enabled or disabled via the AdvancedPeripherals config:

```
APConfig.PERIPHERALS_CONFIG.enableChunkyTurtle
```

## Notes

- This is a **turtle-only** upgrade; it cannot be used as a block peripheral.
- The chunk loading is managed by `ChunkManager`, which tracks loaded chunks by UUID.
- Chunks are automatically unloaded if the turtle has not been active for a configurable period.
- Unlike `level.setChunkForced()`, the `ChunkManager` approach is safer and avoids chunk leaks.

## Related

- [AdvancedPeripherals Documentation](https://docs.srendi.de/) — Official documentation
