# CompactFlap

**モジュール:** `control_craft::compact_flap`  
**ペリフェラルタイプ:** `controlcraft:compact_flap_peripheral`

Control-Craft の CompactFlap ペリフェラル。コンパクトフラップの角度とチルトを制御します。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |
| `book_next_get_tilt` | `read_last_get_tilt` | `get_tilt_imm` | `f64` |

---

### セッター

#### `book_next_set_angle` / `read_last_set_angle`
フラップ角度（度）を設定します。
```rust
pub fn book_next_set_angle(&mut self, angle: f64) { ... }
pub fn read_last_set_angle(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `angle: f64` — フラップ角度（度）

#### `book_next_set_tilt` / `read_last_set_tilt`
フラップのチルト角度（度）を設定します。
```rust
pub fn book_next_set_tilt(&mut self, tilt: f64) { ... }
pub fn read_last_set_tilt(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `tilt: f64` — チルト角度（度）

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 使用例

```rust
use rust_computers_api::control_craft::compact_flap::*;
use rust_computers_api::peripheral::Peripheral;

let mut flap = CompactFlap::find().unwrap();

// フラップ角度とチルトを設定
flap.book_next_set_angle(30.0);
flap.book_next_set_tilt(10.0);
wait_for_next_tick().await;
let _ = flap.read_last_set_angle();
let _ = flap.read_last_set_tilt();

// 現在の角度を即時取得
let angle = flap.get_angle_imm().unwrap();
```
