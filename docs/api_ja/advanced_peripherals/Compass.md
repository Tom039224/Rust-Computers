# Compass (APCompass)

**モジュール:** `advanced_peripherals::compass`  
**ペリフェラルタイプ:** `advancedPeripherals:compass`

AdvancedPeripherals コンパス ペリフェラル（タートルアップグレード）。タートルが向いている方向を返します。

> **注意:** Rust構造体名は `Compass` です。他のコンパス型との名前衝突を避けるために `APCompass` をエイリアスとして使用してください。

## ブックリードメソッド

### `book_next_get_facing` / `read_last_get_facing`
タートルが向いている方向を取得します。
```rust
pub fn book_next_get_facing(&mut self)
pub fn read_last_get_facing(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String` — `"north"`、`"south"`、`"east"`、`"west"` のいずれか

## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::advanced_peripherals::Compass;
use rust_computers_api::peripheral::Peripheral;

let mut compass = Compass::wrap(addr);

loop {
    let facing = compass.read_last_get_facing();

    compass.book_next_get_facing();
    wait_for_next_tick().await;
}
```
