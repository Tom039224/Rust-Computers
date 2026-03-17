//! Some-Peripherals WorldScanner。

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ブロック情報。
/// Block information returned by WorldScanner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPBlockInfo {
    pub block_type: String,
    #[serde(default)]
    pub ship_id: Option<i64>,
}

/// WorldScanner ペリフェラル。
/// WorldScanner peripheral for scanning blocks in the world.
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

impl WorldScanner {
    /// 指定座標のブロック情報を取得する。
    /// Get block information at the specified coordinates.
    ///
    /// # Arguments
    /// * `x` - X 座標
    /// * `y` - Y 座標
    /// * `z` - Z 座標
    /// * `is_shipyard` - Valkyrien Skies の shipyard 座標かどうか
    pub fn book_next_get_block_at(&mut self, x: i32, y: i32, z: i32, is_shipyard: bool) {
        let args = msgpack::array(&[
            msgpack::int(x),
            msgpack::int(y),
            msgpack::int(z),
            msgpack::bool_val(is_shipyard),
        ]);
        peripheral::book_request(self.addr, "getBlockAt", &args);
    }

    pub fn read_last_get_block_at(&self) -> Result<SPBlockInfo, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBlockAt")?;
        peripheral::decode(&data)
    }
}

impl WorldScanner {
    pub async fn async_get_block_at(&mut self, x: i32, y: i32, z: i32, is_shipyard: bool) -> Result<SPBlockInfo, PeripheralError> {
        self.book_next_get_block_at(x, y, z, is_shipyard);
        crate::wait_for_next_tick().await;
        self.read_last_get_block_at()
    }
}
