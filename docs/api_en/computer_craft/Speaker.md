# Speaker

**Module:** `computer_craft::speaker`  
**Peripheral Type:** `speaker`

CC:Tweaked Speaker peripheral for playing notes and sounds.

## Book-Read Methods

### `book_next_play_note` / `read_last_play_note`
Play a note with a specified instrument.
```rust
pub fn book_next_play_note(
    &mut self,
    instrument: SpeakerInstrument,
    volume: Option<f32>,
    pitch: Option<f32>,
) { ... }
pub fn read_last_play_note(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `instrument: SpeakerInstrument` — Instrument to use
- `volume: Option<f32>` — Volume level (optional)
- `pitch: Option<f32>` — Pitch value (optional)

**Returns:** `()`

---

### `book_next_play_sound` / `read_last_play_sound`
Play a Minecraft sound by resource name.
```rust
pub fn book_next_play_sound(
    &mut self,
    name: &str,
    volume: Option<f32>,
    pitch: Option<f32>,
) { ... }
pub fn read_last_play_sound(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `name: &str` — Sound resource name (e.g. `"minecraft:block.note_block.harp"`)
- `volume: Option<f32>` — Volume level (optional)
- `pitch: Option<f32>` — Pitch value (optional)

**Returns:** `()`

---

### `book_next_stop` / `read_last_stop`
Stop all current playback.
```rust
pub fn book_next_stop(&mut self) { ... }
pub fn read_last_stop(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

## Types

### `SpeakerInstrument`
Available instrument types for note playback.
```rust
pub enum SpeakerInstrument {
    Harp, Basedrum, Snare, Hat, Bass, Flute, Bell,
    Guitar, Chime, Xylophone, IronXylophone, CowBell,
    Didgeridoo, Bit, Banjo, Pling,
}
```
Each variant has an `as_str()` method returning the instrument name string (e.g. `"harp"`, `"basedrum"`).

## Usage Example

```rust
use rust_computers_api::computer_craft::speaker::*;
use rust_computers_api::peripheral::Peripheral;

let mut speaker = Speaker::find().unwrap();

// Play a harp note at default volume and pitch
speaker.book_next_play_note(SpeakerInstrument::Harp, None, None);
wait_for_next_tick().await;
let _ = speaker.read_last_play_note();

// Play a custom sound
speaker.book_next_play_sound("minecraft:entity.experience_orb.pickup", Some(1.0), Some(1.0));
wait_for_next_tick().await;
let _ = speaker.read_last_play_sound();

// Stop playback
speaker.book_next_stop();
wait_for_next_tick().await;
let _ = speaker.read_last_stop();
```
