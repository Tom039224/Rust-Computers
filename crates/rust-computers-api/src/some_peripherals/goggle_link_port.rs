//! Some-Peripherals GoggleLinkPort。

use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// GoggleLinkPort ペリフェラル。
pub struct GoggleLinkPort {
    dir: Direction,
}

impl Peripheral for GoggleLinkPort {
    const NAME: &'static str = "sp:goggle_link_port";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl GoggleLinkPort {
    /// 接続中の Goggles 一覧を取得する。
    pub async fn get_connected(
        &self,
    ) -> Result<BTreeMap<String, crate::msgpack::Value>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getConnected",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }
}
