# SpatialAnchor

**モジュール:** `control_craft::spatial_anchor`  
**ペリフェラルタイプ:** `controlcraft:spatial_anchor_peripheral`

Control-Craft の SpatialAnchor ペリフェラル。船の静的状態、アンカーの動作状態、オフセット距離、位置/回転PIDゲイン、通信チャンネルを制御します。

## Book-Read メソッド

### セッター

#### `book_next_set_static` / `read_last_set_static`
船を静的（動かない）状態に設定します。
```rust
pub fn book_next_set_static(&mut self, enabled: bool) { ... }
pub fn read_last_set_static(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `enabled: bool` — 船を静的にするか

#### `book_next_set_running` / `read_last_set_running`
アンカーの動作状態を設定します。
```rust
pub fn book_next_set_running(&mut self, enabled: bool) { ... }
pub fn read_last_set_running(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `enabled: bool` — アンカーを動作させるか

#### `book_next_set_offset` / `read_last_set_offset`
アンカーのオフセット距離を設定します。
```rust
pub fn book_next_set_offset(&mut self, offset: f64) { ... }
pub fn read_last_set_offset(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `offset: f64` — オフセット距離

#### `book_next_set_ppid` / `read_last_set_ppid`
位置制御（PPID）のゲインを設定します。
```rust
pub fn book_next_set_ppid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_ppid(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `p: f64` — 比例ゲイン、`i: f64` — 積分ゲイン、`d: f64` — 微分ゲイン

#### `book_next_set_qpid` / `read_last_set_qpid`
回転制御（QPID）のゲインを設定します。
```rust
pub fn book_next_set_qpid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_qpid(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `p: f64` — 比例ゲイン、`i: f64` — 積分ゲイン、`d: f64` — 微分ゲイン

#### `book_next_set_channel` / `read_last_set_channel`
通信チャンネル番号を設定します。
```rust
pub fn book_next_set_channel(&mut self, channel: i64) { ... }
pub fn read_last_set_channel(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `channel: i64` — チャンネル番号

## 使用例

```rust
use rust_computers_api::control_craft::spatial_anchor::*;
use rust_computers_api::peripheral::Peripheral;

let mut anchor = SpatialAnchor::find().unwrap();

// アンカーを設定
anchor.book_next_set_running(true);
anchor.book_next_set_offset(2.0);
anchor.book_next_set_ppid(1.0, 0.1, 0.05);
anchor.book_next_set_qpid(0.5, 0.05, 0.02);
wait_for_next_tick().await;
let _ = anchor.read_last_set_running();
let _ = anchor.read_last_set_offset();
let _ = anchor.read_last_set_ppid();
let _ = anchor.read_last_set_qpid();

// 静的モードを設定
anchor.book_next_set_static(true);
wait_for_next_tick().await;
let _ = anchor.read_last_set_static();
```
