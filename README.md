# RustComputers

Minecraft 1.20.1 Forge (47.4.10) Mod — CC:Tweaked の Lua VM を Rust/WASM に置き換えながら、CC の Java ペリフェラルエコシステムとの互換性を維持する。  
Replaces CC:Tweaked's Lua VM with Rust/WASM while maintaining full compatibility with CC's Java peripheral ecosystem.

**WASM Runtime**: [Chicory](https://github.com/dylibso/chicory) 1.7.2 (pure-Java — JNI 不要 / no JNI required)

---

## Documentation

- **[docs/spec.md](docs/spec.md)** — Full design specification (Japanese)
- **[docs/wasm-runtime-comparison.md](docs/wasm-runtime-comparison.md)** — WASM runtime selection rationale
- **[docs/java-binding-investigation.md](docs/java-binding-investigation.md)** — Java binding & Minecraft integration research

---

## Implementation Progress

| Phase | Task | Status |
|---|---|---|
| Planning/Spec | CC API Investigation | ✅ Done |
| | WASM Runtime Selection | ✅ Done (Chicory 1.7.2) |
| | Reference Mod Research | ✅ Done |
| | Java Binding Research | ✅ Done |
| v0.1 Core | Java↔WASM Bridge | ✅ Done |
| | Host Functions (log/stdin/info/action) | ✅ Done |
| | Computer Block & BlockEntity | ✅ Done |
| | WASM Loading from `config/rustcomputers/` | ✅ Done |
| | Server Tick Execution Loop | ✅ Done |
| | Immediate Request (`host_request_info_imm`) | ✅ Done |
| | `/rc` Command (`load`/`reload`/`stop`/`log`) | ✅ Done |
| v0.1.4 | Player log streaming (`/rc log follow`) | ✅ Done |
| | `getComputerId()` host function | ✅ Done |
| | `host_request_info` async peripheral call | ✅ Done |
| v0.1.5 | `isModAvailable()` host function | ✅ Done |
| | `VanillaRedstonePeripheral` | ✅ Done |
| | `VanillaInventoryPeripheral` | ✅ Done |
| | `MsgPack` encoder/decoder | ✅ Done |
| | `KnownMod` CRC32 lookup table | ✅ Done |
| | PeripheralProvider block registration | ✅ Done |
| Future | CC:Tweaked optional integration | ⏳ Not Started |
| | AdvancedPeripherals optional integration | ⏳ Not Started |
| | Tom's Peripherals optional integration | ⏳ Not Started |

---

## Host Functions

WASM モジュールが呼び出せるホスト関数 / Host functions callable from WASM modules:

| Function | Signature | Description |
|---|---|---|
| `host_log` | `(ptr, len)` | Write to server log and optional player stream |
| `host_stdin_read_line` | `(buf, bufLen) → i32` | Read a line from stdin buffer |
| `host_request_info` | `(dir, mid, argsPtr, argsLen) → i32` | Async peripheral request |
| `host_do_action` | `(dir, mid, argsPtr, argsLen) → i32` | Async peripheral action |
| `host_request_info_imm` | `(dir, mid, argsPtr, argsLen, resultPtr, resultLen) → i32` | Immediate peripheral call |
| `host_poll_result` | `(resultPtr, resultLen) → i32` | Poll pending async result |
| `host_is_mod_available` | `(modId) → i32` | Check if a Forge mod is loaded |
| `host_get_computer_id` | `() → i32` | Get this computer's entity ID |

---

## Peripheral API

### VanillaRedstonePeripheral

対応ブロック: Lever, Redstone Block, Redstone Torch, Redstone Wire, Redstone Lamp, Comparator, Repeater, Observer, ボタン各種、感圧板各種, Sculk Sensor, Daylight Detector, Lightning Rod, Tripwire Hook

| Method | Args | Return | Description |
|---|---|---|---|
| `getPower()` | — | int 0–15 | 6方向から受け取る最大レッドストーン信号強度 |
| `getDirectPower()` | — | int 0–15 | 最大直接信号強度 |
| `isActive()` | — | bool | 信号強度が 1 以上か |
| `toggle()` | — | bool | Lever を反転して新状態を返す（Lever のみ対応） |

### VanillaInventoryPeripheral

対応ブロック: Chest, Trapped Chest, Barrel, Hopper, Furnace, Blast Furnace, Smoker, Dropper, Dispenser, Brewing Stand, Shulker Box (全色)

| Method | Args | Return | Description |
|---|---|---|---|
| `getSize()` | — | int | スロット数 |
| `getItem(slot)` | int | map\|nil | スロットのアイテム情報 (`id`, `count`, `maxCount`) |
| `listItems()` | — | array | 空でないスロットの一覧 |
| `getItemCount(slot)` | int | int | スロットのアイテム数 |

---

## Mod Availability (`isModAvailable`)

Rust 側は `CRC32(mod_name.as_bytes()) as u16` を送信する。  
Java 側は起動時に既知 Mod の u16 キーテーブルを構築し、Forge `ModList` で照合する。

| Mod | Forge ID |
|---|---|
| CC:Tweaked | `computercraft` |
| Create | `create` |
| Valkyrien Skies 2 | `valkyrienskies` |
| Clockwork | `vs_clockwork` |
| CC-VS | `cc_vs` |
| VS-Addition | `vs_addition` |
| Some Peripherals | `some_peripherals` |
| Advanced Peripherals | `advancedperipherals` |
| Tom's Peripherals | `toms_peripherals` |
| Create Addition | `createaddition` |
| Clockwork CC Compat | `clockwork_cc_compat` |

---

## `/rc` Commands

| Command | Description |
|---|---|
| `/rc load <name>` | `config/rustcomputers/<name>.wasm` を読み込んで実行 |
| `/rc reload <name>` | 停止→再読み込み→実行 |
| `/rc stop <name>` | 実行中の WASM を停止 |
| `/rc log <name>` | 現在のログバッファを表示 |
| `/rc log follow <name>` | ログをリアルタイムストリーミング |
| `/rc log stop` | ログストリーミングを停止 |

---

## Reference Repositories

[refs/](./refs/) に以下のリポジトリをクローン済み / Cloned in [refs/](./refs/):

| Repo | Purpose |
|---|---|
| CC-Tweaked | Java peripheral API reference |
| CC-VS | CC × Valkyrien Skies integration example |
| Some-Peripherals | IPeripheral / GenericPeripheral patterns |
| Control-Craft | IPeripheral + Forge Capability pattern |
| AdvancedPeripherals | Advanced peripheral implementations |
| VS-Addition | VS-Addition peripheral examples |
| Toms-Peripherals | Tom's Peripherals implementations |
| createaddition | Create Addition peripheral patterns |
| Create | Create mod source (peripheral targets) |
| Clockwork_CC_Compat | Clockwork CC compatibility layer |
| wasmtime-java (kawamuray) | MavenCentral published Java binding |
| wasmtime-java (bluejekyll) | Alternative Java binding |
| fabric-wasmcraft-mod | Minecraft WASM integration example |
| chicory | Pure-Java WASM runtime (currently used) |

---

## Building

```bash
cd forge
./gradlew build
# → build/libs/rustcomputers-*.jar
```

WASM バイナリのビルド:

```bash
cd crates/rust-computers-api
cargo build --target wasm32-unknown-unknown --release
```
