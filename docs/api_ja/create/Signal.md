```markdown
# Signal

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:signal`

Create Signal ペリフェラル。列車シグナルの状態監視・制御、ブロック中の列車一覧、シグナルタイプの管理を行います。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_get_state` / `read_last_get_state`

シグナルの現在の状態を取得します。

```rust
pub fn book_next_get_state(&mut self)
pub fn read_last_get_state(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — 現在のシグナル状態（例: `"green"`、`"yellow"`、`"red"`）。

### `book_next_is_forced_red` / `read_last_is_forced_red`

強制赤信号かどうかを取得します。

```rust
pub fn book_next_is_forced_red(&mut self)
pub fn read_last_is_forced_red(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 強制赤信号の場合 `true`。

### `book_next_set_forced_red` / `read_last_set_forced_red`

強制赤信号を設定します。

```rust
pub fn book_next_set_forced_red(&mut self, powered: bool)
pub fn read_last_set_forced_red(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `powered` | `bool` | `true` で信号を強制的に赤にする |

### `book_next_list_blocking_train_names` / `read_last_list_blocking_train_names`

このシグナルセクションをブロックしている列車名の一覧を取得します。

```rust
pub fn book_next_list_blocking_train_names(&mut self)
pub fn read_last_list_blocking_train_names(&self) -> Result<Vec<String>, PeripheralError>
```

**戻り値:** `Vec<String>` — ブロック中の列車名リスト。

### `book_next_get_signal_type` / `read_last_get_signal_type`

シグナルタイプを取得します。

```rust
pub fn book_next_get_signal_type(&mut self)
pub fn read_last_get_signal_type(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — シグナルタイプ。

### `book_next_cycle_signal_type` / `read_last_cycle_signal_type`

シグナルタイプを次のタイプに切り替えます。

```rust
pub fn book_next_cycle_signal_type(&mut self)
pub fn read_last_cycle_signal_type(&self) -> Result<(), PeripheralError>
```

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 即時メソッド (Immediate)

### `get_state_imm`

シグナルの現在の状態を即時取得します（tick 待機不要）。

```rust
pub fn get_state_imm(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — 現在のシグナル状態。

### `is_forced_red_imm`

強制赤信号かどうかを即時取得します。

```rust
pub fn is_forced_red_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — 強制赤信号の場合 `true`。

### `list_blocking_train_names_imm`

ブロック中の列車名一覧を即時取得します。

```rust
pub fn list_blocking_train_names_imm(&self) -> Result<Vec<String>, PeripheralError>
```

**戻り値:** `Vec<String>` — ブロック中の列車名リスト。

### `get_signal_type_imm`

シグナルタイプを即時取得します。

```rust
pub fn get_signal_type_imm(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — シグナルタイプ。

## イベント待機メソッド (Event-Wait)

### `book_next_try_pull_train_signal_state_change` / `read_last_try_pull_train_signal_state_change`

シグナル状態変化イベントを 1 tick 待機して取得します。イベントがなければ `None` を返します。

```rust
pub fn book_next_try_pull_train_signal_state_change(&mut self)
pub fn read_last_try_pull_train_signal_state_change(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_signal_state_change`

シグナル状態変化イベントを受信するまで待機する非同期メソッドです。

```rust
pub async fn pull_train_signal_state_change(&self) -> Result<(), PeripheralError>
```

## 使用例

```rust
use rust_computers_api::create::signal::Signal;
use rust_computers_api::peripheral::Peripheral;

let mut signal = Signal::wrap(addr);

// シグナル状態を即時取得
let state = signal.get_state_imm()?;

// 強制赤信号に設定
signal.book_next_set_forced_red(true);
wait_for_next_tick().await;
signal.read_last_set_forced_red()?;

// ブロック中の列車一覧を取得
let trains = signal.list_blocking_train_names_imm()?;

// シグナル状態変化を待機
signal.pull_train_signal_state_change().await?;
```

```
