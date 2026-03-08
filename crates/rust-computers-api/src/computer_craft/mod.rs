//! CC:Tweaked ペリフェラル群。
//! CC:Tweaked peripheral modules.

pub mod inventory;
pub mod modem;
pub mod monitor;
pub mod speaker;

pub use inventory::{Inventory, ItemDetail, SlotInfo};
pub use modem::{Modem, ReceiveData};
pub use monitor::{Monitor, MonitorColor, MonitorPosition, MonitorSize, MonitorTextScale, TouchEvent};
pub use speaker::{Speaker, SpeakerInstrument};
