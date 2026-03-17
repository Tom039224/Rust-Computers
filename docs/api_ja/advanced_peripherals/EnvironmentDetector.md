# EnvironmentDetector

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:environment_detector`  
**ソース:** `EnvironmentDetectorPeripheral.java`

## 概要

EnvironmentDetectorペリフェラルは、ビーム情報、ディメンションの詳細、天候条件、光レベル、時間、月相、地形特性、睡眠ステータス、エンティティスキャンなど、環境データへの包括的なアクセスを提供します。天候認識システム、時間ベースの自動化、エンティティ検出システム、環境監視の作成に役立ちます。

## 3つの関数パターン

EnvironmentDetector APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```rust
// Rust example to be added
```

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## メソッド

### 環境と天候

#### `getBiome()` / `book_next_get_biome()` / `read_last_get_biome()` / `async_get_biome()`

ペリフェラルの位置にあるバイオームIDを取得します。

**Rust署名:**
```rust
pub fn book_next_get_biome(&mut self)
pub fn read_last_get_biome(&self) -> Result<String, PeripheralError>
pub async fn async_get_biome(&self) -> Result<String, PeripheralError>
```

**戻り値:** `string` — バイオームレジストリ名（例：`"minecraft:plains"`、`"minecraft:forest"`）

**例:**
```rust
// Rust example to be added
```
---

#### `getDimension()` / `book_next_get_dimension()` / `read_last_get_dimension()` / `async_get_dimension()`

ディメンションIDを取得します。

**Rust署名:**
```rust
pub fn book_next_get_dimension(&mut self)
pub fn read_last_get_dimension(&self) -> Result<String, PeripheralError>
pub async fn async_get_dimension(&self) -> Result<String, PeripheralError>
```

**戻り値:** `string` — ディメンションID（例：`"minecraft:overworld"`、`"minecraft:the_nether"`）

---

#### `isDimension(dim)` / `book_next_is_dimension(dim)` / `read_last_is_dimension()` / `async_is_dimension(dim)`

ペリフェラルが特定のディメンションにあるかどうかを確認します。

**Rust署名:**
```rust
pub fn book_next_is_dimension(&mut self, dim: &str)
pub fn read_last_is_dimension(&self) -> Result<bool, PeripheralError>
pub async fn async_is_dimension(&self, dim: &str) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `dim: string` — 確認するディメンションID

**戻り値:** `boolean` — 指定されたディメンションにある場合は`true`

---

#### `listDimensions()` / `book_next_list_dimensions()` / `read_last_list_dimensions()` / `async_list_dimensions()`

サーバー上で利用可能なすべてのディメンションIDを一覧表示します。

**Rust署名:**
```rust
pub fn book_next_list_dimensions(&mut self)
pub fn read_last_list_dimensions(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_list_dimensions(&self) -> Result<Vec<String>, PeripheralError>
```

**戻り値:** `table` — ディメンションIDの配列

---

#### `isRaining()` / `book_next_is_raining()` / `read_last_is_raining()` / `async_is_raining()`

現在雨が降っているかどうかを確認します。

**Rust署名:**
```rust
pub fn book_next_is_raining(&mut self)
pub fn read_last_is_raining(&self) -> Result<bool, PeripheralError>
pub async fn async_is_raining(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — 雨が降っている場合は`true`

---

#### `isThunder()` / `book_next_is_thunder()` / `read_last_is_thunder()` / `async_is_thunder()`

雷雨が発生しているかどうかを確認します。

**Rust署名:**
```rust
pub fn book_next_is_thunder(&mut self)
pub fn read_last_is_thunder(&self) -> Result<bool, PeripheralError>
pub async fn async_is_thunder(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — 雷雨の場合は`true`

---

#### `isSunny()` / `book_next_is_sunny()` / `read_last_is_sunny()` / `async_is_sunny()`

晴れているかどうかを確認します。

**Rust署名:**
```rust
pub fn book_next_is_sunny(&mut self)
pub fn read_last_is_sunny(&self) -> Result<bool, PeripheralError>
pub async fn async_is_sunny(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — 晴れている場合は`true`

---

### 光と時間

#### `getSkyLightLevel()` / `book_next_get_sky_light_level()` / `read_last_get_sky_light_level()` / `async_get_sky_light_level()`

空の光レベルを取得します（0–15）。

**Rust署名:**
```rust
pub fn book_next_get_sky_light_level(&mut self)
pub fn read_last_get_sky_light_level(&self) -> Result<i32, PeripheralError>
pub async fn async_get_sky_light_level(&self) -> Result<i32, PeripheralError>
```

**戻り値:** `number` — 空の光レベル（0-15）

---

#### `getBlockLightLevel()` / `book_next_get_block_light_level()` / `read_last_get_block_light_level()` / `async_get_block_light_level()`

ブロックの光レベルを取得します（0–15）。

**Rust署名:**
```rust
pub fn book_next_get_block_light_level(&mut self)
pub fn read_last_get_block_light_level(&self) -> Result<i32, PeripheralError>
pub async fn async_get_block_light_level(&self) -> Result<i32, PeripheralError>
```

**戻り値:** `number` — ブロックの光レベル（0-15）

---

#### `getDayLightLevel()` / `book_next_get_day_light_level()` / `read_last_get_day_light_level()` / `async_get_day_light_level()`

日光レベルを取得します（0–15）。

**Rust署名:**
```rust
pub fn book_next_get_day_light_level(&mut self)
pub fn read_last_get_day_light_level(&self) -> Result<i32, PeripheralError>
pub async fn async_get_day_light_level(&self) -> Result<i32, PeripheralError>
```

**戻り値:** `number` — 日光レベル（0-15）

---

#### `getTime()` / `book_next_get_time()` / `read_last_get_time()` / `async_get_time()`

ワールド時間をティックで取得します。

**Rust署名:**
```rust
pub fn book_next_get_time(&mut self)
pub fn read_last_get_time(&self) -> Result<i64, PeripheralError>
pub async fn async_get_time(&self) -> Result<i64, PeripheralError>
```

**戻り値:** `number` — ワールド時間（ティック単位、1日あたり0-24000）

**例:**
```rust
// Rust example to be added
```
---

### 月相

#### `getMoonId()` / `book_next_get_moon_id()` / `read_last_get_moon_id()` / `async_get_moon_id()`

現在の月相IDを取得します（0–7）。

**Rust署名:**
```rust
pub fn book_next_get_moon_id(&mut self)
pub fn read_last_get_moon_id(&self) -> Result<i32, PeripheralError>
pub async fn async_get_moon_id(&self) -> Result<i32, PeripheralError>
```

**戻り値:** `number` — 月相ID（0-7）

---

#### `getMoonName()` / `book_next_get_moon_name()` / `read_last_get_moon_name()` / `async_get_moon_name()`

月相名を取得します。

**Rust署名:**
```rust
pub fn book_next_get_moon_name(&mut self)
pub fn read_last_get_moon_name(&self) -> Result<String, PeripheralError>
pub async fn async_get_moon_name(&self) -> Result<String, PeripheralError>
```

**戻り値:** `string` — 月相名（例：`"満月"`、`"新月"`）

---

#### `isMoon(phase)` / `book_next_is_moon(phase)` / `read_last_is_moon()` / `async_is_moon(phase)`

現在の月相が指定された名前と一致するかどうかを確認し��す。

**Rust署名:**
```rust
pub fn book_next_is_moon(&mut self, phase: &str)
pub fn read_last_is_moon(&self) -> Result<bool, PeripheralError>
pub async fn async_is_moon(&self, phase: &str) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `phase: string` — 確認する月相名

**戻り値:** `boolean` — 現在の月相が一致する場合は`true`

---

### 地形

#### `isSlimeChunk()` / `book_next_is_slime_chunk()` / `read_last_is_slime_chunk()` / `async_is_slime_chunk()`

現在のチャンクがスライムチャンクかどうかを確認します。

**Rust署名:**
```rust
pub fn book_next_is_slime_chunk(&mut self)
pub fn read_last_is_slime_chunk(&self) -> Result<bool, PeripheralError>
pub async fn async_is_slime_chunk(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — 現在のチャンクがスライムチャンクの場合は`true`

---

### 睡眠

#### `canSleepHere()` / `book_next_can_sleep_here()` / `read_last_can_sleep_here()` / `async_can_sleep_here()`

このディメンションで睡眠が可能かどうかを確認します。

**Rust署名:**
```rust
pub fn book_next_can_sleep_here(&mut self)
pub fn read_last_can_sleep_here(&self) -> Result<bool, PeripheralError>
pub async fn async_can_sleep_here(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — 睡眠が可能な場合は`true`

---

#### `canSleepPlayer(name)` / `book_next_can_sleep_player(name)` / `read_last_can_sleep_player()` / `async_can_sleep_player(name)`

特定のプレイヤーが睡眠できるかどうかを確認します。

**Rust署名:**
```rust
pub fn book_next_can_sleep_player(&mut self, name: &str)
pub fn read_last_can_sleep_player(&self) -> Result<bool, PeripheralError>
pub async fn async_can_sleep_player(&self, name: &str) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `name: string` — プレイヤー名

**戻り値:** `boolean` — プレイヤーが睡眠できる場合は`true`

---

### エンティティスキャン

#### `scanEntities(radius)` / `book_next_scan_entities(radius)` / `read_last_scan_entities()` / `async_scan_entities(radius)`

指定された半径内のエンティティをスキャンします。

**Rust署名:**
```rust
pub fn book_next_scan_entities(&mut self, radius: f64)
pub fn read_last_scan_entities(&self) -> Result<Vec<EntityInfo>, PeripheralError>
pub async fn async_scan_entities(&self, radius: f64) -> Result<Vec<EntityInfo>, PeripheralError>
```

**パラメータ:**
- `radius: number` — スキャン半径（ブロック単位）

**戻り値:** `table` — エンティティ情報の配列

**エンティティ構造体:**
```rust
// Rust example to be added
```

**例:**
```rust
// Rust example to be added
```
---

#### `scanCost(radius)` / `book_next_scan_cost(radius)` / `read_last_scan_cost()` / `scan_cost_imm(radius)`

スキャン操作の燃料コストを取得します。

**Rust署名:**
```rust
pub fn book_next_scan_cost(&mut self, radius: f64)
pub fn read_last_scan_cost(&self) -> Result<f64, PeripheralError>
pub fn scan_cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```

**パラメータ:**
- `radius: number` — スキャン半径（ブロック単位）

**戻り値:** `number` — 燃料コスト

**注:** `scan_cost_imm`メソッドは次のティックを待たずに即座に結果を返します。

---

## イベント

EnvironmentDetectorペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: 天候監視

```rust
// Rust example to be added
```

### 例2: 時間ベースの自動化

```rust
// Rust example to be added
```

### 例3: エンティティ検出

```rust
// Rust example to be added
```

### 例4: バイオーム検出

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **ペリフェラル切断**: EnvironmentDetectorにアクセスできなくなった
- **無効な半径**: スキャン半径が負または最大値を超えている
- **無効なディメンション**: ディメンションIDが存在しない
- **無効なプレイヤー**: プレイヤー名が見つからない

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### EntityInfo
```rust
// Rust example to be added
```
---

## 注記

- 光レベルは0（暗い）から15（明るい）までです
- ワールド時間はティックで測定されます（1日あたり0-24000）
- 月相は0（満月）から7（新月）までサイクルします
- エンティティスキャンには、半径とともに増加する燃料コストがかかります
- 3つの関数パターンは効率的なバッチ操作を可能にします
- スライムチャンクはワールドシードとチャンク座標によって決定されます

---

## 関連

- [BlockReader](./BlockReader.md) — ブロック情報の読み取り
- [PlayerDetector](./PlayerDetector.md) — プレイヤーの検出
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント