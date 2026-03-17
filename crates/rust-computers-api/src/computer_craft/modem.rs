//! CC:Tweaked Modem ペリフェラル。
//! CC:Tweaked Modem peripheral.

use alloc::string::String;
use alloc::vec::Vec;

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
    pub fn read_last_open(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "open")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
    pub async fn async_open(&mut self, channel: u32) -> Result<(), PeripheralError> {
        self.book_next_open(channel);
        crate::wait_for_next_tick().await;
        self.read_last_open()
            .into_iter()
            .next()
            .unwrap_or(Ok(()))
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
    pub async fn async_is_open(&mut self, channel: u32) -> Result<bool, PeripheralError> {
        self.book_next_is_open(channel);
        crate::wait_for_next_tick().await;
        self.read_last_is_open()
    }

    /// チャンネルを閉じる。
    /// Close a channel.
    pub fn book_next_close(&mut self, channel: u32) {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::book_action(self.addr, "close", &args);
    }
    pub fn read_last_close(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "close")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
    pub async fn async_close(&mut self, channel: u32) -> Result<(), PeripheralError> {
        self.book_next_close(channel);
        crate::wait_for_next_tick().await;
        self.read_last_close()
            .into_iter()
            .next()
            .unwrap_or(Ok(()))
    }

    /// 全チャンネルを閉じる。
    /// Close all channels.
    pub fn book_next_close_all(&mut self) {
        peripheral::book_action(self.addr, "closeAll", &msgpack::array(&[]));
    }
    pub fn read_last_close_all(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "closeAll")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
    pub async fn async_close_all(&mut self) -> Result<(), PeripheralError> {
        self.book_next_close_all();
        crate::wait_for_next_tick().await;
        self.read_last_close_all()
            .into_iter()
            .next()
            .unwrap_or(Ok(()))
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
    pub fn read_last_transmit(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "transmit")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
    pub async fn async_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T) -> Result<(), PeripheralError> {
        self.book_next_transmit(channel, reply_channel, payload);
        crate::wait_for_next_tick().await;
        self.read_last_transmit()
            .into_iter()
            .next()
            .unwrap_or(Ok(()))
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
    pub fn read_last_transmit_raw(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "transmit")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
    pub async fn async_transmit_raw(&mut self, channel: u32, reply_channel: u32, payload: &str) -> Result<(), PeripheralError> {
        self.book_next_transmit_raw(channel, reply_channel, payload);
        crate::wait_for_next_tick().await;
        self.read_last_transmit_raw()
            .into_iter()
            .next()
            .unwrap_or(Ok(()))
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
    pub async fn async_try_receive_raw(&mut self) -> Result<Option<ReceiveData<String>>, PeripheralError> {
        self.book_next_try_receive_raw();
        crate::wait_for_next_tick().await;
        self.read_last_try_receive_raw()
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

    /// ワイヤレスモデムかどうかを確認する。
    /// Check if this modem is wireless.
    pub fn book_next_is_wireless(&mut self) {
        peripheral::book_request(self.addr, "isWireless", &msgpack::array(&[]));
    }
    pub fn read_last_is_wireless(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isWireless")?;
        peripheral::decode(&data)
    }
    pub async fn async_is_wireless(&mut self) -> Result<bool, PeripheralError> {
        self.book_next_is_wireless();
        crate::wait_for_next_tick().await;
        self.read_last_is_wireless()
    }

    /// 有線ネットワーク上のペリフェラル名を取得する。
    /// Get names of peripherals on the wired network.
    pub fn book_next_get_names_remote(&mut self) {
        peripheral::book_request(self.addr, "getNamesRemote", &msgpack::array(&[]));
    }
    pub fn read_last_get_names_remote(&self) -> Result<Vec<String>, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getNamesRemote")?;
        peripheral::decode(&data)
    }
    pub async fn async_get_names_remote(&mut self) -> Result<Vec<String>, PeripheralError> {
        self.book_next_get_names_remote();
        crate::wait_for_next_tick().await;
        self.read_last_get_names_remote()
    }
}
