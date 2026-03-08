# CannonMount

**モジュール:** `control_craft::cannon_mount`  
**ペリフェラルタイプ:** `controlcraft:cannon_mount_peripheral`

Control-Craft の CannonMount ペリフェラル。キャノンのピッチ・ヨー方向の制御、およびコントラプションの組み立て・分解を行います。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_pitch` | `read_last_get_pitch` | `get_pitch_imm` | `f64` |
| `book_next_get_yaw` | `read_last_get_yaw` | `get_yaw_imm` | `f64` |

---

### セッター

#### `book_next_set_pitch` / `read_last_set_pitch`
キャノンのピッチ角度（度）を設定します。
```rust
pub fn book_next_set_pitch(&mut self, pitch: f64) { ... }
pub fn read_last_set_pitch(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `pitch: f64` — ピッチ角（度）

#### `book_next_set_yaw` / `read_last_set_yaw`
キャノンのヨー角度（度）を設定します。
```rust
pub fn book_next_set_yaw(&mut self, yaw: f64) { ... }
pub fn read_last_set_yaw(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `yaw: f64` — ヨー角（度）

---

### コントラプション制御

#### `book_next_assemble` / `read_last_assemble`
コントラプションを組み立てます（mainThread で実行）。
```rust
pub fn book_next_assemble(&mut self) { ... }
pub fn read_last_assemble(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_disassemble` / `read_last_disassemble`
コントラプションを分解します（mainThread で実行）。
```rust
pub fn book_next_disassemble(&mut self) { ... }
pub fn read_last_disassemble(&self) -> Result<(), PeripheralError> { ... }
```

## 使用例

```rust
use rust_computers_api::control_craft::cannon_mount::*;
use rust_computers_api::peripheral::Peripheral;

let mut mount = CannonMount::find().unwrap();

// キャノンの向きを設定
mount.book_next_set_pitch(-15.0);
mount.book_next_set_yaw(45.0);
wait_for_next_tick().await;
let _ = mount.read_last_set_pitch();
let _ = mount.read_last_set_yaw();

// 現在のピッチを即時取得
let pitch = mount.get_pitch_imm().unwrap();

// コントラプションを組み立て
mount.book_next_assemble();
wait_for_next_tick().await;
let _ = mount.read_last_assemble();
```
