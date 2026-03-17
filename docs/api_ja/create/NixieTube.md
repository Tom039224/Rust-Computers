# NixieTube

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:nixie_tube`

Create Nixie Tube ペリフェラル。ニキシー管のテキスト、カラー、シグナル表示を制御します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_set_text` / `read_last_set_text`

表示テキストを設定します。オプションでカラーを指定できます。

```rust
pub fn book_next_set_text(&mut self, text: &str, colour: Option<&str>)
pub fn read_last_set_text(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `text` | `&str` | 表示するテキスト |
| `colour` | `Option<&str>` | オプションのカラー名（例: `"red"`, `"blue"`） |

### `book_next_set_text_colour` / `read_last_set_text_colour`

テキストカラーを設定します。

```rust
pub fn book_next_set_text_colour(&mut self, colour: &str)
pub fn read_last_set_text_colour(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `colour` | `&str` | 設定するカラー名 |

### `book_next_set_signal` / `read_last_set_signal`

シグナル表示パラメータを設定します。front（前面）は必須、back（背面）はオプションです。

```rust
pub fn book_next_set_signal(&mut self, front: &CRSignalParams, back: Option<&CRSignalParams>) -> Result<(), PeripheralError>
pub fn read_last_set_signal(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `front` | `&CRSignalParams` | 前面シグナルパラメータ |
| `back` | `Option<&CRSignalParams>` | オプションの背面シグナルパラメータ |

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 型定義

### `CRSignalParams`

```rust
pub struct CRSignalParams {
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
    pub glow_width: Option<u8>,
    pub glow_height: Option<u8>,
    pub blink_period: Option<u8>,
    pub blink_off_time: Option<u8>,
}
```

## 使用例

```rust
use rust_computers_api::create::nixie_tube::NixieTube;
use rust_computers_api::create::common::CRSignalParams;
use rust_computers_api::peripheral::Peripheral;

let mut nixie = NixieTube::wrap(addr);

// カラー付きテキストを設定
nixie.book_next_set_text("42", Some("orange"));
wait_for_next_tick().await;
nixie.read_last_set_text()?;

// シグナル表示を設定
let front = CRSignalParams {
    r: Some(255), g: Some(128), b: Some(0),
    glow_width: None, glow_height: None,
    blink_period: None, blink_off_time: None,
};
nixie.book_next_set_signal(&front, None)?;
wait_for_next_tick().await;
nixie.read_last_set_signal()?;
```
