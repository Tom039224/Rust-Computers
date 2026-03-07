//! Some-Peripherals ペリフェラル群。
//! Some-Peripherals peripheral modules.

pub mod ballistic_accelerator;
pub mod digitizer;
pub mod goggle_link_port;
pub mod radar;
pub mod raycaster;
pub mod world_scanner;

pub use ballistic_accelerator::{
    BallisticAccelerator, SPCoordinate, SPDroneBlueprint, SPPitchResult, SPTimeResult,
};
pub use digitizer::{Digitizer, SPDigitizedItem, SPItemData};
pub use goggle_link_port::GoggleLinkPort;
pub use radar::{Radar, SPEntityInfo, SPShipInfo};
pub use raycaster::{Raycaster, SPRaycastResult};
pub use world_scanner::{SPBlockInfo, WorldScanner};
