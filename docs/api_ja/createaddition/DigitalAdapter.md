# DigitalAdapter

**モジュール:** `createaddition::digital_adapter`  
**ペリフェラルタイプ:** `createaddition:digital_adapter`

Create Additions デジタルアダプター ペリフェラル。ディスプレイ制御、キネティックネットワーク管理、マシンステート読み取り（プーリー、ピストン、ベアリング、エレベーター）、Createモッドのコントラプション用ユーティリティ計算を提供する多機能ペリフェラルです。

## ブックリードメソッド

### ディスプレイ操作

#### `book_next_clear_line` / `read_last_clear_line`
特定のディスプレイ行をクリアします。
```rust
pub fn book_next_clear_line(&mut self, line: i32)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
```
**パラメータ:**
- `line: i32` — クリアする行番号

**戻り値:** `()`

---

#### `book_next_clear` / `read_last_clear`
すべてのディスプレイ行をクリアします。
```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
```
**戻り値:** `()`

---

#### `book_next_print` / `read_last_print`
次に利用可能な行にテキストを印刷します。
```rust
pub fn book_next_print(&mut self, text: &str)
pub fn read_last_print(&self) -> Result<(), PeripheralError>
```
**パラメータ:**
- `text: &str` — 印刷するテキスト

**戻り値:** `()`

---

#### `book_next_get_line` / `read_last_get_line`
特定の行のテキストを取得します。
```rust
pub fn book_next_get_line(&mut self, line: i32)
pub fn read_last_get_line(&self) -> Result<String, PeripheralError>
```
**パラメータ:**
- `line: i32` — 行番号

**戻り値:** `String`

---

#### `book_next_set_line` / `read_last_set_line`
特定の行のテキストを設定します。
```rust
pub fn book_next_set_line(&mut self, line: i32, text: &str)
pub fn read_last_set_line(&self) -> Result<(), PeripheralError>
```
**パラメータ:**
- `line: i32` — 行番号
- `text: &str` — 設定するテキスト

**戻り値:** `()`

---

#### `book_next_get_max_lines` / `read_last_get_max_lines`
ディスプレイの最大行数を取得します。
```rust
pub fn book_next_get_max_lines(&mut self)
pub fn read_last_get_max_lines(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

### キネティック制御

#### `book_next_set_target_speed` / `read_last_set_target_speed`
指定方向の機械コンポーネントの目標速度を設定します。
```rust
pub fn book_next_set_target_speed(&mut self, dir: &str, speed: f64)
pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError>
```
**パラメータ:**
- `dir: &str` — 方向（例: `"north"`、`"up"`）
- `speed: f64` — 目標RPM

**戻り値:** `()`

---

#### `book_next_get_target_speed` / `read_last_get_target_speed`
方向の目標速度を取得します。
```rust
pub fn book_next_get_target_speed(&mut self, dir: &str)
pub fn read_last_get_target_speed(&self) -> Result<f64, PeripheralError>
```
**パラメータ:**
- `dir: &str` — 方向

**戻り値:** `f64`

---

#### `book_next_get_kinetic_stress` / `read_last_get_kinetic_stress`
指定方向のストレス（SU）を取得します。
```rust
pub fn book_next_get_kinetic_stress(&mut self, dir: &str)
pub fn read_last_get_kinetic_stress(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_kinetic_capacity` / `read_last_get_kinetic_capacity`
指定方向のストレス容量（SU）を取得します。
```rust
pub fn book_next_get_kinetic_capacity(&mut self, dir: &str)
pub fn read_last_get_kinetic_capacity(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_kinetic_speed` / `read_last_get_kinetic_speed`
指定方向の実際の速度（RPM）を取得します。
```rust
pub fn book_next_get_kinetic_speed(&mut self, dir: &str)
pub fn read_last_get_kinetic_speed(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_kinetic_top_speed` / `read_last_get_kinetic_top_speed`
キネティックネットワークの最大速度（RPM）を取得します。
```rust
pub fn book_next_get_kinetic_top_speed(&mut self)
pub fn read_last_get_kinetic_top_speed(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### マシンステート

#### `book_next_get_pulley_distance` / `read_last_get_pulley_distance`
プーリーの延伸距離（ブロック）を取得します。
```rust
pub fn book_next_get_pulley_distance(&mut self, dir: &str)
pub fn read_last_get_pulley_distance(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_piston_distance` / `read_last_get_piston_distance`
ピストンの延伸距離（ブロック）を取得します。
```rust
pub fn book_next_get_piston_distance(&mut self, dir: &str)
pub fn read_last_get_piston_distance(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_bearing_angle` / `read_last_get_bearing_angle`
ベアリングの角度（度）を取得します。
```rust
pub fn book_next_get_bearing_angle(&mut self, dir: &str)
pub fn read_last_get_bearing_angle(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_elevator_floor` / `read_last_get_elevator_floor`
現在のエレベーターフロア番号を取得します。
```rust
pub fn book_next_get_elevator_floor(&mut self, dir: &str)
pub fn read_last_get_elevator_floor(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_has_elevator_arrived` / `read_last_has_elevator_arrived`
エレベーターが目的フロアに到着したかを確認します。
```rust
pub fn book_next_has_elevator_arrived(&mut self, dir: &str)
pub fn read_last_has_elevator_arrived(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

#### `book_next_get_elevator_floors` / `read_last_get_elevator_floors`
エレベーターの総フロア数を取得します。
```rust
pub fn book_next_get_elevator_floors(&mut self, dir: &str)
pub fn read_last_get_elevator_floors(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_elevator_floor_name` / `read_last_get_elevator_floor_name`
特定のフロア名を取得します。
```rust
pub fn book_next_get_elevator_floor_name(&mut self, dir: &str, index: i32)
pub fn read_last_get_elevator_floor_name(&self) -> Result<String, PeripheralError>
```
**パラメータ:**
- `dir: &str` — 方向
- `index: i32` — フロアインデックス

**戻り値:** `String`

---

#### `book_next_goto_elevator_floor` / `read_last_goto_elevator_floor`
エレベーターを指定フロアに移動させます。Yオフセットを返します。
```rust
pub fn book_next_goto_elevator_floor(&mut self, dir: &str, index: i32)
pub fn read_last_goto_elevator_floor(&self) -> Result<f64, PeripheralError>
```
**パラメータ:**
- `dir: &str` — 方向
- `index: i32` — 目標フロアインデックス

**戻り値:** `f64` — Y座標の差分

---

### ユーティリティ

#### `book_next_get_duration_angle` / `read_last_get_duration_angle`
指定RPMで指定角度を回転させるのにかかる時間（秒）を計算します。
```rust
pub fn book_next_get_duration_angle(&mut self, degrees: f64, rpm: f64)
pub fn read_last_get_duration_angle(&self) -> Result<f64, PeripheralError>
```
**パラメータ:**
- `degrees: f64` — 回転角度
- `rpm: f64` — RPMでの速度

**戻り値:** `f64` — 秒単位の所要時間

---

#### `book_next_get_duration_distance` / `read_last_get_duration_distance`
指定RPMで指定距離を移動するのにかかる時間（秒）を計算します。
```rust
pub fn book_next_get_duration_distance(&mut self, blocks: f64, rpm: f64)
pub fn read_last_get_duration_distance(&self) -> Result<f64, PeripheralError>
```
**パラメータ:**
- `blocks: f64` — ブロック単位の距離
- `rpm: f64` — RPMでの速度

**戻り値:** `f64` — 秒単位の所要時間

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::createaddition::DigitalAdapter;
use rust_computers_api::peripheral::Peripheral;

let mut adapter = DigitalAdapter::wrap(addr);

// 速度を設定してステートを読み取り
adapter.book_next_set_target_speed("north", 64.0);
adapter.book_next_get_kinetic_speed("north");
wait_for_next_tick().await;

loop {
    let speed = adapter.read_last_get_kinetic_speed();
    let angle = adapter.read_last_get_bearing_angle();

    adapter.book_next_get_kinetic_speed("north");
    adapter.book_next_get_bearing_angle("north");
    wait_for_next_tick().await;
}
```
