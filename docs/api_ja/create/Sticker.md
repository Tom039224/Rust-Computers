```markdown
# Sticker

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:sticker`

Create Sticker ペリフェラル。スティッカー（ピストン状ブロック）の伸展・収縮状態を制御します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_is_extended` / `read_last_is_extended`

スティッカーが伸展状態かどうかを取得します。

```rust
pub fn book_next_is_extended(&mut self)
pub fn read_last_is_extended(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 伸展状態の場合 `true`。

### `book_next_is_attached_to_block` / `read_last_is_attached_to_block`

ブロックに接続されているかどうかを取得します。

```rust
pub fn book_next_is_attached_to_block(&mut self)
pub fn read_last_is_attached_to_block(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — ブロックに接続されている場合 `true`。

### `book_next_extend` / `read_last_extend`

スティッカーを伸展させます。操作の成否を返します。

```rust
pub fn book_next_extend(&mut self)
pub fn read_last_extend(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 伸展に成功した場合 `true`。

### `book_next_retract` / `read_last_retract`

スティッカーを収縮させます。操作の成否を返します。

```rust
pub fn book_next_retract(&mut self)
pub fn read_last_retract(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 収縮に成功した場合 `true`。

### `book_next_toggle` / `read_last_toggle`

スティッカーの伸展/収縮を切り替えます。操作の成否を返します。

```rust
pub fn book_next_toggle(&mut self)
pub fn read_last_toggle(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 切り替えに成功した場合 `true`。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 即時メソッド (Immediate)

### `is_extended_imm`

伸展状態かどうかを即時取得します（tick 待機不要）。

```rust
pub fn is_extended_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 伸展状態の場合 `true`。

### `is_attached_to_block_imm`

ブロックに接続されているかどうかを即時取得します。

```rust
pub fn is_attached_to_block_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — ブロックに接続されている場合 `true`。

## 使用例

```rust
use rust_computers_api::create::sticker::Sticker;
use rust_computers_api::peripheral::Peripheral;

let mut sticker = Sticker::wrap(addr);

// 伸展状態を確認
let extended = sticker.is_extended_imm()?;

// スティッカーを伸展
sticker.book_next_extend();
wait_for_next_tick().await;
let success = sticker.read_last_extend()?;

// 状態を切り替え
sticker.book_next_toggle();
wait_for_next_tick().await;
let success = sticker.read_last_toggle()?;
```

```
