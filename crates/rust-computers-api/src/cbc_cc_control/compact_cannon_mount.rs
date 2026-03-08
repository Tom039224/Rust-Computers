//! CBC CC Control CompactCannonMount ペリフェラル。
//! CBC CC Control CompactCannonMount peripheral.

use crate::error::PeripheralError;
use alloc::vec::Vec;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// CBC キャノンマウント ペリフェラル。
/// Peripheral for Create Big Cannons cannon mounts.
pub struct CompactCannonMount {
    addr: PeriphAddr,
}

impl Peripheral for CompactCannonMount {
    const NAME: &'static str = "cbc_cannon_mount";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

macro_rules! book_read_imm {
    ($book:ident, $read:ident, $fn_imm:ident, $method:literal, $ret:ty) => {
        pub fn $book(&mut self) {
            peripheral::book_request(self.addr, $method, &crate::msgpack::array(&[]));
        }
        pub fn $read(&self) -> Result<$ret, PeripheralError> {
            let data = peripheral::read_result(self.addr, $method)?;
            peripheral::decode(&data)
        }
        pub fn $fn_imm(&self) -> Result<$ret, PeripheralError> {
            let data =
                peripheral::request_info_imm(self.addr, $method, &crate::msgpack::array(&[]))?;
            peripheral::decode(&data)
        }
    };
}

impl CompactCannonMount {
    // ====== 読み取り系 (imm 対応) ======

    book_read_imm!(book_next_is_running, read_last_is_running, is_running_imm, "isRunning", bool);

    book_read_imm!(book_next_get_yaw, read_last_get_yaw, get_yaw_imm, "getYaw", f64);

    book_read_imm!(book_next_get_pitch, read_last_get_pitch, get_pitch_imm, "getPitch", f64);

    // ====== アクション系 ======

    /// コントラプションを組み立てる (mainThread)。
    /// `isRunning()` の結果（組み立て成功なら `true`）を返す。
    pub fn book_next_assemble(&mut self) {
        peripheral::book_action(self.addr, "assemble", &msgpack::array(&[]));
    }
    pub fn read_last_assemble(&self) -> Vec<Result<bool, PeripheralError>> {
        peripheral::read_action_results(self.addr, "assemble")
            .into_iter()
            .map(|r| match r {
                Ok(data) => peripheral::decode(&data),
                Err(e) => Err(PeripheralError::Bridge(e)),
            })
            .collect()
    }

    /// コントラプションを分解する (mainThread)。
    pub fn book_next_disassemble(&mut self) {
        peripheral::book_action(self.addr, "disassemble", &msgpack::array(&[]));
    }
    pub fn read_last_disassemble(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "disassemble")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ヨー角度 (deg) を設定する (mainThread)。
    pub fn book_next_set_yaw(&mut self, yaw: f64) {
        let args = msgpack::array(&[msgpack::float64(yaw)]);
        peripheral::book_action(self.addr, "setYaw", &args);
    }
    pub fn read_last_set_yaw(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setYaw")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ピッチ角度 (deg) を設定する (mainThread)。
    pub fn book_next_set_pitch(&mut self, pitch: f64) {
        let args = msgpack::array(&[msgpack::float64(pitch)]);
        peripheral::book_action(self.addr, "setPitch", &args);
    }
    pub fn read_last_set_pitch(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setPitch")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// キャノンを発射する（`onRedstoneUpdate()` を呼び出す）。
    pub fn book_next_fire(&mut self) {
        peripheral::book_action(self.addr, "fire", &msgpack::array(&[]));
    }
    pub fn read_last_fire(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "fire")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
