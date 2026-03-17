# Boiler
**モジュール:** `clockwork_cc_compat::boiler`  
**ペリフェラルタイプ:** `clockwork:boiler`

Clockwork CC Compat のボイラーペリフェラル。Create スタイルのボイラーの熱、水供給、サイズ、エンジン/ホイッスル接続、流体内容、コントローラー位置を含む包括的なモニタリングを提供します。

## 型定義

### `CLFluidInfo`
```rust
pub struct CLFluidInfo {
    pub fluid: String,
    pub amount: u32,
    pub capacity: u32,
}
```
流体のタイプ名、現在の量、タンク容量を含みます。

### `CLPosition`
```rust
pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```
3D ブロック座標。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## Book-Read メソッド

以下のすべてのメソッドには `_imm` Immediate バリアントもあります（Immediate メソッドセクションを参照）。

### `book_next_is_active` / `read_last_is_active`
ボイラーが現在アクティブかどうかを確認する。
```rust
pub fn book_next_is_active(&mut self)
pub fn read_last_is_active(&self) -> Result<bool, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** アクティブなら `true`。

---

### `book_next_get_heat_level` / `read_last_get_heat_level`
現在の熱レベルを取得する。
```rust
pub fn book_next_get_heat_level(&mut self)
pub fn read_last_get_heat_level(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 熱レベル (`f64`)。

---

### `book_next_get_active_heat` / `read_last_get_active_heat`
アクティブ熱の値を取得する。
```rust
pub fn book_next_get_active_heat(&mut self)
pub fn read_last_get_active_heat(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** アクティブ熱 (`f64`)。

---

### `book_next_is_passive_heat` / `read_last_is_passive_heat`
ボイラーがパッシブ熱を使用しているか確認する。
```rust
pub fn book_next_is_passive_heat(&mut self)
pub fn read_last_is_passive_heat(&self) -> Result<bool, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** パッシブ熱使用中なら `true`。

---

### `book_next_get_water_supply` / `read_last_get_water_supply`
現在の水供給レベルを取得する。
```rust
pub fn book_next_get_water_supply(&mut self)
pub fn read_last_get_water_supply(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 水供給量 (`f64`)。

---

### `book_next_get_attached_engines` / `read_last_get_attached_engines`
接続エンジン数を取得する。
```rust
pub fn book_next_get_attached_engines(&mut self)
pub fn read_last_get_attached_engines(&self) -> Result<u32, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 接続エンジン数 (`u32`)。

---

### `book_next_get_attached_whistles` / `read_last_get_attached_whistles`
接続ホイッスル数を取得する。
```rust
pub fn book_next_get_attached_whistles(&mut self)
pub fn read_last_get_attached_whistles(&self) -> Result<u32, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 接続ホイッスル数 (`u32`)。

---

### `book_next_get_engine_efficiency` / `read_last_get_engine_efficiency`
エンジン効率を取得する。
```rust
pub fn book_next_get_engine_efficiency(&mut self)
pub fn read_last_get_engine_efficiency(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** エンジン効率 (`f64`)。

---

### `book_next_get_boiler_size` / `read_last_get_boiler_size`
ボイラーサイズを取得する。
```rust
pub fn book_next_get_boiler_size(&mut self)
pub fn read_last_get_boiler_size(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** ボイラーサイズ (`f64`)。

---

### `book_next_get_width` / `read_last_get_width`
ボイラーの幅を取得する。
```rust
pub fn book_next_get_width(&mut self)
pub fn read_last_get_width(&self) -> Result<u32, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 幅 (`u32`)。

---

### `book_next_get_height` / `read_last_get_height`
ボイラーの高さを取得する。
```rust
pub fn book_next_get_height(&mut self)
pub fn read_last_get_height(&self) -> Result<u32, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 高さ (`u32`)。

---

### `book_next_get_max_heat_for_size` / `read_last_get_max_heat_for_size`
現在のボイラーサイズに対する最大熱を取得する。
```rust
pub fn book_next_get_max_heat_for_size(&mut self)
pub fn read_last_get_max_heat_for_size(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** サイズに対する最大熱 (`f64`)。

---

### `book_next_get_max_heat_for_water` / `read_last_get_max_heat_for_water`
現在の水供給に対する最大熱を取得する。
```rust
pub fn book_next_get_max_heat_for_water(&mut self)
pub fn read_last_get_max_heat_for_water(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 水供給に対する最大熱 (`f64`)。

---

### `book_next_get_fill_state` / `read_last_get_fill_state`
ボイラーの充填状態を取得する。
```rust
pub fn book_next_get_fill_state(&mut self)
pub fn read_last_get_fill_state(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 充填状態 (`f64`)。

---

### `book_next_get_fluid_contents` / `read_last_get_fluid_contents`
ボイラーの流体内容を取得する。
```rust
pub fn book_next_get_fluid_contents(&mut self)
pub fn read_last_get_fluid_contents(&self) -> Result<CLFluidInfo, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 流体タイプ、量、容量を含む `CLFluidInfo`。

---

### `book_next_get_controller_pos` / `read_last_get_controller_pos`
ボイラーコントローラーブロックの位置を取得する。
```rust
pub fn book_next_get_controller_pos(&mut self)
pub fn read_last_get_controller_pos(&self) -> Result<CLPosition, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** x, y, z 座標を含む `CLPosition`。

## Immediate メソッド

上記のすべての book-read メソッドには、`_imm` サフィックスの Immediate バリアントがあります。book-read サイクルなしで結果を返します:

| メソッド | 戻り値型 |
|---------|----------|
| `is_active_imm()` | `Result<bool, PeripheralError>` |
| `get_heat_level_imm()` | `Result<f64, PeripheralError>` |
| `get_active_heat_imm()` | `Result<f64, PeripheralError>` |
| `is_passive_heat_imm()` | `Result<bool, PeripheralError>` |
| `get_water_supply_imm()` | `Result<f64, PeripheralError>` |
| `get_attached_engines_imm()` | `Result<u32, PeripheralError>` |
| `get_attached_whistles_imm()` | `Result<u32, PeripheralError>` |
| `get_engine_efficiency_imm()` | `Result<f64, PeripheralError>` |
| `get_boiler_size_imm()` | `Result<f64, PeripheralError>` |
| `get_width_imm()` | `Result<u32, PeripheralError>` |
| `get_height_imm()` | `Result<u32, PeripheralError>` |
| `get_max_heat_for_size_imm()` | `Result<f64, PeripheralError>` |
| `get_max_heat_for_water_imm()` | `Result<f64, PeripheralError>` |
| `get_fill_state_imm()` | `Result<f64, PeripheralError>` |
| `get_fluid_contents_imm()` | `Result<CLFluidInfo, PeripheralError>` |
| `get_controller_pos_imm()` | `Result<CLPosition, PeripheralError>` |

## 使用例

```rust
use rust_computers_api::clockwork_cc_compat::boiler::Boiler;
use rust_computers_api::peripheral::Peripheral;

let mut boiler = Boiler::find().expect("Boiler が見つかりません");

// Book-read: アクティブ状態と熱を確認
boiler.book_next_is_active();
boiler.book_next_get_heat_level();

let active = boiler.read_last_is_active().unwrap();
let heat = boiler.read_last_get_heat_level().unwrap();

// Immediate: 流体内容を取得
let fluid = boiler.get_fluid_contents_imm().unwrap();
println!("流体: {}, 量: {}/{}", fluid.fluid, fluid.amount, fluid.capacity);

// Immediate: コントローラー位置を取得
let pos = boiler.get_controller_pos_imm().unwrap();
```
