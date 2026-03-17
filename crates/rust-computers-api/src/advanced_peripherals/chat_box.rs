//! AdvancedPeripherals ChatBox。

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// ChatBox ペリフェラル。
pub struct ChatBox {
    addr: PeriphAddr,
}

impl Peripheral for ChatBox {
    const NAME: &'static str = "chat_box";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl ChatBox {
    // ─── sendMessage ─────────────────────────────────────────

    /// 全プレイヤーにメッセージを送信する。
    pub fn book_next_send_message(
        &mut self,
        message: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) {
        let mut args = alloc::vec![msgpack::str(message)];
        if let Some(p) = prefix {
            args.push(msgpack::str(p));
        }
        if let Some(b) = brackets {
            args.push(msgpack::str(b));
        }
        if let Some(c) = color {
            args.push(msgpack::str(c));
        }
        peripheral::book_action(self.addr, "sendMessage", &msgpack::array(&args));
    }
    pub fn read_last_send_message(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "sendMessage")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    // ─── sendFormattedMessage ────────────────────────────────

    /// JSON フォーマットのメッセージを全プレイヤーに送信する。
    pub fn book_next_send_formatted_message(
        &mut self,
        json: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) {
        let mut args = alloc::vec![msgpack::str(json)];
        if let Some(p) = prefix {
            args.push(msgpack::str(p));
        }
        if let Some(b) = brackets {
            args.push(msgpack::str(b));
        }
        if let Some(c) = color {
            args.push(msgpack::str(c));
        }
        peripheral::book_action(self.addr, "sendFormattedMessage", &msgpack::array(&args));
    }
    pub fn read_last_send_formatted_message(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "sendFormattedMessage")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    // ─── sendMessageToPlayer ─────────────────────────────────

    /// 指定プレイヤーにメッセージを送信する。
    pub fn book_next_send_message_to_player(
        &mut self,
        message: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) {
        let mut args = alloc::vec![msgpack::str(message), msgpack::str(player)];
        if let Some(p) = prefix {
            args.push(msgpack::str(p));
        }
        if let Some(b) = brackets {
            args.push(msgpack::str(b));
        }
        if let Some(c) = color {
            args.push(msgpack::str(c));
        }
        peripheral::book_action(self.addr, "sendMessageToPlayer", &msgpack::array(&args));
    }
    pub fn read_last_send_message_to_player(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "sendMessageToPlayer")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    // ─── sendFormattedMessageToPlayer ────────────────────────

    /// JSON フォーマットのメッセージを指定プレイヤーに送信する。
    pub fn book_next_send_formatted_message_to_player(
        &mut self,
        json: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) {
        let mut args = alloc::vec![msgpack::str(json), msgpack::str(player)];
        if let Some(p) = prefix {
            args.push(msgpack::str(p));
        }
        if let Some(b) = brackets {
            args.push(msgpack::str(b));
        }
        if let Some(c) = color {
            args.push(msgpack::str(c));
        }
        peripheral::book_action(
            self.addr,
            "sendFormattedMessageToPlayer",
            &msgpack::array(&args),
        );
    }
    pub fn read_last_send_formatted_message_to_player(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "sendFormattedMessageToPlayer")?;
        peripheral::decode(&data)
    }

    // ─── sendToastToPlayer ───────────────────────────────────

    /// 指定プレイヤーにトースト通知を送信する。
    pub fn book_next_send_toast_to_player(
        &mut self,
        title: &str,
        subtitle: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) {
        let mut args = alloc::vec![
            msgpack::str(title),
            msgpack::str(subtitle),
            msgpack::str(player),
        ];
        if let Some(p) = prefix {
            args.push(msgpack::str(p));
        }
        if let Some(b) = brackets {
            args.push(msgpack::str(b));
        }
        if let Some(c) = color {
            args.push(msgpack::str(c));
        }
        peripheral::book_action(self.addr, "sendToastToPlayer", &msgpack::array(&args));
    }
    pub fn read_last_send_toast_to_player(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "sendToastToPlayer")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    // ─── sendFormattedToastToPlayer ──────────────────────────

    /// JSON フォーマットのトースト通知を指定プレイヤーに送信する。
    pub fn book_next_send_formatted_toast_to_player(
        &mut self,
        json_title: &str,
        json_subtitle: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) {
        let mut args = alloc::vec![
            msgpack::str(json_title),
            msgpack::str(json_subtitle),
            msgpack::str(player),
        ];
        if let Some(p) = prefix {
            args.push(msgpack::str(p));
        }
        if let Some(b) = brackets {
            args.push(msgpack::str(b));
        }
        if let Some(c) = color {
            args.push(msgpack::str(c));
        }
        peripheral::book_action(
            self.addr,
            "sendFormattedToastToPlayer",
            &msgpack::array(&args),
        );
    }
    pub fn read_last_send_formatted_toast_to_player(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "sendFormattedToastToPlayer")?;
        peripheral::decode(&data)
    }
}

impl ChatBox {
    // ─── async_* バリアント ──────────────────────────────────

    pub async fn async_send_message(
        &mut self,
        message: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) -> Vec<Result<bool, PeripheralError>> {
        self.book_next_send_message(message, prefix, brackets, color);
        crate::wait_for_next_tick().await;
        self.read_last_send_message()
    }

    pub async fn async_send_formatted_message(
        &mut self,
        json: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) -> Vec<Result<bool, PeripheralError>> {
        self.book_next_send_formatted_message(json, prefix, brackets, color);
        crate::wait_for_next_tick().await;
        self.read_last_send_formatted_message()
    }

    pub async fn async_send_message_to_player(
        &mut self,
        message: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) -> Vec<Result<bool, PeripheralError>> {
        self.book_next_send_message_to_player(message, player, prefix, brackets, color);
        crate::wait_for_next_tick().await;
        self.read_last_send_message_to_player()
    }

    pub async fn async_send_formatted_message_to_player(
        &mut self,
        json: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) -> Result<bool, PeripheralError> {
        self.book_next_send_formatted_message_to_player(json, player, prefix, brackets, color);
        crate::wait_for_next_tick().await;
        self.read_last_send_formatted_message_to_player()
    }

    pub async fn async_send_toast_to_player(
        &mut self,
        title: &str,
        subtitle: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) -> Vec<Result<bool, PeripheralError>> {
        self.book_next_send_toast_to_player(title, subtitle, player, prefix, brackets, color);
        crate::wait_for_next_tick().await;
        self.read_last_send_toast_to_player()
    }

    pub async fn async_send_formatted_toast_to_player(
        &mut self,
        json_title: &str,
        json_subtitle: &str,
        player: &str,
        prefix: Option<&str>,
        brackets: Option<&str>,
        color: Option<&str>,
    ) -> Result<bool, PeripheralError> {
        self.book_next_send_formatted_toast_to_player(json_title, json_subtitle, player, prefix, brackets, color);
        crate::wait_for_next_tick().await;
        self.read_last_send_formatted_toast_to_player()
    }
}
