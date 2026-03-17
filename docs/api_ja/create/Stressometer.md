```markdown
# Stressometer

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:stressometer`

Create Stressometer ペリフェラル。回転応力とキャパシティを監視し、過負荷イベントとストレス変化イベントを提供します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_get_stress` / `read_last_get_stress`

現在のストレス値を取得します。

```rust
pub fn book_next_get_stress(&mut self)
pub fn read_last_get_stress(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在のストレス値 (SU)。

### `book_next_get_stress_capacity` / `read_last_get_stress_capacity`

ストレス容量を取得します。

```rust
pub fn book_next_get_stress_capacity(&mut self)
pub fn read_last_get_stress_capacity(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — ストレス容量 (SU)。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 即時メソッド (Immediate)

### `get_stress_imm`

現在のストレス値を即時取得します（tick 待機不要）。

```rust
pub fn get_stress_imm(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在のストレス値 (SU)。

### `get_stress_capacity_imm`

ストレス容量を即時取得します。

```rust
pub fn get_stress_capacity_imm(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — ストレス容量 (SU)。

## イベント待機メソッド (Event-Wait)

### `book_next_try_pull_overstressed` / `read_last_try_pull_overstressed`

過負荷イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_overstressed(&mut self)
pub fn read_last_try_pull_overstressed(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_overstressed`

過負荷イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_overstressed(&self) -> Result<(), PeripheralError>
```

### `book_next_try_pull_stress_change` / `read_last_try_pull_stress_change`

ストレス変化イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_stress_change(&mut self)
pub fn read_last_try_pull_stress_change(&self) -> Result<Option<(f32, f32)>, PeripheralError>
```

**戻り値:** `Option<(f32, f32)>` — イベント発生時は (ストレス, 容量) のタプル、未発生時は `None`。

### `pull_stress_change`

ストレス変化イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_stress_change(&self) -> Result<(f32, f32), PeripheralError>
```

**戻り値:** `(f32, f32)` — (ストレス, 容量) のタプル。

## 使用例

```rust
use rust_computers_api::create::stressometer::Stressometer;
use rust_computers_api::peripheral::Peripheral;

let mut stressometer = Stressometer::wrap(addr);

// 即時でストレスを取得
let stress = stressometer.get_stress_imm()?;
let capacity = stressometer.get_stress_capacity_imm()?;

// ストレス変化イベントを待機
let (new_stress, new_capacity) = stressometer.pull_stress_change().await?;

// 過負荷イベントを待機
stressometer.pull_overstressed().await?;
```

```
