//! CC:Tweaked ペリフェラル群。
//! CC:Tweaked peripheral modules.

pub mod inventory;
pub mod modem;
pub mod monitor;
pub mod speaker;

pub use inventory::{Inventory, ItemDetail, SlotInfo};
pub use modem::{Modem, ReceiveData, WiredModem, WirelessModem};
pub use monitor::{
    AdvancedMonitor, Monitor, MonitorColor, MonitorPosition, MonitorSize, MonitorTextScale,
    NormalMonitor,
};
pub use speaker::{Speaker, SpeakerInstrument};
