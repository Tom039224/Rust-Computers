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
    pub async fn open(&self, channel: u32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::do_action(self.addr, "open", &args).await?;
        Ok(())
    }

    /// チャンネルが開いているか確認する。
    /// Check if a channel is open.
    pub async fn is_open(&self, channel: u32) -> Result<bool, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        let data = peripheral::request_info(self.addr, "isOpen", &args).await?;
        peripheral::decode(&data)
    }

    /// チャンネルを閉じる。
    /// Close a channel.
    pub async fn close(&self, channel: u32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::do_action(self.addr, "close", &args).await?;
        Ok(())
    }

    /// 全チャンネルを閉じる。
    /// Close all channels.
    pub async fn close_all(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.addr, "closeAll", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// serde でシリアライズ可能なペイロードを送信する。
    /// Transmit a serde-serializable payload.
    pub async fn transmit<T: Serialize>(
        &self,
        channel: u32,
        reply_channel: u32,
        payload: &T,
    ) -> Result<(), PeripheralError> {
        let payload_bytes = crate::serde_msgpack::to_bytes(payload)?;
        let args = msgpack::array(&[
            msgpack::int(channel as i32),
            msgpack::int(reply_channel as i32),
            payload_bytes,
        ]);
        peripheral::do_action(self.addr, "transmit", &args).await?;
        Ok(())
    }

    /// 生文字列ペイロードを送信する。
    /// Transmit a raw string payload.
    pub async fn transmit_raw(
        &self,
        channel: u32,
        reply_channel: u32,
        payload: &str,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::int(channel as i32),
            msgpack::int(reply_channel as i32),
            msgpack::str(payload),
        ]);
        peripheral::do_action(self.addr, "transmit", &args).await?;
        Ok(())
    }

    /// 1tick 待機してメッセージを受信する。来なければ None。
    /// Try to receive a message within 1 tick. Returns None if nothing arrives.
    pub async fn try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError> {
        let data = peripheral::request_info(
            self.addr,
            "try_pull_modem_message",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// メッセージ受信を待機する。
    /// Wait until a message is received.
    pub async fn receive_wait_raw(&self) -> Result<ReceiveData<String>, PeripheralError> {
        loop {
            if let Some(msg) = self.try_receive_raw().await? {
                return Ok(msg);
            }
        }
    }
}
