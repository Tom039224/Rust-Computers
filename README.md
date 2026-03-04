# RustComputers

Minecraft 1.20.1 Forge (47.4.10) mod that replaces CC:Tweaked's Lua VM with Rust/WASM, while maintaining full compatibility with CC's Java peripheral ecosystem.

## Documentation

- **[docs/spec.md](docs/spec.md)** — Full design specification (Japanese)
- **[docs/wasm-runtime-comparison.md](docs/wasm-runtime-comparison.md)** — WASM runtime selection (Wasmtime chosen)
- **[docs/java-binding-investigation.md](docs/java-binding-investigation.md)** — Java binding & Minecraft integration research

## Reference Repositories

Cloned in [refs/](./refs/) folder:

- **CC-Tweaked** — Java peripheral API reference
- **CC-VS** — CC:Tweaked mod integration example
- **Some-Peripherals** — IPeripheral/GenericPeripheral patterns
- **Control-Craft** — IPeripheral + Forge Capability pattern
- **wasmtime-java** (kawamuray) — MavenCentral published Java binding
- **wasmtime-java** (bluejekyll) — Alternative Java binding
- **fabric-wasmcraft-mod** (HashiCraft) — Minecraft WASM integration example

## Implementation Progress

| Phase | Task | Status |
|---|---|---|
| Planning/Spec | CC API Investigation | ✅ Done |
| | WASM Runtime Selection | ✅ Done (Wasmtime) |
| | Reference Mod Research | ✅ Done |
| | Java Binding Research | ✅ Done |
| Preparation | Java↔WASM Bridge Design | 🔄 In Progress |
| | @LuaFunction Auto-generation Tool | ⏳ Not Started |
| Implementation | rust_computers Crate Skeleton | ⏳ Not Started |
| | Minecraft Mod Integration | ⏳ Not Started |

## Key Findings

1. **wasmtime-java is available on MavenCentral** (not just GitHub)
   - Prebuilt JNI libraries included in JAR
   - No need to build C/Rust JNI wrapper ourselves

2. **fabric-wasmcraft-mod provides excellent integration reference**
   - Complete Minecraft block integration pattern
   - Memory management, host function registration, module caching examples

3. **WasmFunctions.wrap() provides type-safe host function definition**
   - Lambda-based, supports up to 15 arguments
   - Compile-time type checking

See [docs/java-binding-investigation.md](docs/java-binding-investigation.md) for full technical details.
