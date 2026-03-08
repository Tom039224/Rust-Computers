# Speaker

**Module:** `computer_craft`
**Peripheral Type:** `speaker` (the NAME constant)

CC:Tweaked Speaker peripheral. Allows playing note block sounds and Minecraft sound events.

## Methods

### Book/Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_play_note` / `read_last_play_note`

Play a note block sound with the specified instrument.

```rust
pub fn book_next_play_note(
    &mut self,
    instrument: SpeakerInstrument,
    volume: Option<f32>,
    pitch: Option<f32>,
)
pub fn read_last_play_note(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| instrument | `SpeakerInstrument` | The instrument to play |
| volume | `Option<f32>` | Volume level (optional) |
| pitch | `Option<f32>` | Pitch value (optional) |

#### `book_next_play_sound` / `read_last_play_sound`

Play a Minecraft sound event by resource name.

```rust
pub fn book_next_play_sound(
    &mut self,
    name: &str,
    volume: Option<f32>,
    pitch: Option<f32>,
)
pub fn read_last_play_sound(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| name | `&str` | Sound resource name (e.g. `"minecraft:entity.pig.ambient"`) |
| volume | `Option<f32>` | Volume level (optional) |
| pitch | `Option<f32>` | Pitch value (optional) |

#### `book_next_stop` / `read_last_stop`

Stop all current playback.

```rust
pub fn book_next_stop(&mut self)
pub fn read_last_stop(&self) -> Result<(), PeripheralError>
```

## Example

```rust
// Play a harp note
speaker.book_next_play_note(SpeakerInstrument::Harp, Some(1.0), Some(1.0));
wait_for_next_tick().await;
speaker.read_last_play_note().unwrap();

// Play a Minecraft sound
speaker.book_next_play_sound("minecraft:block.note_block.bell", None, None);
wait_for_next_tick().await;
speaker.read_last_play_sound().unwrap();
```

## Types

### SpeakerInstrument

Enum of available note block instrument types.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakerInstrument {
    Harp, Basedrum, Snare, Hat, Bass, Flute, Bell, Guitar,
    Chime, Xylophone, IronXylophone, CowBell, Didgeridoo,
    Bit, Banjo, Pling,
}
```

Each variant maps to a string via `as_str()`:

| Variant | String |
|---------|--------|
| `Harp` | `"harp"` |
| `Basedrum` | `"basedrum"` |
| `Snare` | `"snare"` |
| `Hat` | `"hat"` |
| `Bass` | `"bass"` |
| `Flute` | `"flute"` |
| `Bell` | `"bell"` |
| `Guitar` | `"guitar"` |
| `Chime` | `"chime"` |
| `Xylophone` | `"xylophone"` |
| `IronXylophone` | `"iron_xylophone"` |
| `CowBell` | `"cow_bell"` |
| `Didgeridoo` | `"didgeridoo"` |
| `Bit` | `"bit"` |
| `Banjo` | `"banjo"` |
| `Pling` | `"pling"` |
