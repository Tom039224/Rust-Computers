//! Create Sequenced Gearshift ペリフェラル。
//! Create Sequenced Gearshift peripheral.

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Sequenced Gearshift ペリフェラル。
pub struct SequencedGearshift {
    addr: PeriphAddr,
}

impl Peripheral for SequencedGearshift {
    const NAME: &'static str = "create:sequenced_gearshift";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl SequencedGearshift {
    /// 指定量だけ回転させる。
    pub fn book_next_rotate(
        &mut self,
        amount: i32,
        speed_modifier: Option<i32>,
    ) {
        let mut args_vec = alloc::vec![msgpack::int(amount)];
        if let Some(sm) = speed_modifier {
            args_vec.push(msgpack::int(sm));
        }
        let args = msgpack::array(&args_vec);
        peripheral::book_action(self.addr, "rotate", &args);
    }

    pub fn read_last_rotate(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "rotate")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 指定距離だけ移動させる。
    pub fn book_next_move_by(
        &mut self,
        distance: i32,
        speed_modifier: Option<i32>,
    ) {
        let mut args_vec = alloc::vec![msgpack::int(distance)];
        if let Some(sm) = speed_modifier {
            args_vec.push(msgpack::int(sm));
        }
        let args = msgpack::array(&args_vec);
        peripheral::book_action(self.addr, "moveBy", &args);
    }

    pub fn read_last_move_by(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "moveBy")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 現在動作中かどうかを取得する。
    pub fn book_next_is_running(&mut self) {
        peripheral::book_request(
            self.addr,
            "isRunning",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_running(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isRunning")?;
        peripheral::decode(&data)
    }

    /// 現在動作中かどうかを即時取得する (imm 対応)。
    pub fn is_running_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isRunning",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }
}
