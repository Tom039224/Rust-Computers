# RustComputers API ドキュメント (v0.1.28)

`rust-computers-api` クレートのすべての公開 API のリファレンスです。

> **注:** このドキュメントは v0.1.28 時点の実装に基づいています。

---

## 目次

- [コアAPI](#コアapi)
  - [PeriphAddr と Direction](#periphaddr-と-direction)
  - [Peripheral トレイト](#peripheral-トレイト)
  - [コア関数](#コア関数)
  - [IO API](#io-api)
  - [並行実行 (parallel!)](#並行実行-parallel)
- [ペリフェラル一覧](#ペリフェラル一覧)
  - [Advanced Peripherals](#advanced-peripherals)
  - [CC:VS (Valkyrien Skies CC Compat)](#ccvs-valkyrien-skies-cc-compat)
  - [Clockwork CC Compat](#clockwork-cc-compat)
  - [CC:Tweaked](#cctweaked)
  - [Control Craft](#control-craft)
  - [Create](#create)
  - [Create Addition](#create-addition)
  - [Some Peripherals](#some-peripherals)
  - [Tom's Peripherals](#toms-peripherals)

---

## コアAPI

### PeriphAddr と Direction

ペリフェラルの識別子。`periph_id: u32` のラッパー型。

```rust
pub struct PeriphAddr(pub u32);
// periph_id 0–5: 直接接続（Direction に対応）
// periph_id 6+:  有線モデム経由の接続（将来追加予定）
```

| メソッド | 説明 |
|---|---|
| `PeriphAddr::from_raw(id: u32) -> Self` | raw periph_id から作成 |
| `PeriphAddr::from_dir(dir: Direction) -> Self` | Direction から作成 |
| `.raw() -> u32` | 内部の periph_id を取得 |
| `.as_direction() -> Option<Direction>` | 直接接続なら Direction、有線モデムなら None |
| `.is_direct() -> bool` | 直接接続（periph_id 0–5）かどうか |

`Direction` から `PeriphAddr` への暗黙の変換（`From` トレイト）が実装されています。
ほとんどの関数は `impl Into<PeriphAddr>` を受け取るため、`Direction::North` を直接渡すことができます。

```rust
pub enum Direction {
    Down  = 0,
    Up    = 1,
    North = 2,
    South = 3,
    West  = 4,
    East  = 5,
}

impl Direction {
    pub fn from_id(id: u32) -> Option<Self>
    pub fn id(self) -> u32
}
```

---

### Peripheral トレイト

すべてのペリフェラル構造体が実装するトレイト。

```rust
pub trait Peripheral: Sized {
    /// ペリフェラル型文字列（Java 側の getTypeName() と一致する）
    const NAME: &'static str;

    fn new(addr: PeriphAddr) -> Self;
    fn periph_addr(&self) -> PeriphAddr;

    /// デフォルト実装: periph_addr が方向ベースの場合は Some(Direction)、有線モデムなら None
    fn direction(&self) -> Option<Direction> { self.periph_addr().as_direction() }
}
```

---

### コア関数

`use rc::{peripheral, ...};` でインポートします。

#### 非同期（1 tick 以上かかる）

```rust
/// ペリフェラル情報を非同期取得
pub fn request_info(
    addr: impl Into<PeriphAddr>,
    method_name: &str,
    args: &[u8],
) -> impl Future<Output = Result<Vec<u8>, BridgeError>>

/// ペリフェラルアクションを非同期実行
pub fn do_action(
    addr: impl Into<PeriphAddr>,
    method_name: &str,
    args: &[u8],
) -> impl Future<Output = Result<Vec<u8>, BridgeError>>

/// 指定アドレスのペリフェラルを取得（存在確認付き）
pub async fn wrap<T: Peripheral>(
    addr: impl Into<PeriphAddr>,
) -> Result<T, PeripheralError>
```

#### 即時（同一 tick 内で完結、`@LuaFunction(immediate=true)` 専用）

```rust
/// 即時ペリフェラル情報取得
pub fn request_info_imm(
    addr: impl Into<PeriphAddr>,
    method_name: &str,
    args: &[u8],
) -> Result<Vec<u8>, BridgeError>

/// 即時ペリフェラル取得
pub fn wrap_imm<T: Peripheral>(
    addr: impl Into<PeriphAddr>,
) -> Result<T, PeripheralError>

/// 型名に一致するすべてのペリフェラルを即時検索
/// （CC:Tweaked の peripheral.find() 相当）
pub fn find_imm<T: Peripheral>() -> Vec<T>
```

#### ユーティリティ

```rust
pub fn decode<'de, T: Deserialize<'de>>(data: &[u8]) -> Result<T, PeripheralError>
pub fn encode<T: Serialize>(val: &T) -> Result<Vec<u8>, PeripheralError>

/// ペリフェラルメソッド名の CRC32 ハッシュ
pub fn method_id(name: &str) -> u32

/// 指定 Mod が利用可能かチェック
pub fn is_mod_available(mod_name: &str) -> bool
```

---

### IO API

`use rc::io;` または `use rc::{println, ...};` でインポートします。

```rust
/// GUI ログ欄に文字列を出力
pub fn log_str(s: &str)

/// println! マクロ（GUI ログ欄に出力）
macro_rules! println { ... }

/// eprintln! マクロ（将来の stderr 用、現在は println! と同じ）
macro_rules! eprintln { ... }

/// Enter キーが押されるまで非同期待機して1行読み込む
pub async fn read_line() -> String

/// このコンピューターの ID を取得
pub fn computer_id() -> i32

/// 指定 Mod ID が利用可能かチェック
pub fn is_mod_available(mod_id: u16) -> bool
```

---

### 並行実行 (parallel!)

複数の非同期 Future を同時に発行し、すべての結果を待機します（`join!` に相当）。

```rust
macro_rules! parallel {
    ($a:expr, $b:expr)                             // 2 個: Join2
    ($a:expr, $b:expr, $c:expr)                    // 3 個: Join3
    ($a:expr, $b:expr, $c:expr, $d:expr)           // 4 個: Join4
    // ... 最大 8 個まで対応
}
```

**使用例:**

```rust
let (speed, stress) = rc::parallel!(
    motor.get_speed(),
    motor.get_stress_capacity(),
).await;
```

---

### エントリーポイント

`entry!` マクロで `main` 関数を登録します。

```rust
macro_rules! entry { ($main_fn:ident) => { ... } }
```

**使用例:**

```rust
async fn main() {
    println!("Hello, RustComputers!");
}
entrypoint!(main);
```

---

## ペリフェラル一覧

### メソッドの命名規則

| パターン | 意味 |
|---|---|
| `async fn foo()` | 非同期（1 tick 以上かかる通常 API） |
| `fn foo_imm()` | 即時（同 tick 完結、`@LuaFunction(immediate=true)` のみ） |
| `async fn try_pull_xxx()` | イベントポーリング（1 tick 待機、なければ `None`） |
| `async fn pull_xxx()` | イベント到着まで無限ループ待機 |

---

### Advanced Peripherals

#### InventoryManager

```rust
// NAME = "advancedPeripherals:inventory_manager"
// use rc::advanced_peripherals::InventoryManager;
```

**型:**
```rust
struct ADItemEntry {
    name: String,
    tags: Vec<String>,
    count: u32,
    display_name: String,
    max_stack_size: u32,
    components: Value,
    fingerprint: String,
    slot: Option<u32>,
}
```

**メソッド:**
```rust
async fn get_owner(&self) -> Result<String, PeripheralError>
fn     get_owner_imm(&self) -> Result<String, PeripheralError>
async fn add_item_to_player(&self, slot: u32, count: Option<u32>) -> Result<u32, PeripheralError>
async fn remove_item_from_player(&self, slot: u32, count: Option<u32>) -> Result<u32, PeripheralError>
async fn list(&self) -> Result<Vec<ADItemEntry>, PeripheralError>
async fn get_armor(&self) -> Result<Vec<ADItemEntry>, PeripheralError>
async fn is_player_equipped(&self) -> Result<bool, PeripheralError>
async fn is_wearing(&self, slot: u32) -> Result<bool, PeripheralError>
async fn get_item_in_hand(&self) -> Result<ADItemEntry, PeripheralError>
async fn get_item_in_off_hand(&self) -> Result<ADItemEntry, PeripheralError>
async fn get_empty_space(&self) -> Result<u32, PeripheralError>
async fn is_space_available(&self) -> Result<bool, PeripheralError>
async fn get_free_slot(&self) -> Result<i32, PeripheralError>
async fn list_chest(&self) -> Result<Vec<ADItemEntry>, PeripheralError>
```

---

#### PlayerDetector

```rust
// NAME = "advancedPeripherals:player_detector"
// use rc::advanced_peripherals::PlayerDetector;
```

**型:**
```rust
struct ADPlayerInfo {
    x: f64, y: f64, z: f64,
    name: String,
    uuid: String,
    health: f64,
    max_health: f64,
    is_flying: bool,
    is_sprinting: bool,
    is_sneaking: bool,
    game_mode: String,
    experience: f64,
    level: u32,
    pitch: f64,
}
```

**メソッド:**
```rust
async fn get_online_players(&self) -> Result<Vec<String>, PeripheralError>
async fn get_players_in_range(&self, radius: f64) -> Result<Vec<String>, PeripheralError>
async fn get_players_in_coords(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<Vec<String>, PeripheralError>
async fn get_players_in_cubic(&self, dx: f64, dy: f64, dz: f64) -> Result<Vec<String>, PeripheralError>
async fn is_players_in_range(&self, radius: f64) -> Result<bool, PeripheralError>
async fn is_players_in_coords(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<bool, PeripheralError>
async fn is_players_in_cubic(&self, dx: f64, dy: f64, dz: f64) -> Result<bool, PeripheralError>
async fn is_player_in_range(&self, player: &str, radius: f64) -> Result<bool, PeripheralError>
async fn is_player_in_coords(&self, player: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<bool, PeripheralError>
async fn is_player_in_cubic(&self, player: &str, dx: f64, dy: f64, dz: f64) -> Result<bool, PeripheralError>
async fn get_player_pos(&self, player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, PeripheralError>
async fn get_player(&self, player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, PeripheralError>
```

---

### CC:VS (Valkyrien Skies CC Compat)

#### Aerodynamics

```rust
// NAME = "vs_aerodynamics"
// use rc::cc_vs::Aerodynamics;
```

**型:**
```rust
struct VSAtmosphericParameters { max_y: f64, sea_level: f64, gravity: f64 }
```

**メソッド:**
```rust
fn default_max_imm(&self) -> Result<f64, PeripheralError>
fn default_sea_level_imm(&self) -> Result<f64, PeripheralError>
fn drag_coefficient_imm(&self) -> Result<f64, PeripheralError>
fn gravitational_acceleration_imm(&self) -> Result<f64, PeripheralError>
fn universal_gas_constant_imm(&self) -> Result<f64, PeripheralError>
fn air_molar_mass_imm(&self) -> Result<f64, PeripheralError>
async fn get_atmospheric_parameters(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError>
fn     get_atmospheric_parameters_imm(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError>
async fn get_air_density(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
fn     get_air_density_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
async fn get_air_pressure(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
fn     get_air_pressure_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
async fn get_air_temperature(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
fn     get_air_temperature_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
```

---

#### Drag

```rust
// NAME = "vs_drag"
// use rc::cc_vs::Drag;
```

**型:**
```rust
struct VSVector3 { x: f64, y: f64, z: f64 }
```

**メソッド:**
```rust
async fn get_drag_force(&self) -> Result<Option<VSVector3>, PeripheralError>
fn     get_drag_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError>
async fn get_lift_force(&self) -> Result<Option<VSVector3>, PeripheralError>
fn     get_lift_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError>
async fn enable_drag(&self) -> Result<(), PeripheralError>
async fn disable_drag(&self) -> Result<(), PeripheralError>
async fn enable_lift(&self) -> Result<(), PeripheralError>
async fn disable_lift(&self) -> Result<(), PeripheralError>
async fn enable_rot_drag(&self) -> Result<(), PeripheralError>
async fn disable_rot_drag(&self) -> Result<(), PeripheralError>
async fn set_wind_direction(&self, x: f64, y: f64, z: f64) -> Result<(), PeripheralError>
async fn set_wind_speed(&self, speed: f64) -> Result<(), PeripheralError>
async fn apply_wind_impulse(&self, x: f64, y: f64, z: f64, speed: f64) -> Result<(), PeripheralError>
```

---

#### Ship

```rust
// NAME = "ship"
// use rc::cc_vs::Ship;
```

**型:**
```rust
struct VSVector3    { x: f64, y: f64, z: f64 }
struct VSQuaternion { x: f64, y: f64, z: f64, w: f64 }
struct VSTransformMatrix { matrix: [[f64; 4]; 4] }
struct VSJoint { id: u64, name: String }
struct VSTeleportData { /* フィールド省略 */ }
struct VSPhysicsTickData { /* フィールド省略 */ }
```

**メソッド:**
```rust
async fn get_id(&self) -> Result<i64, PeripheralError>
fn     get_id_imm(&self) -> Result<i64, PeripheralError>
async fn get_mass(&self) -> Result<f64, PeripheralError>
fn     get_mass_imm(&self) -> Result<f64, PeripheralError>
async fn get_slug(&self) -> Result<String, PeripheralError>
fn     get_slug_imm(&self) -> Result<String, PeripheralError>
async fn get_velocity(&self) -> Result<VSVector3, PeripheralError>
fn     get_velocity_imm(&self) -> Result<VSVector3, PeripheralError>
async fn get_angular_velocity(&self) -> Result<VSVector3, PeripheralError>
fn     get_angular_velocity_imm(&self) -> Result<VSVector3, PeripheralError>
async fn get_quaternion(&self) -> Result<VSQuaternion, PeripheralError>
fn     get_quaternion_imm(&self) -> Result<VSQuaternion, PeripheralError>
async fn get_worldspace_position(&self) -> Result<VSVector3, PeripheralError>
fn     get_worldspace_position_imm(&self) -> Result<VSVector3, PeripheralError>
async fn get_shipyard_position(&self) -> Result<VSVector3, PeripheralError>
fn     get_shipyard_position_imm(&self) -> Result<VSVector3, PeripheralError>
async fn get_scale(&self) -> Result<VSVector3, PeripheralError>
fn     get_scale_imm(&self) -> Result<VSVector3, PeripheralError>
async fn get_size(&self) -> Result<VSVector3, PeripheralError>
fn     get_size_imm(&self) -> Result<VSVector3, PeripheralError>
async fn is_static(&self) -> Result<bool, PeripheralError>
fn     is_static_imm(&self) -> Result<bool, PeripheralError>
async fn get_transformation_matrix(&self) -> Result<VSTransformMatrix, PeripheralError>
fn     get_transformation_matrix_imm(&self) -> Result<VSTransformMatrix, PeripheralError>
async fn get_moment_of_inertia_tensor(&self) -> Result<[[f64; 3]; 3], PeripheralError>
fn     get_moment_of_inertia_tensor_imm(&self) -> Result<[[f64; 3]; 3], PeripheralError>
async fn get_joints(&self) -> Result<Vec<VSJoint>, PeripheralError>
fn     get_joints_imm(&self) -> Result<Vec<VSJoint>, PeripheralError>
async fn transform_position_to_world(&self, pos: VSVector3) -> Result<VSVector3, PeripheralError>
fn     transform_position_to_world_imm(&self, pos: VSVector3) -> Result<VSVector3, PeripheralError>
async fn set_slug(&self, name: &str) -> Result<(), PeripheralError>
async fn set_static(&self, is_static: bool) -> Result<(), PeripheralError>
async fn set_scale_value(&self, scale: f64) -> Result<(), PeripheralError>
async fn teleport(&self, data: &VSTeleportData) -> Result<(), PeripheralError>
async fn apply_world_force(&self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) -> Result<(), PeripheralError>
async fn apply_world_torque(&self, tx: f64, ty: f64, tz: f64) -> Result<(), PeripheralError>
async fn apply_model_force(&self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) -> Result<(), PeripheralError>
async fn apply_model_torque(&self, tx: f64, ty: f64, tz: f64) -> Result<(), PeripheralError>
async fn apply_body_force(&self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) -> Result<(), PeripheralError>
async fn apply_body_torque(&self, tx: f64, ty: f64, tz: f64) -> Result<(), PeripheralError>
async fn apply_world_force_to_model_pos(&self, fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64) -> Result<(), PeripheralError>
async fn apply_world_force_to_body_pos(&self, fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64) -> Result<(), PeripheralError>
async fn try_pull_physics_ticks(&self) -> Result<Option<VSPhysicsTickData>, PeripheralError>
async fn pull_physics_ticks(&self) -> Result<VSPhysicsTickData, PeripheralError>
```

---

### Clockwork CC Compat

#### Boiler

```rust
// NAME = "clockwork:boiler"
// use rc::clockwork_cc_compat::Boiler;
```

**型:**
```rust
struct CLFluidInfo { fluid: String, amount: u32, capacity: u32 }
struct CLPosition  { x: i32, y: i32, z: i32 }
```

**メソッド（全て async + imm ペア）:**
```rust
async fn is_active(&self) -> Result<bool, PeripheralError>
async fn get_heat_level(&self) -> Result<f64, PeripheralError>
async fn get_active_heat(&self) -> Result<f64, PeripheralError>
async fn is_passive_heat(&self) -> Result<bool, PeripheralError>
async fn get_water_supply(&self) -> Result<f64, PeripheralError>
async fn get_attached_engines(&self) -> Result<u32, PeripheralError>
async fn get_attached_whistles(&self) -> Result<u32, PeripheralError>
async fn get_engine_efficiency(&self) -> Result<f64, PeripheralError>
async fn get_boiler_size(&self) -> Result<f64, PeripheralError>
async fn get_width(&self) -> Result<u32, PeripheralError>
async fn get_height(&self) -> Result<u32, PeripheralError>
async fn get_max_heat_for_size(&self) -> Result<f64, PeripheralError>
async fn get_max_heat_for_water(&self) -> Result<f64, PeripheralError>
async fn get_fill_state(&self) -> Result<f64, PeripheralError>
async fn get_fluid_contents(&self) -> Result<CLFluidInfo, PeripheralError>
async fn get_controller_pos(&self) -> Result<CLPosition, PeripheralError>
// 各々に対応する *_imm バリアント有り
```

---

#### GasEngine

```rust
// NAME = "clockwork:gas_engine"
// use rc::clockwork_cc_compat::GasEngine;
```

**メソッド:**
```rust
async fn get_attached_engines(&self) -> Result<u32, PeripheralError>
fn     get_attached_engines_imm(&self) -> Result<u32, PeripheralError>
async fn get_total_efficiency(&self) -> Result<f64, PeripheralError>
fn     get_total_efficiency_imm(&self) -> Result<f64, PeripheralError>
```

---

### CC:Tweaked

#### Inventory

```rust
// NAME = "inventory"
// use rc::computer_craft::Inventory;
```

**型:**
```rust
struct ItemDetail {
    name: String,
    count: u32,
    max_count: u32,
    display_name: String,
    damage: Option<u32>,
    max_damage: Option<u32>,
    tags: BTreeMap<String, bool>,
}
struct SlotInfo { name: String, count: u32 }
```

**メソッド:**
```rust
async fn size(&self) -> Result<u32, PeripheralError>
async fn list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
async fn get_item_detail(&self, slot: u32) -> Result<Option<ItemDetail>, PeripheralError>
async fn push_items(&self, to: &Inventory, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
async fn pull_items(&self, from: &Inventory, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
```

---

#### WirelessModem / WiredModem

```rust
// NAME = "modem"
// use rc::computer_craft::{WirelessModem, WiredModem};
// WirelessModem: IS_WIRELESS = true
// WiredModem:    IS_WIRELESS = false
```

**型:**
```rust
struct ReceiveData<T> {
    channel: u32,
    reply_channel: u32,
    payload: T,
    distance: f64,
}
```

**メソッド (Modem トレイト):**
```rust
async fn open(&self, channel: u32) -> Result<(), PeripheralError>
async fn is_open(&self, channel: u32) -> Result<bool, PeripheralError>
async fn close(&self, channel: u32) -> Result<(), PeripheralError>
async fn close_all(&self) -> Result<(), PeripheralError>
async fn transmit<T: Serialize>(&self, channel: u32, reply_channel: u32, payload: &T) -> Result<(), PeripheralError>
async fn transmit_raw(&self, channel: u32, reply_channel: u32, payload: &str) -> Result<(), PeripheralError>
async fn try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError>
async fn receive_wait_raw(&self) -> Result<ReceiveData<String>, PeripheralError>
```

---

#### NormalMonitor / AdvancedMonitor

```rust
// NAME = "monitor"
// use rc::computer_craft::{NormalMonitor, AdvancedMonitor};
// NormalMonitor:  IS_COLOR = false
// AdvancedMonitor: IS_COLOR = true
```

**型:**
```rust
struct MonitorColor(pub u32);
// 定数: MonitorColor::WHITE, BLACK, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA,
//       ORANGE, PURPLE, BROWN, LIGHT_GRAY, GRAY, PINK, LIME, LIGHT_BLUE

struct MonitorTextScale(pub f32);
// 定数: MonitorTextScale::HALF, NORMAL, DOUBLE, TRIPLE, QUADRUPLE, QUINTUPLE

struct MonitorPosition { pub x: u32, pub y: u32 }
struct MonitorSize     { pub x: u32, pub y: u32 }
```

**メソッド (Monitor トレイト):**
```rust
async fn set_text_scale(&self, scale: MonitorTextScale) -> Result<(), PeripheralError>
async fn get_text_scale(&self) -> Result<MonitorTextScale, PeripheralError>
fn     get_text_scale_imm(&self) -> Result<MonitorTextScale, PeripheralError>
async fn write(&self, text: &str) -> Result<(), PeripheralError>
async fn scroll(&self, y: u32) -> Result<(), PeripheralError>
async fn get_cursor_pos(&self) -> Result<MonitorPosition, PeripheralError>
fn     get_cursor_pos_imm(&self) -> Result<MonitorPosition, PeripheralError>
async fn set_cursor_pos(&self, pos: MonitorPosition) -> Result<(), PeripheralError>
async fn get_cursor_blink(&self) -> Result<bool, PeripheralError>
fn     get_cursor_blink_imm(&self) -> Result<bool, PeripheralError>
async fn set_cursor_blink(&self, blink: bool) -> Result<(), PeripheralError>
async fn get_size(&self) -> Result<MonitorSize, PeripheralError>
fn     get_size_imm(&self) -> Result<MonitorSize, PeripheralError>
async fn clear(&self) -> Result<(), PeripheralError>
async fn clear_line(&self) -> Result<(), PeripheralError>
async fn get_text_color(&self) -> Result<MonitorColor, PeripheralError>
fn     get_text_color_imm(&self) -> Result<MonitorColor, PeripheralError>
async fn set_text_color(&self, color: MonitorColor) -> Result<(), PeripheralError>
async fn get_background_color(&self) -> Result<MonitorColor, PeripheralError>
fn     get_background_color_imm(&self) -> Result<MonitorColor, PeripheralError>
async fn set_background_color(&self, color: MonitorColor) -> Result<(), PeripheralError>
async fn blit(&self, text: &str, text_color: MonitorColor, background_color: MonitorColor) -> Result<(), PeripheralError>
```

---

#### Speaker

```rust
// NAME = "speaker"
// use rc::computer_craft::Speaker;
```

**型:**
```rust
enum SpeakerInstrument {
    Harp, Basedrum, Snare, Hat, Bass, Flute, Bell, Guitar,
    Chime, Xylophone, IronXylophone, CowBell, Didgeridoo,
    Bit, Banjo, Pling,
}
```

**メソッド:**
```rust
async fn play_note(&self, instrument: SpeakerInstrument, volume: Option<f32>, pitch: Option<f32>) -> Result<(), PeripheralError>
async fn play_sound(&self, name: &str, volume: Option<f32>, pitch: Option<f32>) -> Result<(), PeripheralError>
async fn stop(&self) -> Result<(), PeripheralError>
```

---

### Control Craft

#### Camera

```rust
// NAME = "controlcraft:camera"
// use rc::control_craft::Camera;
```

**型:**
```rust
struct CTLTransform { matrix: [[f64; 4]; 4] }
struct CTLRaycastResult {
    hit_type: String,
    pos: (f64, f64, f64),
    block_pos: Option<(i32, i32, i32)>,
    entity_id: Option<i64>,
    entity_type: Option<String>,
    ship_id: Option<i64>,
    player_name: Option<String>,
    distance: f64,
}
```

**メソッド:**
```rust
// ゲッター (async + imm ペア)
async fn get_abs_view_transform(&self) -> Result<CTLTransform, PeripheralError>
async fn get_pitch(&self) -> Result<f64, PeripheralError>
async fn get_yaw(&self) -> Result<f64, PeripheralError>
async fn get_transformed_pitch(&self) -> Result<f64, PeripheralError>
async fn get_transformed_yaw(&self) -> Result<f64, PeripheralError>
async fn get_clip_distance(&self) -> Result<f64, PeripheralError>
async fn get_camera_position(&self) -> Result<(f64, f64, f64), PeripheralError>
async fn get_abs_view_forward(&self) -> Result<(f64, f64, f64), PeripheralError>
async fn is_being_used(&self) -> Result<bool, PeripheralError>
async fn get_direction(&self) -> Result<String, PeripheralError>
async fn latest_ship(&self) -> Result<Option<Value>, PeripheralError>
async fn latest_player(&self) -> Result<Option<Value>, PeripheralError>
async fn latest_entity(&self) -> Result<Option<Value>, PeripheralError>
async fn latest_block(&self) -> Result<Option<Value>, PeripheralError>
// (全て *_imm バリアント有り)

// レイキャスト (async のみ)
async fn clip(&self) -> Result<CTLRaycastResult, PeripheralError>
async fn clip_entity(&self) -> Result<CTLRaycastResult, PeripheralError>
async fn clip_block(&self) -> Result<CTLRaycastResult, PeripheralError>
async fn clip_all_entity(&self) -> Result<Vec<CTLRaycastResult>, PeripheralError>
async fn clip_ship(&self) -> Result<CTLRaycastResult, PeripheralError>
async fn clip_player(&self) -> Result<CTLRaycastResult, PeripheralError>
async fn raycast(&self, x: f64, y: f64, z: f64) -> Result<CTLRaycastResult, PeripheralError>
async fn get_entities(&self, radius: f64) -> Result<Vec<Value>, PeripheralError>
async fn get_mobs(&self, radius: f64) -> Result<Vec<Value>, PeripheralError>

// アクション系
async fn set_pitch(&self, degrees: f64) -> Result<(), PeripheralError>
async fn set_yaw(&self, degrees: f64) -> Result<(), PeripheralError>
async fn force_pitch_yaw(&self, pitch: f64, yaw: f64) -> Result<(), PeripheralError>
async fn set_clip_range(&self, range: f64) -> Result<(), PeripheralError>
async fn set_cone_angle(&self, angle: f64) -> Result<(), PeripheralError>
async fn outline_to_user(&self) -> Result<(), PeripheralError>
async fn reset(&self) -> Result<(), PeripheralError>
```

---

### Create

共通型:

```rust
struct CRItemDetail { name: String, count: u32, display_name: String, tags: BTreeMap<String, bool> }
struct CRSlotInfo   { name: String, count: u32 }
struct CROrderItem  { name: String, count: u32 }
struct CRItemFilter { name: Option<String>, request_count: Option<u32> }
struct CRSignalParams { r: Option<u8>, g: Option<u8>, b: Option<u8>, glow_width: Option<u8>, glow_height: Option<u8>, blink_period: Option<u8>, blink_off_time: Option<u8> }
struct CRPackage { address: String }
```

#### CreativeMotor

```rust
// NAME = "create:creative_motor"
// use rc::create::CreativeMotor;
```

```rust
async fn set_generated_speed(&self, speed: i32) -> Result<(), PeripheralError>
async fn get_generated_speed(&self) -> Result<f32, PeripheralError>
fn     get_generated_speed_imm(&self) -> Result<f32, PeripheralError>
```

#### DisplayLink

```rust
// NAME = "create:display_link"
// use rc::create::DisplayLink;
```

```rust
async fn set_cursor_pos(&self, x: u32, y: u32) -> Result<(), PeripheralError>
async fn get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
fn     get_cursor_pos_imm(&self) -> Result<(u32, u32), PeripheralError>
async fn get_size(&self) -> Result<(u32, u32), PeripheralError>
async fn is_color(&self) -> Result<bool, PeripheralError>
fn     is_color_imm(&self) -> Result<bool, PeripheralError>
async fn write(&self, text: &str) -> Result<(), PeripheralError>
async fn write_bytes(&self, data: &[u8]) -> Result<(), PeripheralError>
async fn clear_line(&self) -> Result<(), PeripheralError>
async fn clear(&self) -> Result<(), PeripheralError>
async fn update(&self) -> Result<(), PeripheralError>
```

#### Frogport

```rust
// NAME = "create:frogport"
// use rc::create::Frogport;
```

```rust
async fn set_address(&self, address: &str) -> Result<(), PeripheralError>
async fn get_address(&self) -> Result<String, PeripheralError>
async fn get_configuration(&self) -> Result<CRPackage, PeripheralError>
async fn set_configuration(&self, config: &CRPackage) -> Result<(), PeripheralError>
async fn list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
async fn get_item_detail(&self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError>
async fn try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError>
async fn try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError>
```

#### NixieTube

```rust
// NAME = "create:nixie_tube"
// use rc::create::NixieTube;
```

```rust
async fn set_text(&self, text: &str, colour: Option<&str>) -> Result<(), PeripheralError>
async fn set_text_colour(&self, colour: &str) -> Result<(), PeripheralError>
async fn set_signal(&self, front: &CRSignalParams, back: Option<&CRSignalParams>) -> Result<(), PeripheralError>
```

#### Packager

```rust
// NAME = "create:packager"
// use rc::create::Packager;
```

```rust
async fn make_package(&self) -> Result<(), PeripheralError>
async fn list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
async fn get_item_detail(&self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError>
async fn get_address(&self) -> Result<String, PeripheralError>
async fn set_address(&self, address: &str) -> Result<(), PeripheralError>
async fn get_package(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError>
async fn try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError>
```

#### Postbox

```rust
// NAME = "create:postbox"
// use rc::create::Postbox;
```

```rust
async fn set_address(&self, address: &str) -> Result<(), PeripheralError>
async fn get_address(&self) -> Result<String, PeripheralError>
async fn list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
async fn get_item_detail(&self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError>
async fn get_configuration(&self) -> Result<CRPackage, PeripheralError>
async fn set_configuration(&self, config: &CRPackage) -> Result<(), PeripheralError>
async fn try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError>
async fn try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError>
```

#### RedstoneRequester

```rust
// NAME = "create:redstone_requester"
// use rc::create::RedstoneRequester;
```

```rust
async fn request(&self, items: &[CROrderItem]) -> Result<(), PeripheralError>
async fn set_request(&self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
async fn set_crafting_request(&self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
async fn get_request(&self, slot: u32) -> Result<Option<CRItemFilter>, PeripheralError>
async fn get_configuration(&self) -> Result<CRPackage, PeripheralError>
async fn set_configuration(&self, config: &CRPackage) -> Result<(), PeripheralError>
async fn set_address(&self, address: &str) -> Result<(), PeripheralError>
async fn get_address(&self) -> Result<String, PeripheralError>
```

#### Repackager

```rust
// NAME = "create:repackager"
// use rc::create::Repackager;
```

```rust
async fn make_package(&self) -> Result<(), PeripheralError>
async fn list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
async fn get_item_detail(&self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError>
async fn get_address(&self) -> Result<String, PeripheralError>
async fn set_address(&self, address: &str) -> Result<(), PeripheralError>
async fn get_package(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn try_pull_package_repackaged(&self) -> Result<Option<(CRPackage, u32)>, PeripheralError>
async fn pull_package_repackaged(&self) -> Result<(CRPackage, u32), PeripheralError>
async fn try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError>
async fn try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError>
async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError>
```

#### RotationSpeedController

```rust
// NAME = "create:rotation_speed_controller"
// use rc::create::RotationSpeedController;
```

```rust
async fn set_target_speed(&self, speed: i32) -> Result<(), PeripheralError>
async fn get_target_speed(&self) -> Result<f32, PeripheralError>
fn     get_target_speed_imm(&self) -> Result<f32, PeripheralError>
```

#### SequencedGearshift

```rust
// NAME = "create:sequenced_gearshift"
// use rc::create::SequencedGearshift;
```

```rust
async fn rotate(&self, amount: i32, speed_modifier: Option<i32>) -> Result<(), PeripheralError>
async fn move_by(&self, distance: i32, speed_modifier: Option<i32>) -> Result<(), PeripheralError>
async fn is_running(&self) -> Result<bool, PeripheralError>
fn     is_running_imm(&self) -> Result<bool, PeripheralError>
```

#### Signal

```rust
// NAME = "create:signal"
// use rc::create::Signal;
```

```rust
async fn get_state(&self) -> Result<String, PeripheralError>
fn     get_state_imm(&self) -> Result<String, PeripheralError>
async fn is_forced_red(&self) -> Result<bool, PeripheralError>
fn     is_forced_red_imm(&self) -> Result<bool, PeripheralError>
async fn set_forced_red(&self, powered: bool) -> Result<(), PeripheralError>
async fn list_blocking_train_names(&self) -> Result<Vec<String>, PeripheralError>
fn     list_blocking_train_names_imm(&self) -> Result<Vec<String>, PeripheralError>
async fn get_signal_type(&self) -> Result<String, PeripheralError>
fn     get_signal_type_imm(&self) -> Result<String, PeripheralError>
async fn cycle_signal_type(&self) -> Result<(), PeripheralError>
async fn try_pull_train_signal_state_change(&self) -> Result<Option<()>, PeripheralError>
async fn pull_train_signal_state_change(&self) -> Result<(), PeripheralError>
```

#### Speedometer

```rust
// NAME = "create:speedometer"
// use rc::create::Speedometer;
```

```rust
async fn get_speed(&self) -> Result<f32, PeripheralError>
fn     get_speed_imm(&self) -> Result<f32, PeripheralError>
async fn try_pull_speed_change(&self) -> Result<Option<f32>, PeripheralError>
async fn pull_speed_change(&self) -> Result<f32, PeripheralError>
```

#### Station

```rust
// NAME = "create:station"
// use rc::create::Station;
```

```rust
async fn assemble(&self) -> Result<(), PeripheralError>
async fn disassemble(&self) -> Result<(), PeripheralError>
async fn set_assembly_mode(&self, mode: bool) -> Result<(), PeripheralError>
async fn is_in_assembly_mode(&self) -> Result<bool, PeripheralError>
fn     is_in_assembly_mode_imm(&self) -> Result<bool, PeripheralError>
async fn get_station_name(&self) -> Result<String, PeripheralError>
fn     get_station_name_imm(&self) -> Result<String, PeripheralError>
async fn set_station_name(&self, name: &str) -> Result<(), PeripheralError>
async fn is_train_present(&self) -> Result<bool, PeripheralError>
fn     is_train_present_imm(&self) -> Result<bool, PeripheralError>
async fn is_train_imminent(&self) -> Result<bool, PeripheralError>
fn     is_train_imminent_imm(&self) -> Result<bool, PeripheralError>
async fn is_train_enroute(&self) -> Result<bool, PeripheralError>
fn     is_train_enroute_imm(&self) -> Result<bool, PeripheralError>
async fn get_train_name(&self) -> Result<String, PeripheralError>
fn     get_train_name_imm(&self) -> Result<String, PeripheralError>
async fn set_train_name(&self, name: &str) -> Result<(), PeripheralError>
async fn has_schedule(&self) -> Result<bool, PeripheralError>
fn     has_schedule_imm(&self) -> Result<bool, PeripheralError>
async fn get_schedule(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
fn     get_schedule_imm(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
async fn set_schedule(&self, schedule: &BTreeMap<String, Value>) -> Result<(), PeripheralError>
async fn can_train_reach(&self, dest: &str) -> Result<(bool, Option<String>), PeripheralError>
fn     can_train_reach_imm(&self, dest: &str) -> Result<(bool, Option<String>), PeripheralError>
async fn distance_to(&self, dest: &str) -> Result<(Option<f64>, Option<String>), PeripheralError>
fn     distance_to_imm(&self, dest: &str) -> Result<(Option<f64>, Option<String>), PeripheralError>
async fn try_pull_train_arrive(&self) -> Result<Option<()>, PeripheralError>
async fn pull_train_arrive(&self) -> Result<(), PeripheralError>
async fn try_pull_train_depart(&self) -> Result<Option<()>, PeripheralError>
async fn pull_train_depart(&self) -> Result<(), PeripheralError>
```

#### Sticker

```rust
// NAME = "create:sticker"
// use rc::create::Sticker;
```

```rust
async fn is_extended(&self) -> Result<bool, PeripheralError>
fn     is_extended_imm(&self) -> Result<bool, PeripheralError>
async fn is_attached_to_block(&self) -> Result<bool, PeripheralError>
fn     is_attached_to_block_imm(&self) -> Result<bool, PeripheralError>
async fn extend(&self) -> Result<bool, PeripheralError>
async fn retract(&self) -> Result<bool, PeripheralError>
async fn toggle(&self) -> Result<bool, PeripheralError>
```

#### StockTicker

```rust
// NAME = "create:stock_ticker"
// use rc::create::StockTicker;
```

```rust
async fn stock(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
async fn get_stock_item_detail(&self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError>
async fn request_filtered(&self, filters: &[CRItemFilter]) -> Result<(), PeripheralError>
async fn list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
async fn get_item_detail(&self, slot: u32) -> Result<Option<CRItemDetail>, PeripheralError>
```

#### Stressometer

```rust
// NAME = "create:stressometer"
// use rc::create::Stressometer;
```

```rust
async fn get_stress(&self) -> Result<f32, PeripheralError>
fn     get_stress_imm(&self) -> Result<f32, PeripheralError>
async fn get_stress_capacity(&self) -> Result<f32, PeripheralError>
fn     get_stress_capacity_imm(&self) -> Result<f32, PeripheralError>
async fn try_pull_overstressed(&self) -> Result<Option<()>, PeripheralError>
async fn pull_overstressed(&self) -> Result<(), PeripheralError>
async fn try_pull_stress_change(&self) -> Result<Option<(f32, f32)>, PeripheralError>
async fn pull_stress_change(&self) -> Result<(f32, f32), PeripheralError>
```

#### TableclothShop

```rust
// NAME = "create:tablecloth_shop"
// use rc::create::TableclothShop;
```

```rust
async fn is_shop(&self) -> Result<bool, PeripheralError>
async fn get_address(&self) -> Result<String, PeripheralError>
async fn set_address(&self, address: &str) -> Result<(), PeripheralError>
async fn get_price_tag_item(&self) -> Result<Option<CRItemDetail>, PeripheralError>
async fn set_price_tag_item(&self, item_name: &str) -> Result<(), PeripheralError>
async fn get_price_tag_count(&self) -> Result<u32, PeripheralError>
async fn set_price_tag_count(&self, count: u32) -> Result<(), PeripheralError>
async fn get_wares(&self) -> Result<Option<CRItemDetail>, PeripheralError>
async fn set_wares(&self, item_name: &str) -> Result<(), PeripheralError>
```

#### TrackObserver

```rust
// NAME = "create:track_observer"
// use rc::create::TrackObserver;
```

```rust
async fn is_train_passing(&self) -> Result<bool, PeripheralError>
fn     is_train_passing_imm(&self) -> Result<bool, PeripheralError>
async fn get_passing_train_name(&self) -> Result<Option<String>, PeripheralError>
fn     get_passing_train_name_imm(&self) -> Result<Option<String>, PeripheralError>
async fn try_pull_train_passing(&self) -> Result<Option<()>, PeripheralError>
async fn pull_train_passing(&self) -> Result<(), PeripheralError>
async fn try_pull_train_passed(&self) -> Result<Option<()>, PeripheralError>
async fn pull_train_passed(&self) -> Result<(), PeripheralError>
```

---

### Create Addition

#### ElectricMotor

```rust
// NAME = "createaddition:electric_motor"
// use rc::createaddition::ElectricMotor;
```

```rust
async fn get_type(&self) -> Result<String, PeripheralError>
fn     get_type_imm(&self) -> Result<String, PeripheralError>
async fn set_speed(&self, speed: f64) -> Result<(), PeripheralError>   // RPM、符号で回転方向
async fn stop(&self) -> Result<(), PeripheralError>
async fn get_speed(&self) -> Result<f64, PeripheralError>
async fn get_stress_capacity(&self) -> Result<f64, PeripheralError>
async fn get_energy_consumption(&self) -> Result<f64, PeripheralError>
async fn rotate(&self, degrees: f64, rpm: Option<f64>) -> Result<f64, PeripheralError>   // 戻り値: 所要秒数
async fn translate(&self, distance: f64, rpm: Option<f64>) -> Result<f64, PeripheralError>
async fn get_max_insert(&self) -> Result<f64, PeripheralError>
async fn get_max_extract(&self) -> Result<f64, PeripheralError>
```

---

### Some Peripherals

#### BallisticAccelerator

```rust
// NAME = "sp:ballistic_accelerator"
// use rc::some_peripherals::BallisticAccelerator;
```

**型:**
```rust
struct SPCoordinate { x: f64, y: f64, z: f64 }
struct SPTimeResult  { ticks: f64, aux: f64 }
struct SPPitchResult { pitch: f64, aux: f64 }
```

**メソッド:**
```rust
async fn time_in_air(&self, y_proj: f64, y_tgt: f64, y_vel: f64, gravity: Option<f64>, drag: Option<f64>, max_steps: Option<u32>) -> Result<SPTimeResult, PeripheralError>
fn     time_in_air_imm(&self, ...) -> Result<SPTimeResult, PeripheralError>
async fn try_pitch(&self, pitch: f64, speed: f64, length: f64, dist: f64, cannon: SPCoordinate, target: SPCoordinate, gravity: Option<f64>, drag: Option<f64>, max_steps: Option<u32>) -> Result<(f64, f64, f64), PeripheralError>
fn     try_pitch_imm(&self, ...) -> Result<(f64, f64, f64), PeripheralError>
async fn calculate_pitch(&self, cannon: SPCoordinate, target: SPCoordinate, speed: f64, length: f64, amin: Option<f64>, amax: Option<f64>, gravity: Option<f64>, drag: Option<f64>, max_delta_t_error: Option<f64>, max_steps: Option<u32>, num_iterations: Option<u32>, num_elements: Option<u32>, check_impossible: Option<bool>) -> Result<SPPitchResult, PeripheralError>
fn     calculate_pitch_imm(&self, ...) -> Result<SPPitchResult, PeripheralError>
async fn batch_calculate_pitches(&self, cannon: SPCoordinate, targets: &[SPCoordinate], speed: f64, length: f64, ...) -> Result<Vec<SPPitchResult>, PeripheralError>
async fn get_drag(&self, base_drag: f64, dim_drag_multiplier: f64) -> Result<f64, PeripheralError>
fn     get_drag_imm(&self, ...) -> Result<f64, PeripheralError>
```

---

#### Digitizer

```rust
// NAME = "sp:digitizer"
// use rc::some_peripherals::Digitizer;
```

**型:**
```rust
struct SPItemData { id: String, count: u32, tag: Value }
```

**メソッド:**
```rust
async fn digitize_amount(&self, amount: Option<u32>) -> Result<String, PeripheralError>   // UUID を返す
async fn rematerialize_amount(&self, uuid: &str, amount: Option<u32>) -> Result<bool, PeripheralError>
async fn merge_digital_items(&self, into_uuid: &str, from_uuid: &str, amount: Option<u32>) -> Result<bool, PeripheralError>
async fn separate_digital_item(&self, from_uuid: &str, amount: u32) -> Result<String, PeripheralError>
async fn check_id(&self, uuid: &str) -> Result<SPItemData, PeripheralError>
async fn get_item_in_slot(&self) -> Result<SPItemData, PeripheralError>
async fn get_item_limit_in_slot(&self) -> Result<u32, PeripheralError>
```

---

#### GoggleLinkPort

```rust
// NAME = "sp:goggle_link_port"
// use rc::some_peripherals::GoggleLinkPort;
```

```rust
async fn get_connected(&self) -> Result<BTreeMap<String, Value>, PeripheralError>
```

---

#### Radar

```rust
// NAME = "sp_radar"
// use rc::some_peripherals::Radar;
```

**型:**
```rust
struct SPEntityInfo {
    x: f64, y: f64, z: f64,
    id: String,
    entity_type: String,
    name: String,
}
struct SPShipInfo {
    is_ship: bool,
    ship_id: i64,
    pos: SPCoordinate,
    mass: f64,
    rotation: VSQuaternion,
    velocity: VSVector3,
    size: VSVector3,
    scale: VSVector3,
    // ...
}
```

**メソッド:**
```rust
async fn scan_for_entities(&self, radius: f64) -> Result<Vec<SPEntityInfo>, PeripheralError>
async fn scan_for_ships(&self, radius: f64) -> Result<Vec<SPShipInfo>, PeripheralError>
async fn scan_for_players(&self, radius: f64) -> Result<Vec<SPEntityInfo>, PeripheralError>
async fn get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError>
fn     get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError>
```

---

#### Raycaster

```rust
// NAME = "sp:raycaster"
// use rc::some_peripherals::Raycaster;
```

**型:**
```rust
struct SPRaycastResult {
    is_block: bool,
    is_entity: bool,
    abs_pos: SPCoordinate,
    hit_pos: SPCoordinate,
    distance: f64,
    block_type: Option<String>,
    rel_hit_pos: Option<SPCoordinate>,
    id: Option<i64>,
    description_id: Option<String>,
    ship_id: Option<i64>,
    hit_pos_ship: Option<SPCoordinate>,
    error: Option<String>,
}
```

**メソッド:**
```rust
async fn raycast(&self, distance: f64, variables: Option<(f64, f64, Option<f64>)>, euler_mode: Option<bool>, im_execute: Option<bool>, check_for_blocks: Option<bool>, only_distance: Option<bool>) -> Result<SPRaycastResult, PeripheralError>
async fn add_stickers(&self, state: bool) -> Result<(), PeripheralError>
async fn get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError>
fn     get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError>
async fn get_facing_direction(&self) -> Result<String, PeripheralError>
fn     get_facing_direction_imm(&self) -> Result<String, PeripheralError>
```

---

#### WorldScanner

```rust
// NAME = "sp:world_scanner"
// use rc::some_peripherals::WorldScanner;
```

> **注:** `WorldScanner` は現在 `Peripheral` トレイトの実装のみ（接続確認用）。メソッドは未実装。

---

### Tom's Peripherals

#### GPU

```rust
// NAME = "tm:gpu"
// use rc::toms_peripherals::Gpu;
```

**型:**
```rust
struct TMImage  { width: u32, height: u32, data: Vec<u32> }
struct TMWindow { x: f64, y: f64, width: u32, height: u32 }
```

**メソッド:**
```rust
async fn set_size(&self, pixels: u32) -> Result<(), PeripheralError>
async fn refresh_size(&self) -> Result<(), PeripheralError>
// 戻り値: (pixel_width, pixel_height, columns, rows, pixel_size)
async fn get_size(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError>
fn     get_size_imm(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError>
async fn fill(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
async fn sync(&self) -> Result<(), PeripheralError>
async fn filled_rectangle(&self, x: u32, y: u32, w: u32, h: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
async fn draw_image(&self, image: &TMImage, x: u32, y: u32) -> Result<(), PeripheralError>
async fn draw_text(&self, text: &str, x: u32, y: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
async fn draw_char(&self, ch: char, x: u32, y: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
async fn get_text_length(&self, text: &str) -> Result<u32, PeripheralError>
fn     get_text_length_imm(&self, text: &str) -> Result<u32, PeripheralError>
async fn set_font(&self, font_name: &str) -> Result<(), PeripheralError>
async fn clear_chars(&self) -> Result<(), PeripheralError>
async fn add_new_char(&self, codepoint: u32, data: &[u8]) -> Result<(), PeripheralError>
async fn create_window(&self, x: u32, y: u32, w: u32, h: u32) -> Result<TMWindow, PeripheralError>
fn     create_window_imm(&self, x: u32, y: u32, w: u32, h: u32) -> Result<TMWindow, PeripheralError>
async fn decode_image(&self, data: &str) -> Result<TMImage, PeripheralError>
async fn new_image(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError>
fn     new_image_imm(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError>
```

---

#### Keyboard

```rust
// NAME = "tm:keyboard"
// use rc::toms_peripherals::Keyboard;
```

```rust
async fn set_fire_native_events(&self, enabled: bool) -> Result<(), PeripheralError>
```

---

#### RedstonePort

```rust
// NAME = "tm:redstone_port"
// use rc::toms_peripherals::RedstonePort;
```

```rust
fn     get_sides_imm(&self) -> Result<Vec<String>, PeripheralError>   // imm のみ
async fn get_input(&self, side: &str) -> Result<bool, PeripheralError>
async fn get_analog_input(&self, side: &str) -> Result<u8, PeripheralError>
async fn get_bundled_input(&self, side: &str) -> Result<u16, PeripheralError>
async fn get_output(&self, side: &str) -> Result<bool, PeripheralError>
fn     get_output_imm(&self, side: &str) -> Result<bool, PeripheralError>
async fn get_analog_output(&self, side: &str) -> Result<u8, PeripheralError>
fn     get_analog_output_imm(&self, side: &str) -> Result<u8, PeripheralError>
async fn get_bundled_output(&self, side: &str) -> Result<u16, PeripheralError>
fn     get_bundled_output_imm(&self, side: &str) -> Result<u16, PeripheralError>
async fn set_output(&self, side: &str, value: bool) -> Result<(), PeripheralError>
async fn set_analog_output(&self, side: &str, value: u8) -> Result<(), PeripheralError>
async fn set_bundled_output(&self, side: &str, mask: u16) -> Result<(), PeripheralError>
async fn test_bundled_input(&self, side: &str, mask: u16) -> Result<bool, PeripheralError>
```

---

#### WatchdogTimer

```rust
// NAME = "tm:watchdog_timer"
// use rc::toms_peripherals::WatchdogTimer;
```

```rust
async fn is_enabled(&self) -> Result<bool, PeripheralError>
fn     is_enabled_imm(&self) -> Result<bool, PeripheralError>
async fn get_timeout(&self) -> Result<u32, PeripheralError>   // 単位: ticks
fn     get_timeout_imm(&self) -> Result<u32, PeripheralError>
async fn set_enabled(&self, enabled: bool) -> Result<(), PeripheralError>
async fn set_timeout(&self, ticks: u32) -> Result<(), PeripheralError>
async fn reset(&self) -> Result<(), PeripheralError>
```

---

## 移行ガイド (v0.1.27 → v0.1.28)

### `find_imm` の変更

```diff
- // v0.1.27: find_imm は存在しなかった
+ // v0.1.28
+ let motors: Vec<ElectricMotor> = rc::peripheral::find_imm::<ElectricMotor>();
```

### `Peripheral` トレイトの `direction(&self)` の変更

```diff
- // v0.1.27
- fn direction(&self) -> Direction { ... }

+ // v0.1.28
+ fn periph_addr(&self) -> PeriphAddr { ... }
+ // direction() はデフォルト実装で Option<Direction> を返す
+ fn direction(&self) -> Option<Direction> { self.periph_addr().as_direction() }
```

### `new(dir: Direction)` → `new(addr: PeriphAddr)` の変更

```diff
- // v0.1.27
- let monitor = NormalMonitor::new(Direction::North);

+ // v0.1.28 (Direction は impl Into<PeriphAddr> なので変更不要)
+ let monitor = NormalMonitor::new(Direction::North.into());
+ // または
+ let monitor = peripheral::wrap_imm::<NormalMonitor>(Direction::North).unwrap();
```
