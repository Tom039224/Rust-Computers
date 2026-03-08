# Modem

**モジュール:** `computer_craft::modem`  
**ペリフェラルタイプ:** `modem`

CC:Tweaked のモデムペリフェラル。ワイヤレスおよび有線ネットワーク通信に使用します。チャンネルの開閉、メッセージの送信、データの受信をサポートしています。

## Book-Read メソッド

### `book_next_open` / `read_last_open`
指定チャンネルを開いてリスニング状態にします。
```rust
pub fn book_next_open(&mut self, channel: u32) { ... }
pub fn read_last_open(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `channel: u32` — 開くチャンネル番号

**戻り値:** `()`

---

### `book_next_is_open` / `read_last_is_open`
指定チャンネルが開いているか確認します。
```rust
pub fn book_next_is_open(&mut self, channel: u32) { ... }
pub fn read_last_is_open(&self) -> Result<bool, PeripheralError> { ... }
```
**パラメータ:**
- `channel: u32` — 確認するチャンネル番号

**戻り値:** `bool`

---

### `book_next_close` / `read_last_close`
チャンネルを閉じます。
```rust
pub fn book_next_close(&mut self, channel: u32) { ... }
pub fn read_last_close(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `channel: u32` — 閉じるチャンネル番号

**戻り値:** `()`

---

### `book_next_close_all` / `read_last_close_all`
開いている全チャンネルを閉じます。
```rust
pub fn book_next_close_all(&mut self) { ... }
pub fn read_last_close_all(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_transmit` / `read_last_transmit`
serde でシリアライズ可能なペイロードをチャンネルに送信します。
```rust
pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T) { ... }
pub fn read_last_transmit(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `channel: u32` — 送信先チャンネル
- `reply_channel: u32` — 応答用チャンネル
- `payload: &T` — シリアライズ可能なペイロード

**戻り値:** `()`

---

### `book_next_transmit_raw` / `read_last_transmit_raw`
生文字列ペイロードをチャンネルに送信します。
```rust
pub fn book_next_transmit_raw(&mut self, channel: u32, reply_channel: u32, payload: &str) { ... }
pub fn read_last_transmit_raw(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `channel: u32` — 送信先チャンネル
- `reply_channel: u32` — 応答用チャンネル
- `payload: &str` — 文字列ペイロード

**戻り値:** `()`

---

### `book_next_try_receive_raw` / `read_last_try_receive_raw`
1tick 以内にメッセージの受信を試みます。データが届かなければ `None` を返します。
```rust
pub fn book_next_try_receive_raw(&mut self) { ... }
pub fn read_last_try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError> { ... }
```
**戻り値:** `Option<ReceiveData<String>>`

## イベント待機メソッド

### `receive_wait_raw`
メッセージを受信するまで待機します（非同期）。毎 tick ポーリングしてデータが届くまでループします。
```rust
pub async fn receive_wait_raw(&self) -> Result<ReceiveData<String>, PeripheralError> { ... }
```
**戻り値:** `ReceiveData<String>`

## 型定義

### `ReceiveData<T>`
受信メッセージデータ。
```rust
pub struct ReceiveData<T> {
    pub channel: u32,
    pub reply_channel: u32,
    pub payload: T,
    pub distance: u32,
}
```

## 使用例

```rust
use rust_computers_api::computer_craft::modem::*;
use rust_computers_api::peripheral::Peripheral;

let mut modem = Modem::find().unwrap();

// チャンネル1を開く
modem.book_next_open(1);
wait_for_next_tick().await;
let _ = modem.read_last_open();

// 文字列メッセージを送信
modem.book_next_transmit_raw(1, 1, "hello");
wait_for_next_tick().await;
let _ = modem.read_last_transmit_raw();

// メッセージ受信を待機（非同期）
let msg = modem.receive_wait_raw().await.unwrap();
println!("受信: {} 距離: {}", msg.payload, msg.distance);
```
