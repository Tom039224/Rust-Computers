```markdown
# Speedometer

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:speedometer`

Create Speedometer ペリフェラル。現在の回転速度を読み取り、速度変化イベントを提供します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_get_speed` / `read_last_get_speed`

現在の回転速度を取得します。

```rust
pub fn book_next_get_speed(&mut self)
pub fn read_last_get_speed(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在の回転速度 (RPM)。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 即時メソッド (Immediate)

### `get_speed_imm`

現在の回転速度を即時取得します（tick 待機不要）。

```rust
pub fn get_speed_imm(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在の回転速度 (RPM)。

## イベント待機メソッド (Event-Wait)

### `book_next_try_pull_speed_change` / `read_last_try_pull_speed_change`

速度変化イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_speed_change(&mut self)
pub fn read_last_try_pull_speed_change(&self) -> Result<Option<f32>, PeripheralError>
```

**戻り値:** `Option<f32>` — イベント発生時は新しい速度、未発生時は `None`。

### `pull_speed_change`

速度変化イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_speed_change(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 新しい速度。

## 使用例

```rust
use rust_computers_api::create::speedometer::Speedometer;
use rust_computers_api::peripheral::Peripheral;

let mut speedometer = Speedometer::wrap(addr);

// 即時で速度を取得
let speed = speedometer.get_speed_imm()?;

// book-read パターンで取得
speedometer.book_next_get_speed();
wait_for_next_tick().await;
let speed = speedometer.read_last_get_speed()?;

// 速度変化イベントを待機
let new_speed = speedometer.pull_speed_change().await?;
```

```
