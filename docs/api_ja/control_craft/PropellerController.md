# PropellerController

**モジュール:** `control_craft::propeller_controller`  
**ペリフェラルタイプ:** `controlcraft:propeller_controller_peripheral`

Control-Craft の PropellerController ペリフェラル。プロペラの目標速度の設定と読み取りを行います。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_target_speed` | `read_last_get_target_speed` | `get_target_speed_imm` | `f64` |

---

### セッター

#### `book_next_set_target_speed` / `read_last_set_target_speed`
プロペラの目標速度（RPM）を設定します。
```rust
pub fn book_next_set_target_speed(&mut self, speed: f64) { ... }
pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `speed: f64` — 目標速度（RPM）

## 使用例

```rust
use rust_computers_api::control_craft::propeller_controller::*;
use rust_computers_api::peripheral::Peripheral;

let mut prop = PropellerController::find().unwrap();

// 目標速度を設定
prop.book_next_set_target_speed(256.0);
wait_for_next_tick().await;
let _ = prop.read_last_set_target_speed();

// 現在の目標速度を即時取得
let speed = prop.get_target_speed_imm().unwrap();
```
