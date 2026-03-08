# GoggleLinkPort

**モジュール:** `some_peripherals`  
**ペリフェラルタイプ:** `sp:goggle_link_port` (Peripheral::NAME)

Some-Peripherals Mod の GoggleLinkPort ペリフェラル。接続中のゴーグル情報にアクセスできます。

## Book-Read メソッド

### `book_next_get_connected` / `read_last_get_connected`

現在接続中のゴーグル一覧を取得します。

```rust
pub fn book_next_get_connected(&mut self) { ... }
pub fn read_last_get_connected(
    &self,
) -> Result<BTreeMap<String, msgpack::Value>, PeripheralError> { ... }
```

**パラメータ:** なし

**戻り値:** `Result<BTreeMap<String, msgpack::Value>, PeripheralError>` — 接続中のゴーグルのマップ（動的な値を持つキー・値ペア）

## 使用例

```rust
// 接続中のゴーグルを取得
peripheral.book_next_get_connected();
wait_for_next_tick().await;
let connected = peripheral.read_last_get_connected()?;

for (key, value) in &connected {
    log!("接続中: {} = {:?}", key, value);
}
```
