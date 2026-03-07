//! Create Sequenced Gearshift ペリフェラル。
//! Create Sequenced Gearshift peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// Sequenced Gearshift ペリフェラル。
pub struct SequencedGearshift {
    dir: Direction,
}

impl Peripheral for SequencedGearshift {
    const NAME: &'static str = "create:sequenced_gearshift";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl SequencedGearshift {
    /// 指定量だけ回転させる。
    pub async fn rotate(
        &self,
        amount: i32,
        speed_modifier: Option<i32>,
    ) -> Result<(), PeripheralError> {
        let mut args_vec = alloc::vec![msgpack::int(amount)];
        if let Some(sm) = speed_modifier {
            args_vec.push(msgpack::int(sm));
        }
        let args = msgpack::array(&args_vec);
        peripheral::do_action(self.dir, "rotate", &args).await?;
        Ok(())
    }

    /// 指定距離だけ移動させる。
    pub async fn move_by(
        &self,
        distance: i32,
        speed_modifier: Option<i32>,
    ) -> Result<(), PeripheralError> {
        let mut args_vec = alloc::vec![msgpack::int(distance)];
        if let Some(sm) = speed_modifier {
            args_vec.push(msgpack::int(sm));
        }
        let args = msgpack::array(&args_vec);
        peripheral::do_action(self.dir, "moveBy", &args).await?;
        Ok(())
    }

    /// 現在動作中かどうかを取得する。
    pub async fn is_running(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "isRunning",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// 現在動作中かどうかを即時取得する (imm 対応)。
    pub fn is_running_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "isRunning",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
