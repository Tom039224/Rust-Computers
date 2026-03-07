//! Some-Peripherals GoggleLinkPort。

use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// GoggleLinkPort ペリフェラル。
pub struct GoggleLinkPort {
    addr: PeriphAddr,
}

impl Peripheral for GoggleLinkPort {
    const NAME: &'static str = "sp:goggle_link_port";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl GoggleLinkPort {
    /// 接続中の Goggles 一覧を取得する。
    pub async fn get_connected(
        &self,
    ) -> Result<BTreeMap<String, crate::msgpack::Value>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "getConnected",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }
}
