# CompactCannonMount

**モジュール:** `cbc_cc_control::compact_cannon_mount`  
**ペリフェラルタイプ:** `cbc_cannon_mount`

Create Big Cannons (CBC) のキャノンマウントペリフェラル。
ピッチ・ヨー方向の制御、コントラプションの組み立て・分解、発射を行います。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドはすべて `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_is_running` | `read_last_is_running` | `is_running_imm` | `bool` |
| `book_next_get_yaw` | `read_last_get_yaw` | `get_yaw_imm` | `f64` |
| `book_next_get_pitch` | `read_last_get_pitch` | `get_pitch_imm` | `f64` |

---

## アクションメソッド

### `book_next_assemble` / `read_last_assemble`
コントラプションを組み立てます。内部的に `tryUpdatingSpeed()`、`assemble()`、`sendData()` を呼び出し、最後に `isRunning()` の結果を返します。mainThread で実行されます。
```rust
pub fn book_next_assemble(&mut self) { ... }
pub fn read_last_assemble(&self) -> Vec<Result<bool, PeripheralError>> { ... }
```
**戻り値:** `bool` — 組み立て後にコントラプションが稼働中であれば `true`。

---

### `book_next_disassemble` / `read_last_disassemble`
コントラプションを分解します。mainThread で実行されます。
```rust
pub fn book_next_disassemble(&mut self) { ... }
pub fn read_last_disassemble(&self) -> Vec<Result<(), PeripheralError>> { ... }
```

---

### `book_next_set_yaw` / `read_last_set_yaw`
キャノンのヨー角度（度）を設定します。mainThread で実行されます。
```rust
pub fn book_next_set_yaw(&mut self, yaw: f64) { ... }
pub fn read_last_set_yaw(&self) -> Vec<Result<(), PeripheralError>> { ... }
```
**パラメータ:** `yaw: f64` — ヨー角（度）

---

### `book_next_set_pitch` / `read_last_set_pitch`
キャノンのピッチ角度（度）を設定します。mainThread で実行されます。
```rust
pub fn book_next_set_pitch(&mut self, pitch: f64) { ... }
pub fn read_last_set_pitch(&self) -> Vec<Result<(), PeripheralError>> { ... }
```
**パラメータ:** `pitch: f64` — ピッチ角（度）

---

### `book_next_fire` / `read_last_fire`
キャノンを発射します（内部的に `onRedstoneUpdate()` を呼び出します）。
```rust
pub fn book_next_fire(&mut self) { ... }
pub fn read_last_fire(&self) -> Vec<Result<(), PeripheralError>> { ... }
```

---

## 使用例

```rust
use rust_computers_api::cbc_cc_control::compact_cannon_mount::CompactCannonMount;
use rust_computers_api::peripheral::Peripheral;
use rust_computers_api::wait_for_next_tick;

let mut cannon = CompactCannonMount::find().unwrap();

// 現在の向きを即時取得
let yaw = cannon.get_yaw_imm().unwrap();
let pitch = cannon.get_pitch_imm().unwrap();

// 照準を合わせて発射
cannon.book_next_set_yaw(90.0);
cannon.book_next_set_pitch(-10.0);
wait_for_next_tick().await;
let _ = cannon.read_last_set_yaw();
let _ = cannon.read_last_set_pitch();

cannon.book_next_fire();
wait_for_next_tick().await;
let _ = cannon.read_last_fire();

// コントラプションを組み立てて稼働確認
cannon.book_next_assemble();
wait_for_next_tick().await;
let results = cannon.read_last_assemble();
let is_running = results.into_iter().next().unwrap().unwrap();
```
