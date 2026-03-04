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
| **統合方式** | C API + JNI ブリッジ（自作） |
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

### 4.1 基本構造

```
[Java ServerTickEvent]
    ↓
wasm_instance.call("poll")   ← 各コンピューターを poll
    ↓
Rust Future::poll()
    ↓ host_import 呼び出し
java_host_request_info(peripheral_id, method_id, args...)
java_host_write(peripheral_id, method_id, args...)
```

### 4.2 バインディング自動生成

CC 対応 Mod の `@LuaFunction` アノテーションから **Rust バインディングを自動生成** することを目標とします。

```
[Java ソース (@LuaFunction アノテーション付き)]
    ↓ ビルド時ツール (Gradle タスク / Annotation Processor)
[生成された Rust ソース (host_import ラッパー)]
    ↓
rust_computers クレートの feature として提供
```

手書きによるフォールバックも許容しますが、自動生成を優先します。

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

## 9. 未決定事項

| # | 項目 | 状態 |
|---|---|---|
| W-1 | WASM ランタイムの選定 (GraalVM / Wasmtime / Wasmer) | **未定** |
| W-2 | Java ↔ WASM ブリッジの具体的な実装方式 | **未定** |
| W-3 | `@LuaFunction` → Rust 自動生成ツールの実装方式 | **未定** |
| W-4 | WASM バイナリのアップロード UI の詳細 | **未定** |
| W-5 | WASM バイナリのサイズ上限値 | **未定** |

---

## 10. 参照リポジトリ

| リポジトリ | 目的 |
|---|---|
| [CC-Tweaked](https://github.com/cc-tweaked/CC-Tweaked) | Java ペリフェラル API の調査 |
| [CC-VS](https://github.com/TechTastic/CC-VS) | `ILuaAPI` パターンの参照実装 |
| [Some-Peripherals](https://github.com/SuperSpaceEye/Some-Peripherals) | `IPeripheral` / `GenericPeripheral` パターンの参照実装 |
| [Control-Craft](https://github.com/Rew1nd-dev/Control-Craft) | `IPeripheral` + Capability パターンの参照実装 |
