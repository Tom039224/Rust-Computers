//! Some-Peripherals WorldScanner。

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::peripheral::{Direction, Peripheral};

/// ブロック情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPBlockInfo {
    pub block_type: String,
    #[serde(default)]
    pub ship_id: Option<i64>,
}

/// WorldScanner ペリフェラル。
pub struct WorldScanner {
    dir: Direction,
}

impl Peripheral for WorldScanner {
    const NAME: &'static str = "sp:world_scanner";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}
