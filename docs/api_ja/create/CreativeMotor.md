# CreativeMotor

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:creative_motor`

Create Creative Motor ペリフェラル。クリエイティブモーターの生成回転速度を制御します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_set_generated_speed` / `read_last_set_generated_speed`

生成する回転速度を設定します。

```rust
pub fn book_next_set_generated_speed(&mut self, speed: i32)
pub fn read_last_set_generated_speed(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `speed` | `i32` | 生成する回転速度 (RPM) |

### `book_next_get_generated_speed` / `read_last_get_generated_speed`

現在の生成回転速度を取得します。

```rust
pub fn book_next_get_generated_speed(&mut self)
pub fn read_last_get_generated_speed(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在の生成速度 (RPM)。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 即時メソッド (Immediate)

### `get_generated_speed_imm`

現在の生成回転速度を即時取得します（tick 待機不要）。

```rust
pub fn get_generated_speed_imm(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在の生成速度 (RPM)。

## 使用例

```rust
use rust_computers_api::create::creative_motor::CreativeMotor;
use rust_computers_api::peripheral::Peripheral;

let mut motor = CreativeMotor::wrap(addr);

// 速度を 256 RPM に設定
motor.book_next_set_generated_speed(256);
wait_for_next_tick().await;
motor.read_last_set_generated_speed()?;

// book-read パターンで速度を読み取り
motor.book_next_get_generated_speed();
wait_for_next_tick().await;
let speed = motor.read_last_get_generated_speed()?;

// または即時取得
let speed = motor.get_generated_speed_imm()?;
```
