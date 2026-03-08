//! CC:Tweaked Modem ペリフェラル。
//! CC:Tweaked Modem peripheral.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// 受信データ。
/// Received data wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveData<T> {
    pub channel: u32,
    pub reply_channel: u32,
    pub payload: T,
    pub distance: u32,
}

/// モデムペリフェラル（ワイヤレス / 有線共通）。
/// Modem peripheral (unified for wireless and wired).
pub struct Modem {
    addr: PeriphAddr,
}

impl Peripheral for Modem {
    const NAME: &'static str = "modem";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Modem {
    /// チャンネルを開く。
    /// Open a channel.
    pub fn book_next_open(&mut self, channel: u32) {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::book_action(self.addr, "open", &args);
    }
    pub fn read_last_open(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "open")?;
        Ok(())
    }

    /// チャンネルが開いているか確認する。
    /// Check if a channel is open.
    pub fn book_next_is_open(&mut self, channel: u32) {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::book_request(self.addr, "isOpen", &args);
    }
    pub fn read_last_is_open(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isOpen")?;
        peripheral::decode(&data)
    }

    /// チャンネルを閉じる。
    /// Close a channel.
    pub fn book_next_close(&mut self, channel: u32) {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::book_action(self.addr, "close", &args);
    }
    pub fn read_last_close(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "close")?;
        Ok(())
    }

    /// 全チャンネルを閉じる。
    /// Close all channels.
    pub fn book_next_close_all(&mut self) {
        peripheral::book_action(self.addr, "closeAll", &msgpack::array(&[]));
    }
    pub fn read_last_close_all(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "closeAll")?;
        Ok(())
    }

    /// serde でシリアライズ可能なペイロードを送信する。
    /// Transmit a serde-serializable payload.
    pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T) {
        if let Ok(payload_bytes) = crate::serde_msgpack::to_bytes(payload) {
            let args = msgpack::array(&[
                msgpack::int(channel as i32),
                msgpack::int(reply_channel as i32),
                payload_bytes,
            ]);
            peripheral::book_action(self.addr, "transmit", &args);
        }
    }
    pub fn read_last_transmit(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "transmit")?;
        Ok(())
    }

    /// 生文字列ペイロードを送信する。
    /// Transmit a raw string payload.
    pub fn book_next_transmit_raw(&mut self, channel: u32, reply_channel: u32, payload: &str) {
        let args = msgpack::array(&[
            msgpack::int(channel as i32),
            msgpack::int(reply_channel as i32),
            msgpack::str(payload),
        ]);
        peripheral::book_action(self.addr, "transmit", &args);
    }
    pub fn read_last_transmit_raw(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "transmit")?;
        Ok(())
    }

    /// 1tick 待機してメッセージを受信する。来なければ None。
    /// Try to receive a message within 1 tick. Returns None if nothing arrives.
    pub fn book_next_try_receive_raw(&mut self) {
        peripheral::book_request(self.addr, "try_pull_modem_message", &msgpack::array(&[]));
    }
    pub fn read_last_try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "try_pull_modem_message")?;
        peripheral::decode(&data)
    }

    /// メッセージ受信を待機する。
    /// Wait until a message is received.
    pub async fn receive_wait_raw(&self) -> Result<ReceiveData<String>, PeripheralError> {
        loop {
            peripheral::book_request(self.addr, "try_pull_modem_message", &crate::msgpack::array(&[]));
            crate::wait_for_next_tick().await;
            let data = peripheral::read_result(self.addr, "try_pull_modem_message")?;
            let result: Option<ReceiveData<String>> = peripheral::decode(&data)?;
            if let Some(msg) = result {
                return Ok(msg);
            }
        }
    }
}
