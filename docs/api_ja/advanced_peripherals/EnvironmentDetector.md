# EnvironmentDetector

**モジュール:** `advanced_peripherals::environment_detector`  
**ペリフェラルタイプ:** `advancedPeripherals:environment_detector`

AdvancedPeripherals 環境検出器 ペリフェラル。バイオーム、ディメンション、天候、光レベル、時間、月相、地形、睡眠状態、エンティティスキャンなどの環境データへの包括的なアクセスを提供します。

## ブックリードメソッド

### 環境・天候

#### `book_next_get_biome` / `read_last_get_biome`
ペリフェラルの位置のバイオームIDを取得します。
```rust
pub fn book_next_get_biome(&mut self)
pub fn read_last_get_biome(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

#### `book_next_get_dimension` / `read_last_get_dimension`
ディメンションIDを取得します。
```rust
pub fn book_next_get_dimension(&mut self)
pub fn read_last_get_dimension(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

#### `book_next_is_dimension` / `read_last_is_dimension`
ペリフェラルが特定のディメンションにあるかを確認します。
```rust
pub fn book_next_is_dimension(&mut self, dim: &str)
pub fn read_last_is_dimension(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `dim: &str` — チェックするディメンションID

**戻り値:** `bool`

---

#### `book_next_list_dimensions` / `read_last_list_dimensions`
全ディメンションIDをリストします。
```rust
pub fn book_next_list_dimensions(&mut self)
pub fn read_last_list_dimensions(&self) -> Result<Vec<String>, PeripheralError>
```
**戻り値:** `Vec<String>`

---

#### `book_next_is_raining` / `read_last_is_raining`
雨が降っているかを確認します。
```rust
pub fn book_next_is_raining(&mut self)
pub fn read_last_is_raining(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

#### `book_next_is_thunder` / `read_last_is_thunder`
雷雨かを確認します。
```rust
pub fn book_next_is_thunder(&mut self)
pub fn read_last_is_thunder(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

#### `book_next_is_sunny` / `read_last_is_sunny`
晴れかを確認します。
```rust
pub fn book_next_is_sunny(&mut self)
pub fn read_last_is_sunny(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

### 光・時間

#### `book_next_get_sky_light_level` / `read_last_get_sky_light_level`
空の光レベル（0–15）を取得します。
```rust
pub fn book_next_get_sky_light_level(&mut self)
pub fn read_last_get_sky_light_level(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_block_light_level` / `read_last_get_block_light_level`
ブロックの光レベル（0–15）を取得します。
```rust
pub fn book_next_get_block_light_level(&mut self)
pub fn read_last_get_block_light_level(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_day_light_level` / `read_last_get_day_light_level`
日光レベル（0–15）を取得します。
```rust
pub fn book_next_get_day_light_level(&mut self)
pub fn read_last_get_day_light_level(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_time` / `read_last_get_time`
ワールド時間をティックで取得します。
```rust
pub fn book_next_get_time(&mut self)
pub fn read_last_get_time(&self) -> Result<i64, PeripheralError>
```
**戻り値:** `i64`

---

### 月相

#### `book_next_get_moon_id` / `read_last_get_moon_id`
現在の月相ID（0–7）を取得します。
```rust
pub fn book_next_get_moon_id(&mut self)
pub fn read_last_get_moon_id(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_moon_name` / `read_last_get_moon_name`
月相名を取得します。
```rust
pub fn book_next_get_moon_name(&mut self)
pub fn read_last_get_moon_name(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

#### `book_next_is_moon` / `read_last_is_moon`
現在の月相が指定した名前と一致するかを確認します。
```rust
pub fn book_next_is_moon(&mut self, phase: &str)
pub fn read_last_is_moon(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `phase: &str` — チェックする月相名

**戻り値:** `bool`

---

### 地形

#### `book_next_is_slime_chunk` / `read_last_is_slime_chunk`
現在のチャンクがスライムチャンクかを確認します。
```rust
pub fn book_next_is_slime_chunk(&mut self)
pub fn read_last_is_slime_chunk(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

### 睡眠

#### `book_next_can_sleep_here` / `read_last_can_sleep_here`
このディメンションで睡眠可能かを確認します。
```rust
pub fn book_next_can_sleep_here(&mut self)
pub fn read_last_can_sleep_here(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

#### `book_next_can_sleep_player` / `read_last_can_sleep_player`
特定のプレイヤーが睡眠可能かを確認します。
```rust
pub fn book_next_can_sleep_player(&mut self, name: &str)
pub fn read_last_can_sleep_player(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `name: &str` — プレイヤー名

**戻り値:** `bool`

---

### エンティティスキャン

#### `book_next_scan_entities` / `read_last_scan_entities`
指定半径内のエンティティをスキャンします。
```rust
pub fn book_next_scan_entities(&mut self, radius: f64)
pub fn read_last_scan_entities(&self) -> Result<Vec<EntityInfo>, PeripheralError>
```
**パラメータ:**
- `radius: f64` — スキャン半径

**戻り値:** `Vec<EntityInfo>`

---

#### `book_next_scan_cost` / `read_last_scan_cost` / `scan_cost_imm`
スキャン操作の燃料コストを取得します。
```rust
pub fn book_next_scan_cost(&mut self, radius: f64)
pub fn read_last_scan_cost(&self) -> Result<f64, PeripheralError>
pub fn scan_cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```
**パラメータ:**
- `radius: f64` — スキャン半径

**戻り値:** `f64`

## イミディエイトメソッド

- `scan_cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>`

## 型定義

```rust
pub struct EntityInfo {
    pub id: String,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub tags: Vec<String>,
    pub can_freeze: Option<bool>,       // serde: "canFreeze"
    pub is_glowing: Option<bool>,       // serde: "isGlowing"
    pub is_in_wall: Option<bool>,       // serde: "isInWall"
    pub health: Option<f64>,
    pub max_health: Option<f64>,        // serde: "maxHealth"
    pub last_damage_source: Option<String>, // serde: "lastDamageSource"
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}
```

## 使用例

```rust
use rust_computers_api::advanced_peripherals::EnvironmentDetector;
use rust_computers_api::peripheral::Peripheral;

let mut env = EnvironmentDetector::wrap(addr);

loop {
    let biome = env.read_last_get_biome();
    let raining = env.read_last_is_raining();
    let entities = env.read_last_scan_entities();

    env.book_next_get_biome();
    env.book_next_is_raining();
    env.book_next_scan_entities(16.0);
    wait_for_next_tick().await;
}
```
