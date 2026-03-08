# Speaker

**モジュール:** `computer_craft::speaker`  
**ペリフェラルタイプ:** `speaker`

CC:Tweaked のスピーカーペリフェラル。ノート音やサウンドの再生に使用します。

## Book-Read メソッド

### `book_next_play_note` / `read_last_play_note`
指定した楽器でノート音を再生します。
```rust
pub fn book_next_play_note(
    &mut self,
    instrument: SpeakerInstrument,
    volume: Option<f32>,
    pitch: Option<f32>,
) { ... }
pub fn read_last_play_note(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `instrument: SpeakerInstrument` — 使用する楽器
- `volume: Option<f32>` — 音量（省略可）
- `pitch: Option<f32>` — ピッチ（省略可）

**戻り値:** `()`

---

### `book_next_play_sound` / `read_last_play_sound`
Minecraft のリソース名でサウンドを再生します。
```rust
pub fn book_next_play_sound(
    &mut self,
    name: &str,
    volume: Option<f32>,
    pitch: Option<f32>,
) { ... }
pub fn read_last_play_sound(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `name: &str` — サウンドリソース名（例: `"minecraft:block.note_block.harp"`）
- `volume: Option<f32>` — 音量（省略可）
- `pitch: Option<f32>` — ピッチ（省略可）

**戻り値:** `()`

---

### `book_next_stop` / `read_last_stop`
現在の再生をすべて停止します。
```rust
pub fn book_next_stop(&mut self) { ... }
pub fn read_last_stop(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

## 型定義

### `SpeakerInstrument`
ノート再生で使用できる楽器の種類。
```rust
pub enum SpeakerInstrument {
    Harp, Basedrum, Snare, Hat, Bass, Flute, Bell,
    Guitar, Chime, Xylophone, IronXylophone, CowBell,
    Didgeridoo, Bit, Banjo, Pling,
}
```
各バリアントは `as_str()` メソッドで楽器名の文字列を返します（例: `"harp"`, `"basedrum"`）。

## 使用例

```rust
use rust_computers_api::computer_craft::speaker::*;
use rust_computers_api::peripheral::Peripheral;

let mut speaker = Speaker::find().unwrap();

// デフォルトの音量とピッチでハープのノートを再生
speaker.book_next_play_note(SpeakerInstrument::Harp, None, None);
wait_for_next_tick().await;
let _ = speaker.read_last_play_note();

// カスタムサウンドを再生
speaker.book_next_play_sound("minecraft:entity.experience_orb.pickup", Some(1.0), Some(1.0));
wait_for_next_tick().await;
let _ = speaker.read_last_play_sound();

// 再生を停止
speaker.book_next_stop();
wait_for_next_tick().await;
let _ = speaker.read_last_stop();
```
