# RustComputers 設計仕様書

> 最終更新: 2026-03-04

## 概要

**RustComputers** は、Minecraft 1.20.1 Forge (47.4.10) 向けの Mod です。  
CC:Tweaked (ComputerCraft) の Java ペリフェラルシステムと互換性を保ちつつ、Lua の代わりに **Rust (WASM)** でコンピューターのプログラムを記述できるようにすることを目的とします。

---

## 1. 設計コンセプト

### 1.1 1tick 遅れ原則

すべての情報取得 API は **1 Game Tick (GT) 遅れ**で結果が返ります。  
これにより Rust 実行環境を完全に Java/Minecraft から隔離し、時間軸のねじれを排除します。

```
GT:N   [Rust]  → 情報取得リクエスト発行 & ワールド干渉指示発行
GT:N   [Java]  → ワールド干渉指示を実行し、結果を保存
GT:N+1 [Java]  → 前 tick のリクエストに対する情報を収集し Rust に渡す
GT:N+1 [Rust]  → 情報を受け取り、次の処理を進める
```

### 1.2 Async / Poll モデル

各コンピューターは独立した **WASM インスタンス** を持ち、1 tick に 1 回 poll されます。  
ユーザーは `async fn main()` を単一エントリーポイントとして記述します。

```rust
#[entry]
async fn main() {
    loop {
        let info = get_something().await;  // 1tick 待つ
        do_something(info);
    }
}
```

`await` ごとに 1 tick 消費します (明示的な待機)。  
並列取得には `parallel!` マクロ (内部的に `join!` 相当) を使用します。

```rust
let (a, b) = parallel!(getInfoA.get(), getInfoB.get());
// どちらも同一 tick でリクエストされるため、1tick 待つだけで両方取得できる
```

---

## 2. アーキテクチャ

### 2.1 1 tick 内のフェーズ

```
┌─────────────────────────────────────────────────────────┐
│  ServerTickEvent                                        │
│                                                         │
│  Phase 1: 前 tick リクエスト結果の収集 [Java]            │
│    └ 情報取得リクエストを処理し、結果を HashMap に格納   │
│                                                         │
│  Phase 2: Rust (WASM) の poll [Java → WASM]             │
│    └ 各コンピューターインスタンスを並列 poll             │
│    └ WASM が新たな情報リクエスト / 干渉指示を積む        │
│                                                         │
│  Phase 3: ワールド干渉指示の実行 [Java]                  │
│    └ 干渉系 API を実行し、結果を保存 (次 tick に渡す)    │
└─────────────────────────────────────────────────────────┘
```

### 2.2 返り値の扱い

| API 種別 | await の要否 | 結果取得タイミング |
|---|---|---|
| 情報取得系 (read) | **必須** | 次 tick (Phase 1) |
| ワールド干渉系 (write) | 任意 | 同 tick 実行 → 結果は次 tick |
| 返り値を捨てる干渉 | 不要 | Fire and forget |

---

## 3. WASM ランタイム

### 3.1 選定: Wasmtime (via JNI)

| 項目 | 仕様 |
|---|---|
| **ランタイム** | **Wasmtime** (Bytecode Alliance / Apache-2.0) |
| **JIT** | Cranelift (最高性能、形式検証済み) |
| **統合方式** | wasmtime-java (Maven / Prebuilt JNI) |
| **採用理由** | セキュリティ最高、 Minecraft Tick Loop での実行効率最高、長期メンテナンス |

### 3.2 インスタンス分離

- **コンピューター 1 台** = **WASM インスタンス 1 つ** (メモリ完全分離)
- 複数コンピューターは スレッドプール で並列 poll
- 1 コンピューター内は シングルスレッド

### 3.3 ランタイム選定の詳細

詳細比較は [docs/wasm-runtime-comparison.md](./wasm-runtime-comparison.md) を参照。

**対象候補:**
- Wasmtime (選定: ✅ 最高性能・最高セキュリティ・長期保守)
- Wasmer (検討: ⚠️ 公式Java JNI あるが保守停滞)
- GraalVM (検討: ⚠️ Java統合は簡単だが複雑性・メモリ overhead)

---

## 4. WASM ホスト関数 (Java ↔ Rust ブリッジ)

### 4.1 概要

Rust コンピューター（WASM）と Java（Minecraft）の相互呼び出しを実現する機構。

**原則**:
- **情報取得 (Read)**: 1 Tick 遅延 (GT:N リクエスト → GT:N+1 結果)
- **ワールド干渉 (Write)**: 同様に 1 Tick 遅延
- **すべて Future ベース**: `parallel!` マクロで複数操作を一括待機
- **タイムアウト&Fuel上限**: 悪意あるプログラム対策

### 4.2 ホスト関数の種類

#### 情報取得系

```
host_request_info(peripheral_id, method_id, arg_count, args_ptr)
  → request_id (>0) or error_code (<0)
  
host_poll_result(request_id)
  → result_addr (>0: ready) or 0 (pending) or error_code (<0)
```

#### ワールド干渉系

```
host_do_action(peripheral_id, method_id, arg_count, args_ptr)
  → request_id (>0) or error_code (<0)
  
host_poll_result(request_id)  // 同じ polling メカニズム
  → result_addr (>0: ready) or 0 (pending) or error_code (<0)
```

#### Mod 確認関数

```
host_is_mod_available(mod_id)
  → 1 (available) or 0 (not available)
```

### 4.3 Shared Buffer

**固定サイズ: 64 KB** (案①採用)

- 引数領域 (16 KB): Rust が Java にリクエストを送る際のパラメータ
- 結果領域 1, 2 (各 8 KB): ダブルバッファで Java が結果を返す
- メタデータ領域 (8 KB): Request state table, timestamp 等
- 予約 (22 KB): 今後の拡張用

将来的に案②（動的割り当て）へ移行可能。その場合は allocate/deallocate を WASM モジュール側でエクスポート。

### 4.4 詳細設計

**[docs/w2-java-wasm-bridge.md](./w2-java-wasm-bridge.md) を参照**

内容:
- Request ID 管理戦略（シンプルインクリメント）
- エラーコード体系（-9 種）
- Thread safety（Synchronized state machine）
- Timeout（100 ticks）& Fuel limit（10M/tick）
- Feature-based mod detection
- 実装イメージ（Java + Rust）

---

## 5. `rust_computers` クレート

### 5.1 構造

```
rust_computers/
├── src/
│   ├── lib.rs
│   ├── runtime/          # WASM ランタイム共通抽象
│   ├── computer_craft/   # CC:Tweaked バインディング
│   │   ├── monitor.rs
│   │   ├── disk_drive.rs
│   │   └── ...
│   ├── some_peripherals/ # Some-Peripherals バインディング
│   │   ├── radar.rs
│   │   └── ...
│   ├── cc_vs/            # CC-VS (Valkyrien Skies) バインディング
│   │   └── ship.rs
│   └── control_craft/    # Control-Craft バインディング
│       ├── jet.rs
│       └── ...
└── Cargo.toml
```

### 5.2 feature フラグ

```toml
[features]
default = []
computer_craft   = []
some_peripherals = []
cc_vs            = []
control_craft    = []
```

特定の Mod がない環境でも、対応 feature を外すことでビルド・動作可能にします。

### 5.3 利用イメージ

```rust
// Cargo.toml
// rust_computers = { version = "x.x", features = ["computer_craft", "some_peripherals"] }

use rust_computers::computer_craft::monitor::Monitor;
use rust_computers::some_peripherals::radar::Radar;

#[entry]
async fn main() {
    // ペリフェラルハンドルを取得 (接続中の場合)
    let monitor: PeripheralHandle<Monitor> = find_peripheral("left").await;
    let radar: PeripheralHandle<Radar>     = find_peripheral("right").await;

    loop {
        // try_get() で現在も接続されているか確認
        if let Ok(m) = monitor.try_get() {
            m.write("Hello").await;
        }

        let scan_result = radar.try_get()?.scan(64.0).await?;
    }
}
```

---

## 6. `PeripheralHandle<T>`

### 6.1 設計

- Java 側: `HashMap<u64, IPeripheral>` で ID ↔ インスタンスを管理
- Rust 側: `PeripheralHandle<T> { id: u64 }` (軽量値型)
- `try_get()` は host_import 経由で Java に ID を渡し、接続確認 + メソッド呼び出しを行う

```rust
impl<T: Peripheral> PeripheralHandle<T> {
    /// 現在も接続されていれば T への参照相当を返す
    pub async fn try_get(&self) -> Result<T, PeripheralError> { ... }
}
```

### 6.2 接続切れの振る舞い

デタッチ済みの ID に対して呼んだ場合、`PeripheralError::Detached` が返ります。  
ユーザーは `?` 演算子または `match` で処理します。

---

## 7. CC 互換ペリフェラルシステムとの関係

| 要素 | 扱い |
|---|---|
| CCの Lua VM | **使用しない** (完全遮断) |
| CCの Java ペリフェラルシステム (`IPeripheral`, `@LuaFunction` 等) | **そのまま流用** |
| CCのモデム / ペリフェラルブロック等の Java 実装 | **一部流用** |
| Lua が絡む部分で重要なもの | 必要に応じて Rust で再実装 (FFI 経由) |

他 Mod の CC 対応は主に以下のパターンです（参照実装より）：

| パターン | 例 Mod | 概要 |
|---|---|---|
| `IPeripheral` + Forge Capability | Control-Craft (`JetPeripheral`) | BlockEntity に Capability として紐付け |
| `GenericPeripheral` | Some-Peripherals (`RadarPeripheral`) | `registerGenericSource()` で登録 |
| `ILuaAPI` | CC-VS (`ShipAPI`) | コンピューターに API を注入 |

RustComputers では、これら全パターンの `@LuaFunction` メソッドを **自動生成ツールで Rust 側バインディングに変換** することを目標とします。

---

## 8. ユーザープログラムライフサイクル

1. ユーザーが `rust_computers` クレートを依存に追加し、`async fn main()` を実装
2. Rust → WASM にコンパイル（ユーザー側ビルド）
3. WASM バイナリをゲーム内 UI (CC に近い形) でサーバーにアップロード
4. サーバーがサイズ制限内か確認し、コンピューターに紐付け
5. コンピューター起動で WASM インスタンス生成 → tick ループ開始

---

## 9. 未決定事項（決定済み部分を更新）

| # | 項目 | 状態 | 詳細 |
|---|---|---|---|
| W-1 | WASM ランタイムの選定 | **✅ 決定済み** | Wasmtime via wasmtime-java (MavenCentral) |
| W-2 | Java ↔ WASM ブリッジ | **✅ 決定済み** | 詳細は [docs/w2-java-wasm-bridge.md](./w2-java-wasm-bridge.md) |
| W-3 | `@LuaFunction` → Rust 自動生成ツール | ⏳ **未定** | 次フェーズ |
| W-4 | WASM バイナリのアップロード UI | ⏳ **未定** | 後期フェーズ |
| W-5 | WASM バイナリのサイズ上限値 | ⏳ **未定** | 後期フェーズ |

### W-2 決定内容

- **Shared Buffer**: 固定 64 KB （案①） 実
- **ホスト関数**: `host_request_info`, `host_poll_result`, `host_do_action`, `host_is_mod_available`
- **干渉パターン**: すべて Future ベース（`parallel!()` で一括待機）
- **エラーハンドリング**: -9 エラーコード体系
- **Timeout**: 100 ticks (≈5 秒)
- **Fuel Limit**: 10M / Tick （悪意あるプログラム対策）
- **Mod 検出**: Feature + runtime `host_is_mod_available()` で対応

---

## 10. 参照リポジトリ

| リポジトリ | 目的 |
|---|---|
| [CC-Tweaked](https://github.com/cc-tweaked/CC-Tweaked) | Java ペリフェラル API の調査 |
| [CC-VS](https://github.com/TechTastic/CC-VS) | `ILuaAPI` パターンの参照実装 |
| [Some-Peripherals](https://github.com/SuperSpaceEye/Some-Peripherals) | `IPeripheral` / `GenericPeripheral` パターンの参照実装 |
| [Control-Craft](https://github.com/Rew1nd-dev/Control-Craft) | `IPeripheral` + Capability パターンの参照実装 |
| [wasmtime-java (kawamuray)](https://github.com/kawamuray/wasmtime-java) | Wasmtime Java バインディング |
| [fabric-wasmcraft-mod](https://github.com/HashiCraft/fabric-wasmcraft-mod) | Minecraft WASM 統合パターン |

```
