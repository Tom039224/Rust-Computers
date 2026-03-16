//! Control-Craft Transmitter ペリフェラル。

use alloc::vec::Vec;

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// トランスミッター ペリフェラル。
pub struct Transmitter {
    addr: PeriphAddr,
}

impl Peripheral for Transmitter {
    const NAME: &'static str = "transmitter";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Transmitter {
    // ====== リモート呼び出し ======

    /// 指定アクセスキーとコンテキストでリモートメソッドを同期呼び出しする。
    /// `extra_args` は追加の可変長引数を msgpack エンコード済みのバイト列で渡す。
    pub fn book_next_call_remote(&mut self, access: &str, ctx: &str, extra_args: &[Vec<u8>]) {
        let mut items: Vec<Vec<u8>> = Vec::new();
        items.push(msgpack::str(access));
        items.push(msgpack::str(ctx));
        for arg in extra_args {
            items.push(arg.clone());
        }
        let args = msgpack::array(&items);
        peripheral::book_request(self.addr, "callRemote", &args);
    }
    pub fn read_last_call_remote(&self) -> Result<crate::msgpack::Value, PeripheralError> {
        let data = peripheral::read_result(self.addr, "callRemote")?;
        peripheral::decode(&data)
    }

    /// 非同期でリモートメソッドを呼び出し、結果を `slot_name` イベントで受け取る。
    /// `extra_args` は追加の可変長引数を msgpack エンコード済みのバイト列で渡す。
    pub fn book_next_call_remote_async(
        &mut self,
        access: &str,
        ctx: &str,
        slot_name: &str,
        remote_name: &str,
        method: &str,
        extra_args: &[Vec<u8>],
    ) {
        let mut items: Vec<Vec<u8>> = Vec::new();
        items.push(msgpack::str(access));
        items.push(msgpack::str(ctx));
        items.push(msgpack::str(slot_name));
        items.push(msgpack::str(remote_name));
        items.push(msgpack::str(method));
        for arg in extra_args {
            items.push(arg.clone());
        }
        let args = msgpack::array(&items);
        peripheral::book_action(self.addr, "callRemoteAsync", &args);
    }
    pub fn read_last_call_remote_async(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "callRemoteAsync")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 使用する通信プロトコル番号 (long) を設定する。
    pub fn book_next_set_protocol(&mut self, protocol: i64) {
        let args = msgpack::array(&[msgpack::int64(protocol)]);
        peripheral::book_action(self.addr, "setProtocol", &args);
    }
    pub fn read_last_set_protocol(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setProtocol")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
