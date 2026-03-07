//! Clockwork CC Compat ペリフェラル群。
//! Clockwork CC Compat peripheral modules.

pub mod boiler;
pub mod gas_engine;

pub use boiler::{Boiler, CLFluidInfo, CLPosition};
pub use gas_engine::GasEngine;
