//! Some-Peripherals WorldScanner。

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ブロック情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPBlockInfo {
    pub block_type: String,
    #[serde(default)]
    pub ship_id: Option<i64>,
}

/// WorldScanner ペリフェラル。
pub struct WorldScanner {
    addr: PeriphAddr,
}

impl Peripheral for WorldScanner {
    const NAME: &'static str = "sp:world_scanner";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}
