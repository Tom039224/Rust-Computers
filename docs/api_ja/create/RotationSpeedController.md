# RotationSpeedController

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:rotation_speed_controller`

Create Rotation Speed Controller ペリフェラル。回転速度コントローラーのターゲット速度を制御します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_set_target_speed` / `read_last_set_target_speed`

ターゲット回転速度を設定します。

```rust
pub fn book_next_set_target_speed(&mut self, speed: i32)
pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `speed` | `i32` | ターゲット回転速度 (RPM) |

### `book_next_get_target_speed` / `read_last_get_target_speed`

現在のターゲット回転速度を取得します。

```rust
pub fn book_next_get_target_speed(&mut self)
pub fn read_last_get_target_speed(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在のターゲット速度 (RPM)。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 即時メソッド (Immediate)

### `get_target_speed_imm`

現在のターゲット回転速度を即時取得します（tick 待機不要）。

```rust
pub fn get_target_speed_imm(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `f32` — 現在のターゲット速度 (RPM)。

## 使用例

```rust
use rust_computers_api::create::rotation_speed_controller::RotationSpeedController;
use rust_computers_api::peripheral::Peripheral;

let mut controller = RotationSpeedController::wrap(addr);

// ターゲット速度を 128 RPM に設定
controller.book_next_set_target_speed(128);
wait_for_next_tick().await;
controller.read_last_set_target_speed()?;

// book-read パターンでターゲット速度を読み取り
controller.book_next_get_target_speed();
wait_for_next_tick().await;
let speed = controller.read_last_get_target_speed()?;

// または即時取得
let speed = controller.get_target_speed_imm()?;
```
