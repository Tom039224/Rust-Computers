//! Create Display Link ペリフェラル。
//! Create Display Link peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// Display Link ペリフェラル。
pub struct DisplayLink {
    dir: Direction,
}

impl Peripheral for DisplayLink {
    const NAME: &'static str = "create:display_link";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl DisplayLink {
    /// カーソル位置を設定する。
    pub async fn set_cursor_pos(&self, x: u32, y: u32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(x as i32), msgpack::int(y as i32)]);
        peripheral::do_action(self.dir, "setCursorPos", &args).await?;
        Ok(())
    }

    /// カーソル位置を取得する。
    pub async fn get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getCursorPos",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// カーソル位置を即時取得する (imm 対応)。
    pub fn get_cursor_pos_imm(&self) -> Result<(u32, u32), PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getCursorPos",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ディスプレイのサイズを取得する (mainThread=true のため imm 非対応)。
    pub async fn get_size(&self) -> Result<(u32, u32), PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getSize",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// カラー対応かどうかを取得する。
    pub async fn is_color(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "isColour",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// カラー対応かどうかを即時取得する (imm 対応)。
    pub fn is_color_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "isColour",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// テキストを書き込む。
    pub async fn write(&self, text: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(text)]);
        peripheral::do_action(self.dir, "write", &args).await?;
        Ok(())
    }

    /// バイト列を書き込む。
    pub async fn write_bytes(&self, data: &[u8]) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(&data)?;
        let args = msgpack::array(&[encoded]);
        peripheral::do_action(self.dir, "writeBytes", &args).await?;
        Ok(())
    }

    /// 現在の行をクリアする。
    pub async fn clear_line(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "clearLine", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// ディスプレイ全体をクリアする。
    pub async fn clear(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "clear", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// ディスプレイを更新する。
    pub async fn update(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "update", &msgpack::array(&[])).await?;
        Ok(())
    }
}
