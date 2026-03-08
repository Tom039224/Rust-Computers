//! CC:Tweaked Speaker ペリフェラル。
//! CC:Tweaked Speaker peripheral.

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// スピーカー楽器。
/// Speaker instrument types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakerInstrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
}

impl SpeakerInstrument {
    /// 楽器名文字列を返す。
    /// Return the instrument name string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Harp => "harp",
            Self::Basedrum => "basedrum",
            Self::Snare => "snare",
            Self::Hat => "hat",
            Self::Bass => "bass",
            Self::Flute => "flute",
            Self::Bell => "bell",
            Self::Guitar => "guitar",
            Self::Chime => "chime",
            Self::Xylophone => "xylophone",
            Self::IronXylophone => "iron_xylophone",
            Self::CowBell => "cow_bell",
            Self::Didgeridoo => "didgeridoo",
            Self::Bit => "bit",
            Self::Banjo => "banjo",
            Self::Pling => "pling",
        }
    }
}

/// スピーカーペリフェラル。
/// Speaker peripheral.
pub struct Speaker {
    addr: PeriphAddr,
}

impl Peripheral for Speaker {
    const NAME: &'static str = "speaker";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Speaker {
    /// ノート音を再生する。
    /// Play a note.
    pub fn book_next_play_note(
        &mut self,
        instrument: SpeakerInstrument,
        volume: Option<f32>,
        pitch: Option<f32>,
    ) {
        let mut args = alloc::vec![msgpack::str(instrument.as_str())];
        if let Some(v) = volume {
            args.push(msgpack::float64(v as f64));
            if let Some(p) = pitch {
                args.push(msgpack::float64(p as f64));
            }
        } else if let Some(p) = pitch {
            args.push(msgpack::nil());
            args.push(msgpack::float64(p as f64));
        }
        peripheral::book_action(self.addr, "playNote", &msgpack::array(&args));
    }
    pub fn read_last_play_note(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "playNote")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// サウンドを再生する。
    /// Play a sound.
    pub fn book_next_play_sound(
        &mut self,
        name: &str,
        volume: Option<f32>,
        pitch: Option<f32>,
    ) {
        let mut args = alloc::vec![msgpack::str(name)];
        if let Some(v) = volume {
            args.push(msgpack::float64(v as f64));
            if let Some(p) = pitch {
                args.push(msgpack::float64(p as f64));
            }
        } else if let Some(p) = pitch {
            args.push(msgpack::nil());
            args.push(msgpack::float64(p as f64));
        }
        peripheral::book_action(self.addr, "playSound", &msgpack::array(&args));
    }
    pub fn read_last_play_sound(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "playSound")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 再生を停止する。
    /// Stop playback.
    pub fn book_next_stop(&mut self) {
        peripheral::book_action(self.addr, "stop", &msgpack::array(&[]));
    }
    pub fn read_last_stop(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "stop")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
