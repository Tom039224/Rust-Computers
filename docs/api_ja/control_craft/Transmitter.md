# Transmitter

**モジュール:** `control_craft::transmitter`  
**ペリフェラルタイプ:** `controlcraft:transmitter_peripheral`

Control-Craft の Transmitter ペリフェラル。リンクされたデバイス間でのリモートメソッド呼び出し（同期・非同期）およびプロトコル設定を行います。

## Book-Read メソッド

### リモート呼び出し

#### `book_next_call_remote` / `read_last_call_remote`
指定したアクセスキーとコンテキストでリモートメソッドを同期呼び出しします。
```rust
pub fn book_next_call_remote(&mut self, access: &str, ctx: &str, extra_args: &[Vec<u8>]) { ... }
pub fn read_last_call_remote(&self) -> Result<Value, PeripheralError> { ... }
```
**パラメータ:**
- `access: &str` — リモートターゲットのアクセスキー
- `ctx: &str` — コンテキスト文字列
- `extra_args: &[Vec<u8>]` — msgpack エンコード済みバイト配列としての追加可変長引数

**戻り値:** `Value` — リモートメソッドの返り値

#### `book_next_call_remote_async` / `read_last_call_remote_async`
リモートメソッドを非同期呼び出しします。結果は指定したイベント（スロット）で受信します。
```rust
pub fn book_next_call_remote_async(
    &mut self,
    access: &str,
    ctx: &str,
    slot_name: &str,
    remote_name: &str,
    method: &str,
    extra_args: &[Vec<u8>],
) { ... }
pub fn read_last_call_remote_async(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `access: &str` — リモートターゲットのアクセスキー
- `ctx: &str` — コンテキスト文字列
- `slot_name: &str` — 結果を受信するイベントスロット名
- `remote_name: &str` — リモートペリフェラル名
- `method: &str` — リモートメソッド名
- `extra_args: &[Vec<u8>]` — msgpack エンコード済みバイト配列としての追加可変長引数

---

### 設定

#### `book_next_set_protocol` / `read_last_set_protocol`
通信プロトコル番号を設定します。
```rust
pub fn book_next_set_protocol(&mut self, protocol: i64) { ... }
pub fn read_last_set_protocol(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `protocol: i64` — プロトコル番号

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 使用例

```rust
use rust_computers_api::control_craft::transmitter::*;
use rust_computers_api::peripheral::Peripheral;

let mut tx = Transmitter::find().unwrap();

// プロトコルを設定
tx.book_next_set_protocol(1);
wait_for_next_tick().await;
let _ = tx.read_last_set_protocol();

// 同期リモート呼び出し
tx.book_next_call_remote("my_access_key", "my_context", &[]);
wait_for_next_tick().await;
let result = tx.read_last_call_remote().unwrap();

// 非同期リモート呼び出し
tx.book_next_call_remote_async(
    "my_access_key",
    "my_context",
    "on_result",
    "remote_device",
    "getStatus",
    &[],
);
wait_for_next_tick().await;
let _ = tx.read_last_call_remote_async();
```
