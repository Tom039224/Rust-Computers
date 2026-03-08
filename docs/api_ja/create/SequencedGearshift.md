```markdown
# SequencedGearshift

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:sequenced_gearshift`

Create Sequenced Gearshift ペリフェラル。シーケンス回転や直線移動をオプションの速度修飾子付きで制御します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_rotate` / `read_last_rotate`

指定量だけ回転させます。

```rust
pub fn book_next_rotate(&mut self, amount: i32, speed_modifier: Option<i32>)
pub fn read_last_rotate(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `amount` | `i32` | 回転量 |
| `speed_modifier` | `Option<i32>` | オプションの速度修飾子 |

### `book_next_move_by` / `read_last_move_by`

指定距離だけ移動させます。

```rust
pub fn book_next_move_by(&mut self, distance: i32, speed_modifier: Option<i32>)
pub fn read_last_move_by(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `distance` | `i32` | 移動距離 |
| `speed_modifier` | `Option<i32>` | オプションの速度修飾子 |

### `book_next_is_running` / `read_last_is_running`

ギアシフトが現在動作中かどうかを取得します。

```rust
pub fn book_next_is_running(&mut self)
pub fn read_last_is_running(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 動作中の場合 `true`。

## 即時メソッド (Immediate)

### `is_running_imm`

現在動作中かどうかを即時取得します（tick 待機不要）。

```rust
pub fn is_running_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 動作中の場合 `true`。

## 使用例

```rust
use rust_computers_api::create::sequenced_gearshift::SequencedGearshift;
use rust_computers_api::peripheral::Peripheral;

let mut gearshift = SequencedGearshift::wrap(addr);

// 速度修飾子付きで 90 度回転
gearshift.book_next_rotate(90, Some(2));
wait_for_next_tick().await;
gearshift.read_last_rotate()?;

// 5 ブロック移動
gearshift.book_next_move_by(5, None);
wait_for_next_tick().await;
gearshift.read_last_move_by()?;

// 即時で動作中か確認
let running = gearshift.is_running_imm()?;
```

```
