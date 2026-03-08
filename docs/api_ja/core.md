# コアインフラAPI

**v0.2.0 以降 (book-read パターン)**

## 概要

Rust コンピューターの実行制御とペリフェラル通信の中核を担うコアインフラです。  
book-read パターンにより、**1ループ = 1 tick** の効率的なプログラミングを実現します。

## 核となるコンポーネント

| 項目 | 説明 |
|---|---|
| `wait_for_next_tick()` | 次 Game Tick まで待機する Future。全予約リクエストを FFI 一括発行、結果回収 |
| `book_next_*()` / `read_last_*()` | ペリフェラルメソッドのペア。リクエスト予約・結果読み取り |
| BookStore | グローバル状態管理 (pending/in_flight/results) |
| PeriphAddr | ペリフェラルアドレス (u32: 0-5=直接接続, 6+=有線モデム) |

---

## 実行フロー

### tick内の動作

```
GT N [Rust]
  ├─ read_last_*()     ← 前tick結果を取得
  ├─ 計算処理
  └─ book_next_*()     ← 次tick予約

  【待機点】
  wait_for_next_tick().await
    ├─ Poll 1回目: pending FFI発行 (flush)
    └─ Poll 2回目以降: 結果ポーリング (poll_all)

GT N+1 [Java]
  ├─ リクエスト実行
  └─ 結果をbookstore結果map に格納

GT N+1 [Rust]
  ├─ read_last_*()で取得可能
  └─ ループ繰り返し
```

### メモリタイムライン

```
GT N:
  ┌──────────────────┐
  │ book_next_A()    │  ← pending {A}
  │ book_next_B()    │  ← pending {A, B}
  │ wait_for...await │
  └──────────────────┘
        ↓ (first poll)
  ┌──────────────────┐
  │ flush()          │  ← in_flight {A, B} (FFI発行)
  │ Poll::Pending    │
  └──────────────────┘

GT N +1:
  ┌──────────────────┐
  │ wait_for...await │
  └──────────────────┘
        ↓ (poll again)
  ┌──────────────────┐
  │ poll_all()       │  ← results {A→data, B→data}
  │ Poll::Ready(())  │
  └──────────────────┘
```

---

## API リファレンス

### `wait_for_next_tick()`

次 Game Tick まで待機する Future を返します。

```rust
pub fn wait_for_next_tick() -> WaitForNextTickFuture
```

**使用パターン:**
```rust
loop {
    // 前 tick 結果取得
    let data = sensor.read_last_get_data()?;
    
    // リクエスト予約
    sensor.book_next_get_data();
    
    // tick 待機（全予約を FFI 一括発行）
    wait_for_next_tick().await;
}
```

**内部動作:**
- **初回 poll**: `BookStore::flush()` → pending → in_flight に移動、FFI 発行（`host_request_info` / `host_do_action`）
- **2回目以降 poll**: `BookStore::poll_all()` → `host_poll_result` で全リクエストをポーリング、完了したら `results` map に移動
- **全完了時**: `Poll::Ready(())` を返し、ループが進む

---

### `PeriphAddr`

ペリフェラルのアドレスを表します。

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeriphAddr(u32);

impl PeriphAddr {
    pub fn raw(&self) -> u32 { self.0 }
}

impl From<Direction> for PeriphAddr { ... }
```

**値の意味:**
|値 | 接続方式 | 説明 |
|---|---|---|
| 0-5 | 直接隣接 | `Direction::{Down, Up, North, South, West, East}` |
| 6+ | 有線モデム | 有線ネットワーク経由のペリフェラル |

**例:**
```rust
// 直接隣接
let monitor = Monitor::from_direction(Direction::South);  // PeriphAddr(3)

// 有線モデム経由（find_imm で自動検索）
let all_radars = find_imm::<Radar>()?;  // PeriphAddr(6), PeriphAddr(7), ...
```

---

### Book-Read メソッドペア

すべてのペリフェラル操作は `book_next_*` と `read_last_*` のペアで構成されます。

#### `book_next_*(&mut self, ...)`

次 tick のリクエストを**予約**します（FFI 呼び出しではなく、ローカルバッファに記録）。

```rust
pub fn book_next_get_data(&mut self) { ... }
pub fn book_next_set_speed(&mut self, speed: f64) { ... }
```

**特性:**
- 即座に戻る（non-blocking）
- `&mut self` が必要
- 同じメソッドへの複数予約は**上書き** (最後の予約が有効)
- `wait_for_next_tick().await` で初めて FFI 発行

**例:**
```rust
// 複数予約
radar.book_next_scan_for_entities(100.0);
motor.book_next_set_speed(50.0);

// 待機
wait_for_next_tick().await;

// 両方の結果が揃っている
let entities = radar.read_last_scan_for_entities()?;
let ack = motor.read_last_set_speed()?;
```

#### `read_last_*(&self) -> Result<T, PeripheralError>`

前 tick のリクエスト**結果**を読み取ります。

```rust
pub fn read_last_get_data(&self) -> Result<Data, PeripheralError> { ... }
pub fn read_last_set_speed(&self) -> Result<(), PeripheralError> { ... }
```

**戻り値:**
- `Ok(data)`: リクエスト成功、結果を返す
- `Err(PeripheralError::NotRequested)`: `book_next_*` が未呼び出し
- `Err(PeripheralError::Bridge(_))`: FFI エラー
- `Err(PeripheralError::DecodeFailed)`: 結果パース失敗

**例:**
```rust
// 初回ループ
match radar.read_last_scan_for_entities() {
    Ok(entities) => { /* 処理 */ },
    Err(PeripheralError::NotRequested) => { /* 初回なので無視 */ },
    Err(e) => { eprintln!("Error: {}", e); },
}

// 次ループ以降
let entities = radar.read_last_scan_for_entities()?;  // Ok が期待される
```

---

### ペリフェラル検索 / ラップ

#### `find_imm<T>()`

指定型のペリフェラルを種類別に検索します（即時）。

```rust
pub fn find_imm<T: Peripheral>() -> Result<Vec<T>, PeripheralError>
```

**使用法:**
```rust
// モニタをすべて検索
let monitors: Vec<Monitor> = find_imm()?;

// 単一ペリフェラル取得
let radar = find_imm::<Radar>()?.into_iter().next().ok_or(...)?;

// 最初の N 個
let (motor, _rest) = find_imm::<ElectricMotor>()?
    .into_iter()
    .next()
    .map(|m| (m, ()))
    .ok_or(PeripheralError::NotFound)?;
```

**注意:**
- 有線モデム経由のペリフェラルも含まれる
- v0.2.0 で高速化（`host_find_peripherals_by_type_imm`）

#### `wrap_imm(periph_addr: PeriphAddr)` / `wrap(periph_addr: PeriphAddr)`

既知のアドレスでペリフェラルをラップします。

```rust
pub fn wrap_imm(addr: PeriphAddr) -> Result<T, PeripheralError>
pub fn wrap(addr: PeriphAddr) -> T  // unchecked
```

**例:**
```rust
// 有線モデム経由のペリフェラル
let radar_6 = wrap_imm::<Radar>(PeriphAddr(6))?;  // 存在確認
let radar_7 = wrap::<Radar>(PeriphAddr(7));       // 存在確認なし
```

---

## ペリフェラルエラー

```rust
pub enum PeripheralError {
    Bridge(BridgeError),        // FFI層エラー
    NotFound,                   // ペリフェラルなし
    DecodeFailed,               // 結果パース失敗
    Unexpected(String),         // その他
    NotRequested,               // read_last 前に book_next なし
}
```

### NotRequested エラーの扱い

初回ループでは `book_next_*` がまだ呼ばれていないため、`read_last_*` は `NotRequested` を返します。

```rust
loop {
    // 初回: NotRequested, 2回目以降: Ok(data)
    let data = match sensor.read_last_get_data() {
        Ok(d) => d,
        Err(PeripheralError::NotRequested) => {
            // 初回ループ、結果がまだない
            Default::default()
        },
        Err(e) => return Err(e),
    };

    // リクエスト予約
    sensor.book_next_get_data();
    wait_for_next_tick().await;
}
```

**パターン例:**
```rust
// Option で扱う
let data = sensor.read_last_get_data().ok();

// unwrap_or で初期値指定
let data = sensor.read_last_get_data().unwrap_or_default();

// ? で早期終了
let data = sensor.read_last_get_data()?;  // NotRequested は Err として伝播
```

---

## 即時メソッド (`_imm`)

`_imm` サフィックスのメソッドは同 tick 内で即座に実行されます（await 不要）。

```rust
// 同 tick 内で返る（ペリフェラル接続確認用など）
let kind = radar.get_type_imm()?;

// 書込結果も同 tick 内で確定
let written = inventory.push_items_imm(
    InventoryRef { side: Direction::South },
    &items,
    &[],
)?;
```

**特性:**
- 1 tick 遅れ原則の意図的な例外
- `host_request_info_imm` / `host_do_action_imm` で実装
- book-read パターン不要

---

## イベント待ち メソッド (async)

`receive_wait_raw` などのイベント駆動メソッドは `async` で提供されます。

```rust
// イベント発生まで待機
let msg = modem.receive_wait_raw().await?;
```

**内部実装:**
- 内部で book → `wait_for_next_tick()` → read をループ
- ユーザーからは `async` に見える
- `parallel!` で複数イベント同時待ち可能

```rust
let (msg, keystroke) = parallel!(
    modem.receive_wait_raw(),
    keyboard.read_line()
).await?;
```

---

## 完全な例

```rust
use rust_computers_api::prelude::*;

#[entry]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut radar = find_imm::<Radar>()?
        .into_iter()
        .next()
        .ok_or(PeripheralError::NotFound)?;
    
    let mut motor = find_imm::<ElectricMotor>()?
        .into_iter()
        .next()
        .ok_or(PeripheralError::NotFound)?;

    // 初期化予約
    radar.book_next_scan_for_entities(100.0);
    wait_for_next_tick().await;

    loop {
        // 前 tick の結果取得
        let entities = match radar.read_last_scan_for_entities() {
            Ok(e) => e,
            Err(PeripheralError::NotRequested) => Vec::new(),
            Err(e) => return Err(e.into()),
        };

        // 計算
        let target_speed = if entities.is_empty() {
            0.0
        } else {
            entities[0].distance * 10.0
        };

        // 設定
        motor.book_next_set_speed(target_speed);
        
        // スキャン予約
        radar.book_next_scan_for_entities(100.0);

        // tick 待機
        wait_for_next_tick().await;

        // ログ出力（_imm）
        let speed = motor.get_speed_imm()?;
        println!("Speed: {:.2}", speed);
    }
}
```

---

## パフォーマンス特性

| 操作 | cost | 説明 |
|---|---|---|
| `book_next_*` | O(1) | ローカルバッファ記録 |
| `read_last_*` | O(1) | グローバルマップ lookup |
| `wait_for_next_tick()` Poll 1 | FFI 呼 | 全ペンディング発行 |
| `wait_for_next_tick()` Poll 2+ | FFI 呼 | ポーリング |
| `_imm` | FFI 呼 | 即座に FFI発行 |

**最適化:**
- `book_*` は FFI 呼び出しなし（純ローカル）
- `wait_for_next_tick()` の最初の poll で全リクエストを**一括 FFI 発行**（並列化）
- `read_last_*` は メモリ lookup のみ

---

## 注意事項

### Peripheral::new() の廃止

v0.2.0 で `Peripheral::new()` は `#[doc(hidden)]` です。  
代わりに以下を使用してください：

```rust
// ❌ 使用禁止
let monitor = Monitor::new(PeriphAddr(3));

// ✅ 推奨
let monitors: Vec<Monitor> = find_imm()?;
let singles = find_imm::<Monitor>()?;

let specific = wrap_imm::<Radar>(PeriphAddr(6))?;
```

### 借用と &mut self

`book_next_*` は `&mut self` を要求するため、同時に複数の `book_next_*` 呼び出しはできません。

```rust
// ❌ エラー（2つめで再借用）
radar.book_next_scan_for_entities(100.0);
radar.book_next_scan_for_entities(200.0);  // 上書き

// ✅ OK（各ペリフェラルは独立）
radar.book_next_scan_for_entities(100.0);
motor.book_next_set_speed(50.0);
```

### 結果バッファ位置

現在、結果バッファは **Rust (WASM) 側**で確保されています。  
将来的に Java 側に移す可能性があります（パフォーマンス最適化のため）。

---

## 関連リファレンス

- [docs/spec.md](../spec.md) — 全体仕様・1 tick 遅れ原則・FFI 詳細
- [docs/api/](.) — 各ペリフェラルの API ドキュメント
