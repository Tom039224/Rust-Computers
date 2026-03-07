//! CC-VS Drag API。
//! CC-VS Drag API for ship drag/lift control.

use super::ship::VSVector3;
use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

/// Drag API (シップ上に配置されたコンピュータから呼び出す)。
/// Drag API (called from a computer placed on a ship).
pub struct Drag {
    dir: Direction,
}

impl Peripheral for Drag {
    const NAME: &'static str = "vs_drag";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl Drag {
    // ====== 読み取り系 (imm 対応) ======

    /// 抗力ベクトルを取得する。
    pub async fn get_drag_force(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getDragForce",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_drag_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getDragForce",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// 揚力ベクトルを取得する。
    pub async fn get_lift_force(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getLiftForce",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_lift_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getLiftForce",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    // ====== 状態変更系 (allow_op) ======

    /// ドラッグを有効化する。
    pub async fn enable_drag(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "enableDrag", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// ドラッグを無効化する。
    pub async fn disable_drag(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "disableDrag", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// リフトを有効化する。
    pub async fn enable_lift(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "enableLift", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// リフトを無効化する。
    pub async fn disable_lift(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "disableLift", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// 回転ドラッグを有効化する。
    pub async fn enable_rot_drag(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "enableRotDrag", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// 回転ドラッグを無効化する。
    pub async fn disable_rot_drag(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "disableRotDrag", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// 風向を設定する。
    pub async fn set_wind_direction(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
        ]);
        peripheral::do_action(self.dir, "setWindDirection", &args).await?;
        Ok(())
    }

    /// 風速を設定する。
    pub async fn set_wind_speed(&self, speed: f64) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::float64(speed)]);
        peripheral::do_action(self.dir, "setWindSpeed", &args).await?;
        Ok(())
    }

    /// 風インパルスを印加する。
    pub async fn apply_wind_impulse(
        &self,
        x: f64,
        y: f64,
        z: f64,
        speed: f64,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(x),
            msgpack::float64(y),
            msgpack::float64(z),
            msgpack::float64(speed),
        ]);
        peripheral::do_action(self.dir, "applyWindImpulse", &args).await?;
        Ok(())
    }
}
