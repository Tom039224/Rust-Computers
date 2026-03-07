//! Some-Peripherals BallisticAccelerator。

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// 3D 座標。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 飛行時間計算結果。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPTimeResult {
    pub ticks: f64,
    pub aux: f64,
}

/// ピッチ角計算結果。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPPitchResult {
    pub pitch: f64,
    pub aux: f64,
}

/// ドローン設計図。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPDroneBlueprint {
    pub cannon: SPCoordinate,
    pub target: SPCoordinate,
    pub speed: f64,
    pub length: f64,
}

/// BallisticAccelerator ペリフェラル。
pub struct BallisticAccelerator {
    addr: PeriphAddr,
}

impl Peripheral for BallisticAccelerator {
    const NAME: &'static str = "sp:ballistic_accelerator";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl BallisticAccelerator {
    /// 飛行時間を計算する (imm 対応)。
    pub async fn time_in_air(
        &self,
        y_proj: f64,
        y_tgt: f64,
        y_vel: f64,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_steps: Option<u32>,
    ) -> Result<SPTimeResult, PeripheralError> {
        let mut args = alloc::vec![
            msgpack::float64(y_proj),
            msgpack::float64(y_tgt),
            msgpack::float64(y_vel),
        ];
        args.push(gravity.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(drag.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(max_steps.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        let data =
            peripheral::request_info(self.addr, "timeInAir", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }

    pub fn time_in_air_imm(
        &self,
        y_proj: f64,
        y_tgt: f64,
        y_vel: f64,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_steps: Option<u32>,
    ) -> Result<SPTimeResult, PeripheralError> {
        let mut args = alloc::vec![
            msgpack::float64(y_proj),
            msgpack::float64(y_tgt),
            msgpack::float64(y_vel),
        ];
        args.push(gravity.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(drag.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(max_steps.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        let data =
            peripheral::request_info_imm(self.addr, "timeInAir", &msgpack::array(&args))?;
        peripheral::decode(&data)
    }

    /// ピッチを試行する (imm 対応)。
    pub async fn try_pitch(
        &self,
        pitch: f64,
        speed: f64,
        length: f64,
        dist: f64,
        cannon: SPCoordinate,
        target: SPCoordinate,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_steps: Option<u32>,
    ) -> Result<(f64, f64, f64), PeripheralError> {
        let args = self.build_try_pitch_args(
            pitch, speed, length, dist, cannon, target, gravity, drag, max_steps,
        );
        let data =
            peripheral::request_info(self.addr, "tryPitch", &msgpack::array(&args)).await?;
        peripheral::decode(&data)
    }

    pub fn try_pitch_imm(
        &self,
        pitch: f64,
        speed: f64,
        length: f64,
        dist: f64,
        cannon: SPCoordinate,
        target: SPCoordinate,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_steps: Option<u32>,
    ) -> Result<(f64, f64, f64), PeripheralError> {
        let args = self.build_try_pitch_args(
            pitch, speed, length, dist, cannon, target, gravity, drag, max_steps,
        );
        let data =
            peripheral::request_info_imm(self.addr, "tryPitch", &msgpack::array(&args))?;
        peripheral::decode(&data)
    }

    fn build_try_pitch_args(
        &self,
        pitch: f64,
        speed: f64,
        length: f64,
        dist: f64,
        cannon: SPCoordinate,
        target: SPCoordinate,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_steps: Option<u32>,
    ) -> Vec<Vec<u8>> {
        let cannon_encoded = peripheral::encode(&cannon).unwrap_or_default();
        let target_encoded = peripheral::encode(&target).unwrap_or_default();
        let mut args = alloc::vec![
            msgpack::float64(pitch),
            msgpack::float64(speed),
            msgpack::float64(length),
            msgpack::float64(dist),
            cannon_encoded,
            target_encoded,
        ];
        args.push(gravity.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(drag.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(max_steps.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        args
    }

    /// ピッチ角を計算する (imm 対応)。
    #[allow(clippy::too_many_arguments)]
    pub async fn calculate_pitch(
        &self,
        cannon: SPCoordinate,
        target: SPCoordinate,
        speed: f64,
        length: f64,
        amin: Option<f64>,
        amax: Option<f64>,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_delta_t_error: Option<f64>,
        max_steps: Option<u32>,
        num_iterations: Option<u32>,
        num_elements: Option<u32>,
        check_impossible: Option<bool>,
    ) -> Result<SPPitchResult, PeripheralError> {
        let args = self.build_calc_pitch_args(
            cannon,
            target,
            speed,
            length,
            amin,
            amax,
            gravity,
            drag,
            max_delta_t_error,
            max_steps,
            num_iterations,
            num_elements,
            check_impossible,
        );
        let data = peripheral::request_info(self.addr, "calculatePitch", &msgpack::array(&args))
            .await?;
        peripheral::decode(&data)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn calculate_pitch_imm(
        &self,
        cannon: SPCoordinate,
        target: SPCoordinate,
        speed: f64,
        length: f64,
        amin: Option<f64>,
        amax: Option<f64>,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_delta_t_error: Option<f64>,
        max_steps: Option<u32>,
        num_iterations: Option<u32>,
        num_elements: Option<u32>,
        check_impossible: Option<bool>,
    ) -> Result<SPPitchResult, PeripheralError> {
        let args = self.build_calc_pitch_args(
            cannon,
            target,
            speed,
            length,
            amin,
            amax,
            gravity,
            drag,
            max_delta_t_error,
            max_steps,
            num_iterations,
            num_elements,
            check_impossible,
        );
        let data = peripheral::request_info_imm(
            self.addr,
            "calculatePitch",
            &msgpack::array(&args),
        )?;
        peripheral::decode(&data)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_calc_pitch_args(
        &self,
        cannon: SPCoordinate,
        target: SPCoordinate,
        speed: f64,
        length: f64,
        amin: Option<f64>,
        amax: Option<f64>,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_delta_t_error: Option<f64>,
        max_steps: Option<u32>,
        num_iterations: Option<u32>,
        num_elements: Option<u32>,
        check_impossible: Option<bool>,
    ) -> Vec<Vec<u8>> {
        let cannon_encoded = peripheral::encode(&cannon).unwrap_or_default();
        let target_encoded = peripheral::encode(&target).unwrap_or_default();
        let mut args = alloc::vec![
            cannon_encoded,
            target_encoded,
            msgpack::float64(speed),
            msgpack::float64(length),
        ];
        args.push(amin.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(amax.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(gravity.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(drag.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(max_delta_t_error.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(max_steps.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        args.push(num_iterations.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        args.push(num_elements.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        args.push(
            check_impossible.map_or_else(|| msgpack::nil(), |v| msgpack::bool_val(v)),
        );
        args
    }

    /// バッチでピッチ角を計算する。
    #[allow(clippy::too_many_arguments)]
    pub async fn batch_calculate_pitches(
        &self,
        cannon: SPCoordinate,
        targets: &[SPCoordinate],
        speed: f64,
        length: f64,
        amin: Option<f64>,
        amax: Option<f64>,
        gravity: Option<f64>,
        drag: Option<f64>,
        max_delta_t_error: Option<f64>,
        max_steps: Option<u32>,
        num_iterations: Option<u32>,
        num_elements: Option<u32>,
        check_impossible: Option<bool>,
    ) -> Result<Vec<SPPitchResult>, PeripheralError> {
        let cannon_encoded = peripheral::encode(&cannon).unwrap_or_default();
        let targets_encoded = peripheral::encode(&targets).unwrap_or_default();
        let mut args = alloc::vec![
            cannon_encoded,
            targets_encoded,
            msgpack::float64(speed),
            msgpack::float64(length),
        ];
        args.push(amin.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(amax.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(gravity.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(drag.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(max_delta_t_error.map_or_else(|| msgpack::nil(), |v| msgpack::float64(v)));
        args.push(max_steps.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        args.push(num_iterations.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        args.push(num_elements.map_or_else(|| msgpack::nil(), |v| msgpack::int(v as i32)));
        args.push(
            check_impossible.map_or_else(|| msgpack::nil(), |v| msgpack::bool_val(v)),
        );
        let data = peripheral::request_info(
            self.addr,
            "batchCalculatePitches",
            &msgpack::array(&args),
        )
        .await?;
        peripheral::decode(&data)
    }

    /// ドラッグ係数を取得する (imm 対応)。
    pub async fn get_drag(
        &self,
        base_drag: f64,
        dim_drag_multiplier: f64,
    ) -> Result<f64, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(base_drag),
            msgpack::float64(dim_drag_multiplier),
        ]);
        let data = peripheral::request_info(self.addr, "getDrag", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_drag_imm(
        &self,
        base_drag: f64,
        dim_drag_multiplier: f64,
    ) -> Result<f64, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(base_drag),
            msgpack::float64(dim_drag_multiplier),
        ]);
        let data = peripheral::request_info_imm(self.addr, "getDrag", &args)?;
        peripheral::decode(&data)
    }
}
