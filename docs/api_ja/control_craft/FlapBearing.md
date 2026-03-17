# FlapBearing

**モジュール:** `control_craft::flap_bearing`  
**ペリフェラルタイプ:** `controlcraft:flap_bearing_peripheral`

Control-Craft の FlapBearing ペリフェラル。翼の角度制御およびコントラプションの組み立て・分解を行います。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |

---

### セッター

#### `book_next_set_angle` / `read_last_set_angle`
翼角度（度）を設定します。
```rust
pub fn book_next_set_angle(&mut self, angle: f64) { ... }
pub fn read_last_set_angle(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `angle: f64` — 翼角度（度）

---

### コントラプション制御

#### `book_next_assemble_next_tick` / `read_last_assemble_next_tick`
次のティックにコントラプションを組み立てます。
```rust
pub fn book_next_assemble_next_tick(&mut self) { ... }
pub fn read_last_assemble_next_tick(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_disassemble_next_tick` / `read_last_disassemble_next_tick`
次のティックにコントラプションを分解します。
```rust
pub fn book_next_disassemble_next_tick(&mut self) { ... }
pub fn read_last_disassemble_next_tick(&self) -> Result<(), PeripheralError> { ... }
```

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 使用例

```rust
use rust_computers_api::control_craft::flap_bearing::*;
use rust_computers_api::peripheral::Peripheral;

let mut bearing = FlapBearing::find().unwrap();

// コントラプションを組み立て
bearing.book_next_assemble_next_tick();
wait_for_next_tick().await;
let _ = bearing.read_last_assemble_next_tick();

// 翼角度を設定
bearing.book_next_set_angle(20.0);
wait_for_next_tick().await;
let _ = bearing.read_last_set_angle();

// 現在の角度を即時取得
let angle = bearing.get_angle_imm().unwrap();
```
