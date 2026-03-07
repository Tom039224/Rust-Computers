次のタスク用のメモ書き。未完成。

ここに、関数の引数と返り値のイメージを用意するので、これに沿って実装を進めてください。  
とても膨大な作業量になってしまい申し訳ないですが、タスク登録して順番に進めてください。  

# 戦略
- imm対応の場合、通常のasync関数に加え、imm関数を実装する。  
  今回の実装計画では冗長になる為省くが、実際の実装では以下のように実装してください。
  ```rust
    // imm対応の関数は通常版と即時版の2つを実装
    async fn some_function() -> bool    // これはimpl Future<Output = bool>を返す
    fn some_function_imm() -> bool      // これはboolを返す
    // imm非対応の関数は通常版のみ
    async fn other_function() -> bool   // これはimpl Future<Output = bool>を返す
  ```
- Luaからの不確定な返り値をRustの強力な型システムに落とし込む
- 各Peripheral StructはJavaと通信するうえで必要なIDを持つ(または代替手段)
- オペレーター権限を必要とする関数もあるので、configにallow_opという項目を追加してください。

# 1: CC-Tweaked
ディレクトリ名：`computer_craft`
## Peripheral関連 (`peripheral.rs`)
### 実装するもの
- `fn get_names_imm() -> Vec<String>`  
  immのみ

- `fn is_present_imm<T: Peripheral>() -> bool`- `fn get_types() -> Vec<Result<dyn Peripheral, String>>`  
  immのみ

- `fn find_imm<T: Peripheral>() -> Result<Vec<T>, PeripheralError>`    
  immのみ
  直接、または有線モデムで接続されているPeripheralをベクタで取得。

- `fn wrap_imm<T: Peripheral>(Direction: Direction) -> Result<T, PeripheralError>`  
  immのみ

- `trait Peripheral: Sized`
  - `const Name: String`

  - `fn get_imm() -> Self`  
  immのみ

  - `fn is_available_imm() -> bool`    
  immのみ

  peripheral実装は必ず`impl Peripheral for T`すること

- `enum PeripheralDirection`
  - `Top`
  - `Bottom`
  - `Left`
  - `Right`
  - `Front`
  - `Back`

- `enum PeripheralError`
  - `PeripheralNotAvailable`  
  接続されていたperipheralが切断などにより利用不可になった場合

  - `Unexpected`  
  上記に当てはまらない場合

## Inventory (`inventory.rs`)
アイテムと液体で実装が分かれていた場合、こちらもそうする。
### 実装するもの
- `struct Inventory`

- `struct ItemDetail {
    name: String,
    count: u32,
    max_count: u32,
    display_name: String,
    damage: Option<u32>,
    max_damage: Option<u32>,
    tags: HashMap<String, bool>,
}`

- `struct SlotInfo {
    name: String,
    count: u32,
}`

- `async fn size() -> u32`

- `async fn list() -> HashMap<u32, SlotInfo>`

- `async fn getItemDetail(slot: u32) -> Option<ItemDetail>`

- `getItemLimit`は上で代替可能なため実装なし

- `async fn pushItems(to: Self, to_slot: Option<u32>, from_slot: u32, limit: Option<u32>) -> u32`  

  `to`は転送先の`Inventory`  
  `to_slot`はあれば指定したスロットのみに流し込む。ないなら可能な場所すべてに  
  `limit`があれば送るアイテムの最大数を設定

- `async fn pullItems(from: Self, to_slot: Option<u32>, from_slot: u32, limit: Option<u32>) -> u32`
## Modem (`modem.rs`)
### 実装するもの
- `struct WirelessModem`
  - `is_wireless = true`

- `struct WiredModem`
  - `is_wireless = false`

- `struct ReceiveData<T> {
    channel: u32,
    reply_channel: u32,
    payload: T,
    distance: u32
}`

- `trait Modem`
  - `const is_wireless: bool`

  - `async fn open(channel: u32)`

  - `async fn is_open(channel: u32)`

  - `async fn close(channel: u32)`

  - `async fn close_all()`

  - `async fn transmit<T: serde::Serialize>(channel: u32, reply_channel: u32, payload: &T)`  
  `payload`は`&T`をJSON文字列に変換して送信すること  
  serdeによる変換処理と、`transmit_raw`での送信を行うこと

  - `async fn transmit_raw(channel: u32, reply_channel: u32, payload: &str)`
  生データを送信する

  - `async fn try_receive<T>() -> Option<Result<ReceiveData<T>>, ReceiveData<String>>`  
  1tickの間(Pollされるまで)にメッセージが来たら返す、来なかったらFutureがReadyになり、`None`を返す  
  変換に失敗したらSome(Err(ReceiveData<String>))の形で生データを返す

  - `async fn try_receive_raw() -> Option<String>`
  1tickの間(Pollされるまで)にメッセージが来たら返す、来なかったらFutureがReadyになり、`None`を返す  
  生データを受信する

  - `async fn receive_wait<T>() -> Result<ReceiveData<T>, ReceiveData<String>>`  
  受信されるまで`Ready`にならない  
  変換に失敗したらErr(ReceiveData<String>)の形で生データを返す

  - `async fn receive_wait_raw() -> String`
  受信されるまで`Ready`にならない  
  生データを受信する

## Monitor (`monitor.rs`)
### 実装するもの
- `struct NormalMonitor`
  - `is_color = false`

- `struct AdvancedMonitor`
  - `is_color = true`

- `enum MonitorTextScale(f32)`  
  0.5から5.0の0.5刻み
  - `size0_5 = 0.5`
  - `size1_0 = 1.0`
  - `size1_5 = 1.5`
  - <省略>
  - `size4_5 = 4.5`
  - `size5_0 = 5.0`

- `struct MonitorPosition {
    x: u32,
    y: u32,
}`

- `struct MonitorSize {
    x: u32,
    y: u32,
}`

- `struct MonitorColor(u32)`  
**RRGGBB**の形式です。
  - `impl MonitorColor`
    - `const White = 0xF0F0F0`
    - `const Orange = 0xF2B233`
    - `const Magenta = 0xE57FD8`
    - `const LightBlue = 0x99B2F2`
    - `const Yellow = 0xDEDE6C`
    - `const Lime = 0x7FCC19`
    - `const Pink = 0xF2B2CC`
    - `const Gray = 0x4C4C4C`
    - `const LightGray = 0x999999`
    - `const Cyan = 0x4C99B2`
    - `const Purple = 0xB266E5`
    - `const Blue = 0x3366CC`
    - `const Brown = 0x7F664C`
    - `const Green = 0x57A64E`
    - `const Red = 0xCC4C4C`
    - `const Black = 0x111111`
    - `fn rgb(r: u8, g: u8, b: u8) -> Self`

- `trait Monitor`
  - `const is_color: bool`

  - `async fn set_text_scale(scale: MonitorTextScale)`

  - `async fn get_text_scale() -> MonitorTextScale`  
  imm対応

  - `async fn write(text: &str)`

  - `async fn scroll(y: u32)`

  - `async fn get_cursor_pos() -> MonitorPosition`  
  imm対応

  - `async fn set_cursor_pos(pos: MonitorPosition)`

  - `async fn get_cursor_blink() -> bool`  
  imm対応

  - `async fn set_cursor_blink(blink: bool)`

  - `async fn get_size() -> MonitorSize`  
  imm対応

  - `async fn clear()`

  - `async fn clear_line()`

  - `async fn get_text_color() -> MonitorColor`  
  imm対応

  - `async fn set_text_color(color: MonitorColor)`

  - `async fn get_background_color() -> MonitorColor`  
  imm対応

  - `async fn set_background_color(color: MonitorColor)`

  - `async fn blit(text: &str, text_color: MonitorColor, backgroud_color: MonitorColor)`

## Speaker (`speaker.rs`)
### 実装するもの
- `struct Speaker`

- `enum SpeakerInstrument`
  - harp
  - basedrun
  - snare
  - hat
  - bass
  - flute
  - bell
  - guitar
  - chime
  - xylophone
  - iron_xylophone
  - cow_bell
  - didgeridoo
  - bit
  - banjo
  - pling

- `async fn play_note(instrument: SpeakerInstrument, volume: Option<f32>, pitch: Option<f32>)`

- `async fn play_sound(name: &str, volume: Option<f32>, pitch: Option<f32>) -> Result<(), ()>`

- `async fn stop()`

## DiskDrive
今回はなし
## Printer
今回はなし

# 2: CC-VS
ディレクトリ名：`cc_vs`

## Ship API (`ship.rs`)

### 実装するもの

関数はベタ書きで

- `struct VSVector3 {
    x: f64,
    y: f64,
    z: f64,
}`

- `struct VSQuaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}`

- `struct VSTransformMatrix {
    matrix: [[f64; 4]; 4],
}`

- `struct VSJoint {
    id: u64,
    name: String,
    // その他のジョイント情報
}`

- `struct VSPhysicsTickData {
    buoyant_factor: f64,
    is_static: bool,
    do_fluid_drag: bool,
    inertia: InertiaInfo,
    pose_vel: PoseVelInfo,
    forces_inducers: Vec<String>,
}`

- `struct VSInertiaInfo {
    moment_of_inertia: Vector3,
    mass: f64,
}`

- `struct VSPoseVelInfo {
    vel: Vector3,
    omega: Vector3,
    pos: Vector3,
    rot: Quaternion,
}`

- `struct VSTeleportData {
    pos: Option<Vector3>,
    rot: Option<Quaternion>,
    vel: Option<Vector3>,
    omega: Option<Vector3>,
    dimension: Option<String>,
    scale: Option<f64>,
}`

#### 読み取り系（imm対応）
- `async fn get_id_imm() -> i64`

- `async fn get_mass_imm() -> f64`

- `async fn get_moment_of_inertia_tensor_imm() -> [[f64; 3]; 3]`

- `async fn get_slug_imm() -> String`

- `async fn get_angular_velocity_imm() -> VSVector3`

- `async fn get_quaternion_imm() -> VSQuaternion`

- `async fn get_scale_imm() -> VSVector3`

- `async fn get_shipyard_position_imm() -> VSVector3`

- `async fn get_size_imm() -> VSVector3`

- `async fn get_velocity_imm() -> VSVector3`

- `async fn get_worldspace_position_imm() -> VSVector3`

- `async fn transform_position_to_world_imm(pos: VSVector3) -> VSVector3`

- `async fn is_static_imm() -> bool`

- `async fn get_transformation_matrix_imm() -> VSTransformMatrix`

- `async fn get_joints_imm() -> Vec<VSJoint>`

#### 状態変更系 (Configのallow opのとき)
- `async fn set_slug(name: &str)`

- `async fn set_static(is_static: bool)`

- `async fn set_scale(scale: f64)`

- `async fn teleport(data: VSTeleportData)`

#### 力の印加系 (Configのallow opのとき)
- `async fn apply_world_force(fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>)`

- `async fn apply_world_torque(tx: f64, ty: f64, tz: f64)`

- `async fn apply_model_force(fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>)`

- `async fn apply_model_torque(tx: f64, ty: f64, tz: f64)`

- `async fn apply_world_force_to_model_pos(fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64)`

- `async fn apply_body_force(fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>)`

- `async fn apply_body_torque(tx: f64, ty: f64, tz: f64)`

- `async fn apply_world_force_to_body_pos(fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64)`

#### イベント系
- `async fn try_pull_physics_ticks() -> Option<VSPhysicsTickData>`  
1tickの間(Pollされるまで)に物理ティックイベントが来たら返す、来なかったらFutureがReadyになり、`None`を返す

- `async fn pull_physics_ticks() -> VSPhysicsTickData`  
物理ティックイベントが発生されるまで`Ready`にならない。設定で無効時はパニック

## Aerodynamics API
グローバル API として実装。

### 実装するもの

関数はベタ書きで

- `struct VSAtmosphericParameters {
    max_y: f64,
    sea_level: f64,
    gravity: f64,
}`

#### プロパティ系（imm対応）
- `fn default_max_imm() -> f64`  
`AerodynamicUtils.DEFAULT_MAX`

- `fn default_sea_level_imm() -> f64`  
`AerodynamicUtils.DEFAULT_SEA_LEVEL`

- `fn drag_coefficient_imm() -> f64`  
サーバー設定値

- `fn gravitational_acceleration_imm() -> f64`  
重力加速度定数

- `fn universal_gas_constant_imm() -> f64`  
気体定数 R

- `fn air_molar_mass_imm() -> f64`  
空気のモル質量

#### 関数メソッド（imm対応）
- `async fn get_atmospheric_parameters_imm() -> Option<VSAtmosphericParameters>`  
現在ディメンションの大気パラメータを返す。未初期化時は None

- `async fn get_air_density_imm(y: Option<f64>) -> Option<f64>`  
指定 Y 座標の空気密度。省略時はコンピューターの Y 座標

- `async fn get_air_pressure_imm(y: Option<f64>) -> Option<f64>`  
指定 Y 座標の大気圧

- `async fn get_air_temperature_imm(y: Option<f64>) -> Option<f64>`  
指定 Y 座標の気温

## Drag API
シップ上に配置されたコンピューターから呼び出す。

### 実装するもの

関数はベタ書き

- `struct Drag` - グローバル Drag API を表す

#### 読み取り系（imm対応）
- `async fn get_drag_force_imm() -> Option<VSVector3>`  
シップにかかっている抗力ベクトル

- `async fn get_lift_force_imm() -> Option<VSVector3>`  
シップにかかっている揚力ベクトル

#### 状態変更系 (Configのallow opのとき)
- `async fn enable_drag()`

- `async fn disable_drag()`

- `async fn enable_lift()`

- `async fn disable_lift()`

- `async fn enable_rot_drag()`

- `async fn disable_rot_drag()`

- `async fn set_wind_direction(x: f64, y: f64, z: f64)`

- `async fn set_wind_speed(speed: f64)`

- `async fn apply_wind_impulse(x: f64, y: f64, z: f64, speed: f64)`  
注：元MODにバグあるらしい。[drag_api.md](./lua_api/CC-VS/drag_api.md)を参照し、再度調査した上で慎重に実装

# 3: Some-Peripherals
ディレクトリ名：`some_peripherals`

## BallísticAccelerator
### 実装するもの

- `struct SPCoordinate {
    x: f64,
    y: f64,
    z: f64,
}`

- `struct SPTimeResult {
    ticks: f64,
    aux: f64,
}`

- `struct SPPitchResult {
    pitch: f64,
    aux: f64,
}`

- `struct SPDroneBlueprint {
    cannon: SPCoordinate,
    target: SPCoordinate,
    speed: f64,
    length: f64,
}`

- `struct BallisticAccelerator`

- `async fn time_in_air_imm(y_proj: f64, y_tgt: f64, y_vel: f64, gravity: Option<f64>, drag: Option<f64>, max_steps: Option<u32>) -> SPTimeResult`  
imm対応。射出体が目標Y座標に到達するまでの飛行時間を計算

- `async fn try_pitch_imm(pitch: f64, speed: f64, length: f64, dist: f64, cannon: SPCoordinate, target: SPCoordinate, gravity: Option<f64>, drag: Option<f64>, max_steps: Option<u32>) -> (f64, f64, f64)`  
imm対応。指定ピッチ角を試す。失敗時は (-1.0, -1.0, -1.0)

- `async fn calculate_pitch_imm(cannon: SPCoordinate, target: SPCoordinate, speed: f64, length: f64, amin: Option<f64>, amax: Option<f64>, gravity: Option<f64>, drag: Option<f64>, max_delta_t_error: Option<f64>, max_steps: Option<u32>, num_iterations: Option<u32>, num_elements: Option<u32>, check_impossible: Option<bool>) -> SPPitchResult`  
imm対応

- `async fn batch_calculate_pitches(cannon: SPCoordinate, targets: Vec<SPCoordinate>, speed: f64, length: f64, amin: Option<f64>, amax: Option<f64>, gravity: Option<f64>, drag: Option<f64>, max_delta_t_error: Option<f64>, max_steps: Option<u32>, num_iterations: Option<u32>, num_elements: Option<u32>, check_impossible: Option<bool>) -> Vec<SPPitchResult>`

- `async fn get_drag_imm(base_drag: f64, dim_drag_multiplier: f64) -> f64`  
imm対応

## Digitizer
### 実装するもの

- `struct SPItemData {
    id: String,
    count: u32,
    tag: serde_json::Value,
    // その他フィールド
}`

- `struct SPDigitizedItem {
    uuid: String,
}`

- `struct Digitizer`

- `async fn digitize_amount(amount: Option<u32>) -> Result<String, String>`  
スロット0のアイテムをデジタル化してUUIDを返す

- `async fn rematerialize_amount(uuid: &str, amount: Option<u32>) -> Result<bool, String>`  
デジタルアイテムを物理化してスロット0に戻す

- `async fn merge_digital_items(into_uuid: &str, from_uuid: &str, amount: Option<u32>) -> Result<bool, String>`  
2つのデジタルアイテムを合成

- `async fn separate_digital_item(from_uuid: &str, amount: u32) -> Result<String, String>`  
デジタルアイテムスタックを分割し、新UUIDを返す

- `async fn check_id(uuid: &str) -> Result<SPItemData, String>`  
UUIDが存在するか確認

- `async fn get_item_in_slot() -> Result<SPItemData, String>`  
スロット0のアイテム情報を返す

- `async fn get_item_limit_in_slot() -> Result<u32, String>`  
スロット0のアイテム上限数を返す

## GoggleLinkPort
### 実装するもの

- `struct Goggle {
    id: String,
    // その他Goggle固有のデータ
}`

- `struct GoggleLinkPort`

- `async fn get_connected() -> Result<HashMap<String, GoggleHandle>, String>`  
接続中のGogglesテーブルを返す。各エントリはGoggleの操作関数セット

## Radar
### 実装するもの

- `struct SPEntityInfo {
    x: f64,
    y: f64,
    z: f64,
    id: String,
    entity_type: String,
    name: String,
}`

- `struct SPShipInfo {
    is_ship: bool,
    ship_id: i64,
    pos: SPCoordinate,
    mass: f64,
    rotation: VSQuaternion,
    velocity: SPCoordinate,
    size: SPCoordinate,
    scale: SPCoordinate,
    moment_of_inertia_tensor: [[f64; 3]; 3],
    center_of_mass_in_ship: SPCoordinate,
}`

- `struct Radar`

- `async fn scan_for_entities(radius: f64) -> Result<Vec<SPEntityInfo>, String>`  
エンティティのみスキャン

- `async fn scan_for_ships(radius: f64) -> Result<Vec<SPShipInfo>, String>`  
VSシップのみスキャン。

- `async fn scan_for_players(radius: f64) -> Result<Vec<SPEntityInfo>, String>`  
プレイヤーのみスキャン

- `async fn get_config_info_imm() -> HashMap<String, String>`  
imm対応。現在の設定情報を返す

## Raycaster
### 実装するもの

- `struct SPRaycastResult {
    is_block: Option<bool>,
    is_entity: Option<bool>,
    abs_pos: Option<SPCoordinate>,
    hit_pos: Option<SPCoordinate>,
    distance: Option<f64>,
    block_type: Option<String>,
    rel_hit_pos: Option<SPCoordinate>,
    id: Option<String>,
    description_id: Option<String>,
    ship_id: Option<i64>,
    hit_pos_ship: Option<SPCoordinate>,
    error: Option<String>,
}`

- `struct SPRaycastHandle {
    // 非同期制御オブジェクト
}`

- `struct Raycaster`

- `async fn raycast(distance: f64, variables: Option<(f64, f64, Option<f64>)>, euler_mode: Option<bool>, im_execute: Option<bool>, check_for_blocks: Option<bool>, only_distance: Option<bool>) -> Result<SPRaycastResult, String>`  
デフォルトではim_executeはtrueで同期実行。falseの場合は制御オブジェクトを返す

- `async fn add_stickers(state: bool)`  
powered ブロックステートを設定

- `async fn get_config_info_imm() -> HashMap<String, String>`  
imm対応

- `async fn get_facing_direction_imm() -> String`  
imm対応。ブロックの向き("north"/"south"/"east"/"west"/"up"/"down")を返す

## WorldScanner
### 実装するもの

- `struct SPBlockInfo {
    block_type: String,
    ship_id: Option<i64>,
}`

- `struct WorldScanner`

# 4: Toms-Peripherals
ディレクトリ名：`toms_peripherals`

## GPU
### 実装するもの

- `struct TMImage {
    width: u32,
    height: u32,
    data: Vec<u32>,  // ARGB形式
}`

- `struct TMWindow {
    x: f64,
    y: f64,
    width: u32,
    height: u32,
}`

- `struct GPU`

- `async fn set_size(pixels: u32)`

- `async fn refresh_size()`

- `async fn get_size_imm() -> (u32, u32, u32, u32, u32)`  
imm対応。(ピクセル幅, 高さ, モニター列数, 行数, ピクセルサイズ)を返す

- `async fn fill(r: f32, g: f32, b: f32, a: f32)`

- `async fn sync()`

- `async fn filled_rectangle(x: u32, y: u32, w: u32, h: u32, r: f32, g: f32, b: f32, a: f32)`

- `async fn draw_image(image: TMImage, x: u32, y: u32)`

- `async fn draw_text(text: &str, x: u32, y: u32, r: f32, g: f32, b: f32, a: f32)`

- `async fn draw_char(char: char, x: u32, y: u32, r: f32, g: f32, b: f32, a: f32)`

- `async fn get_text_length_imm(text: &str) -> u32`  
imm対応

- `async fn set_font(font_name: &str)`

- `async fn clear_chars()`

- `async fn add_new_char(codepoint: u32, data: Vec<u8>)`

- `async fn create_window_imm(x: u32, y: u32, w: u32, h: u32) -> TMWindow`  
imm対応

- `async fn decode_image(data: &str) -> Result<TMImage, String>`

- `async fn new_image_imm(w: u32, h: u32) -> TMImage`  
imm対応

## Keyboard
### 実装するもの

- `struct Keyboard`

- `async fn set_fire_native_events(enabled: bool)`

## RedstonePort
### 実装するもの

- `struct RedstonePort`

- `async fn get_sides_imm() -> Vec<String>`  
imm対応

- `async fn get_input(side: &str) -> bool`

- `async fn get_analog_input(side: &str) -> u8`

- `async fn get_bundled_input(side: &str) -> u16`

- `async fn get_output_imm(side: &str) -> bool`  
imm対応

- `async fn get_analog_output_imm(side: &str) -> u8`  
imm対応

- `async fn get_bundled_output_imm(side: &str) -> u16`  
imm対応

- `async fn set_output(side: &str, value: bool)`

- `async fn set_analog_output(side: &str, value: u8)`

- `async fn set_bundled_output(side: &str, mask: u16)`

- `async fn test_bundled_input(side: &str, mask: u16) -> bool`

## WatchDogTimer
### 実装するもの

- `struct WatchDogTimer`

- `async fn is_enabled_imm() -> bool`  
imm対応

- `async fn get_timeout_imm() -> u32`  
imm対応。ticks単位

- `async fn set_enabled(enabled: bool)`

- `async fn set_timeout(ticks: u32)`

- `async fn reset()`

# 5: Clockwork_CC_Compat
ディレクトリ名：`clockwork_cc_compat`

## Boiler
### 実装するもの

- `struct CLFluidInfo {
    fluid: String,
    amount: u32,
    capacity: u32,
}`

- `struct CLPosition {
    x: i32,
    y: i32,
    z: i32,
}`

- `struct Boiler`

- `async fn is_active_imm() -> bool`  
imm対応

- `async fn get_heat_level_imm() -> f64`  
imm対応

- `async fn get_active_heat_imm() -> f64`  
imm対応

- `async fn is_passive_heat_imm() -> bool`  
imm対応

- `async fn get_water_supply_imm() -> f64`  
imm対応

- `async fn get_attached_engines_imm() -> u32`  
imm対応

- `async fn get_attached_whistles_imm() -> u32`  
imm対応

- `async fn get_engine_efficiency_imm() -> f64`  
imm対応

- `async fn get_boiler_size_imm() -> f64`  
imm対応

- `async fn get_width_imm() -> u32`  
imm対応

- `async fn get_height_imm() -> u32`  
imm対応

- `async fn get_max_heat_for_size_imm() -> f64`  
imm対応

- `async fn get_max_heat_for_water_imm() -> f64`  
imm対応

- `async fn get_fill_state_imm() -> f64`  
imm対応。0.0～1.0

- `async fn get_fluid_contents_imm() -> CLFluidInfo`  
imm対応

- `async fn get_controller_pos_imm() -> CLPosition`  
imm対応

## GasEngine（他ガス関連ペリフェラルは特に重要なメソッドのみ）
### 実装するもの

- `struct GasEngine`

- `async fn get_attached_engines_imm() -> u32`  
imm対応

- `async fn get_total_efficiency_imm() -> f64`  
imm対応

# 6: Create
ディレクトリ名：`create`  
プレフィックス：`CR`  
ソースドキュメント：`docs/lua_api/Create/`

## 共通構造体

- `struct CRItemDetail`  
  CC-Tweaked の `ItemDetail` と同等（`VanillaDetailRegistries.ITEM_STACK` 由来）。  
  `{ name: String, count: u32, display_name: String, tags: HashMap<String, bool>, ... }`

- `struct CRSlotInfo`  
  インベントリ一覧用。`{ name: String, count: u32 }`

- `struct CROrderItem`  
  RedstoneRequester / TableClothShop のリクエスト指定用。  
  `{ name: String, count: u32 }` — count は 1〜256

- `struct CRItemFilter`  
  StockTicker の requestFiltered 用。  
  `{ name: Option<String>, request_count: Option<u32>, ... }`  
  追加フィールドは CRItemDetail の各フィールドに対応

- `struct CRSignalParams`  
  NixieTube の setSignal 用発光パラメータ。  
  `{ r: Option<u8>, g: Option<u8>, b: Option<u8>, glow_width: Option<u8>, glow_height: Option<u8>, blink_period: Option<u8>, blink_off_time: Option<u8> }`

- `struct CRPackage`  
  Frogport / Packager / Postbox / Repackager のパッケージハンドル。  
  `{ address: String }` — 内部で対応 ItemStack を保持

## CreativeMotor
`docs/lua_api/Create/creative_motor.md` 参照
### 実装するもの

- `struct CreativeMotor`

- `async fn set_generated_speed(speed: i32)`

- `async fn get_generated_speed() -> f32`  
  imm対応

## Speedometer
`docs/lua_api/Create/speedometer.md` 参照
### 実装するもの

- `struct Speedometer`

- `async fn get_speed() -> f32`  
  imm対応

- `async fn try_pull_speed_change() -> Option<f32>`  
  1tick待機。来なければ None
- `async fn pull_speed_change() -> f32`

## Stressometer
`docs/lua_api/Create/stressometer.md` 参照
### 実装するもの

- `struct Stressometer`

- `async fn get_stress() -> f32`  
  imm対応

- `async fn get_stress_capacity() -> f32`  
  imm対応

- `async fn try_pull_overstressed() -> Option<()>`
- `async fn pull_overstressed()`

- `async fn try_pull_stress_change() -> Option<(f32, f32)>`  
  `(stress, capacity)`
- `async fn pull_stress_change() -> (f32, f32)`

## RotationSpeedController
`docs/lua_api/Create/speed_controller.md` 参照
### 実装するもの

- `struct RotationSpeedController`

- `async fn set_target_speed(speed: i32)`

- `async fn get_target_speed() -> f32`  
  imm対応

## SequencedGearshift
`docs/lua_api/Create/sequenced_gearshift.md` 参照
### 実装するもの

- `struct SequencedGearshift`

- `async fn rotate(amount: i32, speed_modifier: Option<i32>)`

- `async fn move_by(distance: i32, speed_modifier: Option<i32>)`

- `async fn is_running() -> bool`  
  imm対応

## NixieTube
`docs/lua_api/Create/nixie_tube.md` 参照
### 実装するもの

- `struct NixieTube`

- `async fn set_text(text: String, colour: Option<String>)`

- `async fn set_text_colour(colour: String)`

- `async fn set_signal(front: CRSignalParams, back: Option<CRSignalParams>)`

## DisplayLink
`docs/lua_api/Create/display_link.md` 参照
### 実装するもの

- `struct DisplayLink`

- `async fn set_cursor_pos(x: u32, y: u32) -> Result<(), String>`  
  1未満ならエラー

- `async fn get_cursor_pos() -> (u32, u32)`  
  imm対応

- `async fn get_size() -> (u32, u32)`  
  mainThread=true のため imm非対応

- `async fn is_color() -> bool`  
  imm対応。常に false

- `async fn write(text: String)`

- `async fn write_bytes(data: Vec<u8>)`

- `async fn clear_line()`

- `async fn clear()`

- `async fn update()`

## Sticker
`docs/lua_api/Create/sticker.md` 参照
### 実装するもの

- `struct Sticker`

- `async fn is_extended() -> bool`  
  imm対応

- `async fn is_attached_to_block() -> bool`  
  imm対応

- `async fn extend() -> bool`

- `async fn retract() -> bool`

- `async fn toggle() -> bool`

## Station
`docs/lua_api/Create/station.md` 参照
### 実装するもの

スケジュールは NBT 由来の再帰的テーブル構造。Rust 側では `HashMap<String, serde_json::Value>` 等の動的型で扱う。

- `struct Station`

- `async fn assemble() -> Result<(), String>`

- `async fn disassemble() -> Result<(), String>`

- `async fn set_assembly_mode(assembly_mode: bool) -> Result<(), String>`

- `async fn is_in_assembly_mode() -> bool`  
  imm対応

- `async fn get_station_name() -> Result<String, String>`  
  imm対応

- `async fn set_station_name(name: String) -> Result<(), String>`

- `async fn is_train_present() -> Result<bool, String>`  
  imm対応

- `async fn is_train_imminent() -> Result<bool, String>`  
  imm対応

- `async fn is_train_enroute() -> Result<bool, String>`  
  imm対応

- `async fn get_train_name() -> Result<String, String>`  
  imm対応

- `async fn set_train_name(name: String) -> Result<(), String>`

- `async fn has_schedule() -> Result<bool, String>`  
  imm対応

- `async fn get_schedule() -> Result<HashMap<String, serde_json::Value>, String>`  
  imm対応。スケジュールは動的テーブル

- `async fn set_schedule(schedule: HashMap<String, serde_json::Value>) -> Result<(), String>`

- `async fn can_train_reach(destination_filter: String) -> Result<(bool, Option<String>), String>`  
  imm対応。第2戻り値: 到達不可理由 "cannot-reach" / "no-target"

- `async fn distance_to(destination_filter: String) -> Result<(Option<f64>, Option<String>), String>`  
  imm対応。到達不可の場合は `(None, Some(理由))`

- `async fn try_pull_train_arrive() -> Option<String>`
- `async fn pull_train_arrive() -> String`

- `async fn try_pull_train_depart() -> Option<String>`
- `async fn pull_train_depart() -> String`

## Signal
`docs/lua_api/Create/signal.md` 参照
### 実装するもの

- `struct Signal`

- `async fn get_state() -> String`  
  imm対応。例: "GREEN", "YELLOW", "RED"

- `async fn is_forced_red() -> bool`  
  imm対応

- `async fn set_forced_red(powered: bool)`

- `async fn list_blocking_train_names() -> Result<Vec<String>, String>`  
  imm対応

- `async fn get_signal_type() -> Result<String, String>`  
  imm対応。"ENTRY_SIGNAL" または "CROSS_SIGNAL"

- `async fn cycle_signal_type() -> Result<(), String>`

- `async fn try_pull_train_signal_state_change() -> Option<String>`
- `async fn pull_train_signal_state_change() -> String`

## TrackObserver
`docs/lua_api/Create/track_observer.md` 参照
### 実装するもの

- `struct TrackObserver`

- `async fn is_train_passing() -> bool`  
  imm対応

- `async fn get_passing_train_name() -> Option<String>`  
  imm対応

- `async fn try_pull_train_passing() -> Option<String>`
- `async fn pull_train_passing() -> String`

- `async fn try_pull_train_passed() -> Option<String>`
- `async fn pull_train_passed() -> String`

## Frogport
`docs/lua_api/Create/frogport.md` 参照
### 実装するもの

全メソッドが mainThread=true のため imm非対応。

- `struct Frogport`

- `async fn set_address(address: String)`

- `async fn get_address() -> String`

- `async fn get_configuration() -> Option<String>`  
  "send_recieve" / "send" / None（接続先なし）

- `async fn set_configuration(config: String) -> Result<bool, String>`

- `async fn list() -> HashMap<u32, CRSlotInfo>`

- `async fn get_item_detail(slot: u32) -> Option<CRItemDetail>`

- `async fn try_pull_package_received() -> Option<CRPackage>`
- `async fn pull_package_received() -> CRPackage`
- `async fn try_pull_package_sent() -> Option<CRPackage>`
- `async fn pull_package_sent() -> CRPackage`

## Packager
`docs/lua_api/Create/packager.md` 参照
### 実装するもの

- `struct Packager`

- `async fn make_package() -> bool`

- `async fn list() -> HashMap<u32, CRSlotInfo>`

- `async fn get_item_detail(slot: u32) -> Option<CRItemDetail>`

- `async fn get_address() -> String`

- `async fn set_address(address: Option<String>)`

- `async fn get_package() -> Option<CRPackage>`

- `async fn try_pull_package_received() -> Option<CRPackage>`
- `async fn pull_package_received() -> CRPackage`
- `async fn try_pull_package_sent() -> Option<CRPackage>`
- `async fn pull_package_sent() -> CRPackage`

## Postbox
`docs/lua_api/Create/postbox.md` 参照
### 実装するもの

- `struct Postbox`

- `async fn set_address(address: String)`

- `async fn get_address() -> String`

- `async fn list() -> HashMap<u32, CRSlotInfo>`

- `async fn get_item_detail(slot: u32) -> Option<CRItemDetail>`

- `async fn get_configuration() -> Option<String>`

- `async fn set_configuration(config: String) -> Result<bool, String>`

- `async fn try_pull_package_received() -> Option<CRPackage>`
- `async fn pull_package_received() -> CRPackage`
- `async fn try_pull_package_sent() -> Option<CRPackage>`
- `async fn pull_package_sent() -> CRPackage`

## Repackager
`docs/lua_api/Create/repackager.md` 参照
### 実装するもの

- `struct Repackager`

- `async fn make_package() -> bool`

- `async fn list() -> HashMap<u32, CRSlotInfo>`

- `async fn get_item_detail(slot: u32) -> Option<CRItemDetail>`

- `async fn get_address() -> String`

- `async fn set_address(address: Option<String>)`

- `async fn get_package() -> Option<CRPackage>`

- `async fn try_pull_package_repackaged() -> Option<(CRPackage, u32)>`  
  `(package, count)`
- `async fn pull_package_repackaged() -> (CRPackage, u32)`

- `async fn try_pull_package_received() -> Option<CRPackage>`
- `async fn pull_package_received() -> CRPackage`
- `async fn try_pull_package_sent() -> Option<CRPackage>`
- `async fn pull_package_sent() -> CRPackage`

## RedstoneRequester
`docs/lua_api/Create/redstone_requester.md` 参照
### 実装するもの

- `struct RedstoneRequester`

- `async fn request()`

- `async fn set_request(items: Vec<Option<CROrderItem>>)`  
  最大9スロット。None で空スロット

- `async fn set_crafting_request(count: u32, items: Vec<Option<CROrderItem>>)`

- `async fn get_request() -> HashMap<u32, CRItemDetail>`

- `async fn get_configuration() -> String`  
  "allow_partial" / "strict"

- `async fn set_configuration(config: String) -> Result<(), String>`

- `async fn set_address(address: String)`

- `async fn get_address() -> String`

## StockTicker
`docs/lua_api/Create/stock_ticker.md` 参照
### 実装するもの

- `struct StockTicker`

- `async fn stock(detailed: bool) -> HashMap<u32, CRItemDetail>`

- `async fn get_stock_item_detail(slot: u32) -> Option<CRItemDetail>`

- `async fn request_filtered(address: String, filters: Vec<CRItemFilter>) -> u32`  
  フィルタに合致するアイテムを配送要求。戻り値は送信したアイテム総数。

- `async fn list() -> HashMap<u32, CRSlotInfo>`  
  受信済み支払いインベントリ

- `async fn get_item_detail(slot: u32) -> Option<CRItemDetail>`

## TableClothShop
`docs/lua_api/Create/tablecloth_shop.md` 参照
### 実装するもの

- `struct TableClothShop`

- `async fn is_shop() -> bool`

- `async fn get_address() -> Result<String, String>`

- `async fn set_address(address: String) -> Result<(), String>`

- `async fn get_price_tag_item() -> Result<CRItemDetail, String>`

- `async fn set_price_tag_item(item_name: Option<String>) -> Result<(), String>`

- `async fn get_price_tag_count() -> Result<u32, String>`

- `async fn set_price_tag_count(count: Option<u32>) -> Result<(), String>`  
  1〜100。None で 1

- `async fn get_wares() -> Result<HashMap<u32, CRItemDetail>, String>`

- `async fn set_wares(items: Vec<Option<CROrderItem>>) -> Result<(), String>`  
  最大9スロット。インベントリが空でないとエラー

# 7: createaddition
ディレクトリ名：`createaddition`

## ElectricMotor
### 実装するもの

- `struct ElectricMotor`

- `async fn get_type_imm() -> String`  
imm対応。"electric_motor"を返す

- `async fn set_speed(speed: f64)`  
RPM設定（符号で方向制御）

- `async fn stop()`

- `async fn get_speed() -> f64`

- `async fn get_stress_capacity() -> f64`

- `async fn get_energy_consumption() -> f64`

- `async fn rotate(degrees: f64, rpm: Option<f64>) -> f64`  
指定角度回転。所要秒数を返す

- `async fn translate(distance: f64, rpm: Option<f64>) -> f64`  
指定距離移動。所要秒数を返す

- `async fn get_max_insert() -> f64`

- `async fn get_max_extract() -> f64`

# 8: Control-Craft
ディレクトリ名：`control_craft`

## Camera
### 実装するもの

- `struct CTLTransform {
    matrix: [[f64; 4]; 4],
}`

- `struct CTLRaycastResult {
    // RaycastResult構造化データ
}`

- `struct Camera`

- `async fn get_abs_view_transform_imm() -> CTLTransform`  
imm対応

- `async fn clip() -> Result<CTLRaycastResult, String>`

- `async fn clip_entity() -> Result<CTLRaycastResult, String>`

- `async fn clip_block() -> Result<CTLRaycastResult, String>`

- `async fn clip_all_entity() -> Result<Vec<CTLRaycastResult>, String>`

- `async fn clip_ship() -> Result<CTLRaycastResult, String>`

- `async fn clip_player() -> Result<CTLRaycastResult, String>`

- `async fn set_pitch(degrees: f64)`

- `async fn set_yaw(degrees: f64)`

- `async fn get_pitch_imm() -> f64`  
imm対応

- `async fn get_yaw_imm() -> f64`  
imm対応

- `async fn get_transformed_pitch_imm() -> f64`  
imm対応

- `async fn get_transformed_yaw_imm() -> f64`  
imm対応

- `async fn outline_to_user()`

- `async fn force_pitch_yaw(pitch: f64, yaw: f64)`

- `async fn get_clip_distance_imm() -> f64`  
imm対応

- `async fn set_clip_range(range: f64)`

- `async fn set_cone_angle(angle: f64)`

- `async fn latest_ship_imm() -> Option<ShipInfo>`  
imm対応

- `async fn latest_player_imm() -> Option<PlayerInfo>`  
imm対応

- `async fn latest_entity_imm() -> Option<EntityInfo>`  
imm対応

- `async fn latest_block_imm() -> Option<BlockInfo>`  
imm対応

- `async fn raycast(x: f64, y: f64, z: f64) -> Result<CTLRaycastResult, String>`

- `async fn get_entities(radius: f64) -> Result<Vec<EntityInfo>, String>`

- `async fn get_mobs(radius: f64) -> Result<Vec<EntityInfo>, String>`

- `async fn get_camera_position_imm() -> (f64, f64, f64)`  
imm対応

- `async fn get_abs_view_forward_imm() -> (f64, f64, f64)`  
imm対応

- `async fn is_being_used_imm() -> bool`  
imm対応

- `async fn get_direction_imm() -> String`  
imm対応

- `async fn reset()`

# 9: AdvancedPeripherals
ディレクトリ名：`advanced_peripherals`

## InventoryManager
### 実装するもの

- `struct ADItemEntry {
    name: String,
    tags: Vec<String>,
    count: u32,
    display_name: String,
    max_stack_size: u32,
    components: serde_json::Value,
    fingerprint: String,
    slot: Option<u32>,
}`

- `struct InventoryManager`

- `async fn get_owner_imm() -> String`  
imm対応

- `async fn add_item_to_player(slot: u32, count: Option<u32>) -> u32`

- `async fn remove_item_from_player(slot: u32, count: Option<u32>) -> u32`

- `async fn list() -> Vec<ADItemEntry>`

- `async fn get_armor() -> Vec<ADItemEntry>`

- `async fn is_player_equipped() -> bool`

- `async fn is_wearing(slot: u32) -> bool`

- `async fn get_item_in_hand() -> Result<ADItemEntry, String>`

- `async fn get_item_in_off_hand() -> Result<ADItemEntry, String>`

- `async fn get_empty_space() -> u32`

- `async fn is_space_available() -> bool`

- `async fn get_free_slot() -> i32`  
最初の空きスロット。なければ-1

- `async fn list_chest() -> Vec<ADItemEntry>`

## PlayerDetector
### 実装するもの

- `struct ADPlayerInfo {
    x: f64,
    y: f64,
    z: f64,
    name: Option<String>,
    uuid: Option<String>,
    health: Option<f64>,
    max_health: Option<f64>,
    is_flying: Option<bool>,
    is_sprinting: Option<bool>,
    is_sneaking: Option<bool>,
    game_mode: Option<String>,
    experience: Option<u32>,
    level: Option<u32>,
    pitch: Option<f64>,
}`

- `struct PlayerDetector`

- `async fn get_online_players() -> Vec<String>`

- `async fn get_players_in_range(radius: f64) -> Vec<String>`

- `async fn get_players_in_coords(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Vec<String>`

- `async fn get_players_in_cubic(dx: f64, dy: f64, dz: f64) -> Vec<String>`

- `async fn is_players_in_range(radius: f64) -> bool`

- `async fn is_players_in_coords(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> bool`

- `async fn is_players_in_cubic(dx: f64, dy: f64, dz: f64) -> bool`

- `async fn is_player_in_range(player: &str, radius: f64) -> bool`

- `async fn is_player_in_coords(player: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> bool`

- `async fn is_player_in_cubic(player: &str, dx: f64, dy: f64, dz: f64) -> bool`

- `async fn get_player_pos(player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, String>`

- `async fn get_player(player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, String>`

