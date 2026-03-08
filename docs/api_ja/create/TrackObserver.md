```markdown
# TrackObserver

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:track_observer`

Create Track Observer ペリフェラル。線路上の列車通過をリアルタイムイベントで監視します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_is_train_passing` / `read_last_is_train_passing`

列車が現在通過中かどうかを取得します。

```rust
pub fn book_next_is_train_passing(&mut self)
pub fn read_last_is_train_passing(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が通過中の場合 `true`。

### `book_next_get_passing_train_name` / `read_last_get_passing_train_name`

現在通過中の列車名を取得します。

```rust
pub fn book_next_get_passing_train_name(&mut self)
pub fn read_last_get_passing_train_name(&self) -> Result<Option<String>, PeripheralError>
```

**戻り値:** `Option<String>` — 列車名。列車がない場合は `None`。

## 即時メソッド (Immediate)

### `is_train_passing_imm`

列車が現在通過中かどうかを即時取得します（tick 待機不要）。

```rust
pub fn is_train_passing_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 列車が通過中の場合 `true`。

### `get_passing_train_name_imm`

現在通過中の列車名を即時取得します。

```rust
pub fn get_passing_train_name_imm(&self) -> Result<Option<String>, PeripheralError>
```

**戻り値:** `Option<String>` — 列車名。列車がない場合は `None`。

## イベント待機メソッド (Event-Wait)

### `book_next_try_pull_train_passing` / `read_last_try_pull_train_passing`

列車通過開始イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_train_passing(&mut self)
pub fn read_last_try_pull_train_passing(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_passing`

列車通過開始イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_train_passing(&self) -> Result<(), PeripheralError>
```

### `book_next_try_pull_train_passed` / `read_last_try_pull_train_passed`

列車通過完了イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_train_passed(&mut self)
pub fn read_last_try_pull_train_passed(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_passed`

列車通過完了イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_train_passed(&self) -> Result<(), PeripheralError>
```

## 使用例

```rust
use rust_computers_api::create::track_observer::TrackObserver;
use rust_computers_api::peripheral::Peripheral;

let mut observer = TrackObserver::wrap(addr);

// 列車が通過中か確認
let passing = observer.is_train_passing_imm()?;

// 通過中の列車名を取得
let name = observer.get_passing_train_name_imm()?;

// 列車通過開始を待機
observer.pull_train_passing().await?;

// 列車通過完了を待機
observer.pull_train_passed().await?;
```

```
