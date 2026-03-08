```markdown
# Station

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:station`

Create Station ペリフェラル。列車ステーションの操作を管理します。組み立て・分解、スケジュール管理、列車追跡などの機能を提供します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_assemble` / `read_last_assemble`

このステーションで列車を組み立てます。

```rust
pub fn book_next_assemble(&mut self)
pub fn read_last_assemble(&self) -> Result<(), PeripheralError>
```

### `book_next_disassemble` / `read_last_disassemble`

このステーションの列車を分解します。

```rust
pub fn book_next_disassemble(&mut self)
pub fn read_last_disassemble(&self) -> Result<(), PeripheralError>
```

### `book_next_set_assembly_mode` / `read_last_set_assembly_mode`

組み立てモードを設定します。

```rust
pub fn book_next_set_assembly_mode(&mut self, mode: bool)
pub fn read_last_set_assembly_mode(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `mode` | `bool` | `true` で組み立てモードを有効化 |

### `book_next_is_in_assembly_mode` / `read_last_is_in_assembly_mode`

組み立てモードかどうかを取得します。

```rust
pub fn book_next_is_in_assembly_mode(&mut self)
pub fn read_last_is_in_assembly_mode(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 組み立てモードの場合 `true`。

### `book_next_get_station_name` / `read_last_get_station_name`

ステーション名を取得します。

```rust
pub fn book_next_get_station_name(&mut self)
pub fn read_last_get_station_name(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — ステーション名。

### `book_next_set_station_name` / `read_last_set_station_name`

ステーション名を設定します。

```rust
pub fn book_next_set_station_name(&mut self, name: &str)
pub fn read_last_set_station_name(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `name` | `&str` | 新しいステーション名 |

### `book_next_is_train_present` / `read_last_is_train_present`

列車がステーションに存在するかどうかを取得します。

```rust
pub fn book_next_is_train_present(&mut self)
pub fn read_last_is_train_present(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が存在する場合 `true`。

### `book_next_is_train_imminent` / `read_last_is_train_imminent`

列車が間もなく到着するかどうかを取得します。

```rust
pub fn book_next_is_train_imminent(&mut self)
pub fn read_last_is_train_imminent(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が間もなく到着する場合 `true`。

### `book_next_is_train_enroute` / `read_last_is_train_enroute`

列車が経路上にあるかどうかを取得します。

```rust
pub fn book_next_is_train_enroute(&mut self)
pub fn read_last_is_train_enroute(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が経路上にある場合 `true`。

### `book_next_get_train_name` / `read_last_get_train_name`

このステーションの列車名を取得します。

```rust
pub fn book_next_get_train_name(&mut self)
pub fn read_last_get_train_name(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — 列車名。

### `book_next_set_train_name` / `read_last_set_train_name`

このステーションの列車名を設定します。

```rust
pub fn book_next_set_train_name(&mut self, name: &str)
pub fn read_last_set_train_name(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `name` | `&str` | 新しい列車名 |

### `book_next_has_schedule` / `read_last_has_schedule`

列車にスケジュールがあるかどうかを取得します。

```rust
pub fn book_next_has_schedule(&mut self)
pub fn read_last_has_schedule(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — スケジュールが存在する場合 `true`。

### `book_next_get_schedule` / `read_last_get_schedule`

列車のスケジュールを取得します。

```rust
pub fn book_next_get_schedule(&mut self)
pub fn read_last_get_schedule(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
```

**戻り値:** `BTreeMap<String, Value>` — スケジュールデータ。

### `book_next_set_schedule` / `read_last_set_schedule`

列車のスケジュールを設定します。

```rust
pub fn book_next_set_schedule(&mut self, schedule: &BTreeMap<String, Value>) -> Result<(), PeripheralError>
pub fn read_last_set_schedule(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `schedule` | `&BTreeMap<String, Value>` | 設定するスケジュールデータ |

### `book_next_can_train_reach` / `read_last_can_train_reach`

列車が指定駅に到達可能かどうかを確認します。

```rust
pub fn book_next_can_train_reach(&mut self, dest: &str)
pub fn read_last_can_train_reach(&self) -> Result<(bool, Option<String>), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `dest` | `&str` | 目的地ステーション名 |

**戻り値:** `(bool, Option<String>)` — (到達可否, 理由) のタプル。

### `book_next_distance_to` / `read_last_distance_to`

指定駅までの距離を取得します。

```rust
pub fn book_next_distance_to(&mut self, dest: &str)
pub fn read_last_distance_to(&self) -> Result<(Option<f64>, Option<String>), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `dest` | `&str` | 目的地ステーション名 |

**戻り値:** `(Option<f64>, Option<String>)` — (距離, 理由) のタプル。到達不可の場合、距離は `None`。

## 即時メソッド (Immediate)

### `is_in_assembly_mode_imm`

組み立てモードかどうかを即時取得します。

```rust
pub fn is_in_assembly_mode_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 組み立てモードの場合 `true`。

### `get_station_name_imm`

ステーション名を即時取得します。

```rust
pub fn get_station_name_imm(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — ステーション名。

### `is_train_present_imm`

列車が存在するかどうかを即時取得します。

```rust
pub fn is_train_present_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が存在する場合 `true`。

### `is_train_imminent_imm`

列車が間もなく到着するかどうかを即時取得します。

```rust
pub fn is_train_imminent_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が間もなく到着する場合 `true`。

### `is_train_enroute_imm`

列車が経路上にあるかどうかを即時取得します。

```rust
pub fn is_train_enroute_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が経路上にある場合 `true`。

### `get_train_name_imm`

列車名を即時取得します。

```rust
pub fn get_train_name_imm(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — 列車名。

### `has_schedule_imm`

スケジュールがあるかどうかを即時取得します。

```rust
pub fn has_schedule_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — スケジュールが存在する場合 `true`。

### `get_schedule_imm`

スケジュールを即時取得します。

```rust
pub fn get_schedule_imm(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
```

**戻り値:** `BTreeMap<String, Value>` — スケジュールデータ。

### `can_train_reach_imm`

列車が目的地に到達可能かどうかを即時確認します。

```rust
pub fn can_train_reach_imm(&self, dest: &str) -> Result<(bool, Option<String>), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `dest` | `&str` | 目的地ステーション名 |

**戻り値:** `(bool, Option<String>)` — (到達可否, 理由) のタプル。

### `distance_to_imm`

目的地までの距離を即時取得します。

```rust
pub fn distance_to_imm(&self, dest: &str) -> Result<(Option<f64>, Option<String>), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `dest` | `&str` | 目的地ステーション名 |

**戻り値:** `(Option<f64>, Option<String>)` — (距離, 理由) のタプル。

## イベント待機メソッド (Event-Wait)

### `book_next_try_pull_train_arrive` / `read_last_try_pull_train_arrive`

列車到着イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_train_arrive(&mut self)
pub fn read_last_try_pull_train_arrive(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_arrive`

列車到着イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_train_arrive(&self) -> Result<(), PeripheralError>
```

### `book_next_try_pull_train_depart` / `read_last_try_pull_train_depart`

列車出発イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_train_depart(&mut self)
pub fn read_last_try_pull_train_depart(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_depart`

列車出発イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_train_depart(&self) -> Result<(), PeripheralError>
```

## 使用例

```rust
use rust_computers_api::create::station::Station;
use rust_computers_api::peripheral::Peripheral;

let mut station = Station::wrap(addr);

// ステーション名を取得
let name = station.get_station_name_imm()?;

// 列車の存在を確認
let present = station.is_train_present_imm()?;

// ステーション名を設定
station.book_next_set_station_name("Depot A");
wait_for_next_tick().await;
station.read_last_set_station_name()?;

// 到達可能性を確認
station.book_next_can_train_reach("Depot B");
wait_for_next_tick().await;
let (reachable, reason) = station.read_last_can_train_reach()?;

// 列車到着を待機
station.pull_train_arrive().await?;
```

```
