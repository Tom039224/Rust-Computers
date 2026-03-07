//! AdvancedPeripherals ペリフェラル群。
//! AdvancedPeripherals peripheral modules.

pub mod inventory_manager;
pub mod player_detector;

pub use inventory_manager::{ADItemEntry, InventoryManager};
pub use player_detector::{ADPlayerInfo, PlayerDetector};
