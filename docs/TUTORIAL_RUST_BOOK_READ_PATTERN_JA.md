# Rust-book-read パターンチュートリアル

## 概要

RustComputersでは、Minecraftのゲームループ（1 tick = 0.05秒）と同期を取るために、**book-read パターン**を採用しています。このパターンは、Lua環境での「ループが進まない」動作を再現するために設計されました。

### なぜbook-readパターンが必要か

MinecraftのMod APIは1 tickごとに更新されます。RustからJavaのペリフェラルを呼び出す際、以下の制約があります：

- **1 tick内に複数のFFI呼び出しはできない** - パフォーマンス上の理由から、1 tick内で複数のForeign Function Interface呼び出しを行うと、最後の結果のみが保持されます
- **結果の取得は次のtickになる** - リクエストを発行しても、結果は次のtickにならないと取得できません
- **Luaのループ動作を再現** - CC:Tweakedでは、`while true do peripheral.call(...) end` でループしても1 tickに1回しか実行されません

book-readパターンは、これらの制約を巧みに扱うためのAPI設計です。

## 3つのメソッドペア

book-readパターンでは、各ペリフェラルメソッドに対して3つの形式を提供します：

| メソッド | 説明 | 用途 |
|---------|------|------|
| `book_next_*()` | リクエストを予約（非ブロッキング） | 次のtickで実行するリクエストを予約 |
| `read_last_*()` | 前tickの結果を読み取る | 予約したリクエストの結果を取得 |
| `async_*()` | 非同期メソッド（book + wait + read） | 自動的にbook→wait→readを実行 |

## メソッドの種類

### 1. 取得系メソッド（book_read!）

情報取得系のメソッドに使用します。**上書き動作**のため、最後に予約したリクエストのみが有効です。

```rust
book_read!(size, "size", (), u32);
book_read!(get_item, "getItem", GetItemArgs, ItemDetail);
```

**展開されるコード：**

```rust
pub fn book_next_size(&mut self) {
    crate::peripheral::book_request(self.addr, "size", &[]);
}

pub fn read_last_size(&self) -> Result<u32, crate::peripheral::PeripheralError> {
    let bytes = crate::peripheral::read_result(self.addr, "size")?;
    crate::peripheral::decode(&bytes)
}
```

### 2. 反映系メソッド（book_action!）

ワールド干渉系のメソッドに使用します。**蓄積動作**のため、予約したすべてのアクションが実行されます。

```rust
book_action!(push_item, "pushItem", PushItemArgs, ());
book_action!(set_signal, "setSignal", SetSignalArgs, ());
```

**展開されるコード：**

```rust
pub fn book_next_push_item(&mut self, args: PushItemArgs) {
    let bytes = crate::peripheral::encode(&args).unwrap_or_default();
    crate::peripheral::book_action(self.addr, "pushItem", &bytes);
}

pub fn read_last_push_item(&self) -> Vec<Result<(), crate::peripheral::PeripheralError>> {
    let results = crate::peripheral::read_action_results(self.addr, "pushItem");
    results.into_iter()
        .map(|r| r.and_then(|bytes| crate::peripheral::decode(&bytes)))
        .collect()
}
```

### 3. 非同期メソッド（async_method!）

`book_next_*` + `wait_for_next_tick().await` + `read_last_*()` を自動実行します。

```rust
async_method!(async_size, "size", (), u32);
async_method!(async_get_item, "getItem", GetItemArgs, ItemDetail);
```

**展開されるコード：**

```rust
pub async fn async_size(&mut self) -> Result<u32, crate::peripheral::PeripheralError> {
    self.book_next_size();
    crate::wait_for_next_tick().await;
    self.read_last_size()
}
```

### 4. 即時メソッド（imm_method!）

同一tick内で完結するメソッド用です。Java側で `@LuaFunction(immediate=true)` として実装されたメソッド専用です。

```rust
imm_method!(size_imm, "size", (), u32);
```

**展開されるコード：**

```rust
pub fn size_imm(&self) -> Result<u32, crate::peripheral::PeripheralError> {
    let bytes = crate::peripheral::request_info_imm(self.addr, "size", &[])?;
    crate::peripheral::decode(&result_bytes)
}
```

## 基本的な使用例

### 取得系メソッドの使用例

インベントリサイズを定期的に取得する例：

```rust
use rust_computers_api::prelude::*;

fn get_inventory_size_example() {
    let inventory = find_imm::<Inventory>().unwrap();
    
    // 初回リクエストを予約
    inventory.book_next_size();
    
    // 次のtickまで待つ
    block_on(async {
        wait_for_next_tick().await;
        
        loop {
            // 前tickの結果を読み取り
            let size = inventory.read_last_size().unwrap();
            println!("Inventory size: {}", size);
            
            // 次のtickのリクエストを予約
            inventory.book_next_size();
            wait_for_next_tick().await;
        }
    });
}
```

または、`async_*` メソッドを使用する場合：

```rust
fn get_inventory_size_async_example() {
    let inventory = find_imm::<Inventory>().unwrap();
    
    block_on(async {
        loop {
            // async_* で自動的にbook→wait→readを実行
            let size = inventory.async_size().await.unwrap();
            println!("Inventory size: {}", size);
        }
    });
}
```

### 反映系メソッドの使用例

アイテムを送信する例：

```rust
fn push_item_example() {
    let inventory = find_imm::<Inventory>().unwrap();
    
    block_on(async {
        loop {
            // 複数のアクションを予約
            inventory.book_next_push_item(PushItemArgs {
                to_slot: 1,
                count: 64,
                ..Default::default()
            });
            inventory.book_next_push_item(PushItemArgs {
                to_slot: 2,
                count: 32,
                ..Default::default()
            });
            
            wait_for_next_tick().await;
            
            // 全アクションの結果を取得
            let results = inventory.read_last_push_item();
            for (i, result) in results.into_iter().enumerate() {
                match result {
                    Ok(()) => println!("Push to slot {} succeeded", i + 1),
                    Err(e) => println!("Push to slot {} failed: {:?}", i + 1, e),
                }
            }
        }
    });
}
```

### イベント系メソッドの使用例

モデムのメッセージ受信：

```rust
fn receive_message_example() {
    let modem = find_imm::<Modem>().unwrap();
    
    block_on(async {
        loop {
            // イベント受信を予約
            modem.book_next_receive_raw();
            
            wait_for_next_tick().await;
            
            // 結果を確認（イベント未発生時はNone）
            if let Some(msg) = modem.read_last_receive_raw().unwrap() {
                println!("Received: {:?}", msg);
            }
        }
    });
}
```

## 実践的なパターン

### センサー監視ループ

```rust
fn sensor_monitoring_example() {
    let sensor = find_imm::<Sensor>().unwrap();
    
    block_on(async {
        // 初期リクエスト
        sensor.book_next_scan();
        wait_for_next_tick().await;
        
        loop {
            match sensor.read_last_scan() {
                Ok(Some(scan_result)) => {
                    // スキャン結果処理
                    process_scan_result(&scan_result);
                    
                    // 次のスキャン予約
                    sensor.book_next_scan();
                }
                Ok(None) => {
                    // スキャン結果なし（次のtickで再試行）
                    sensor.book_next_scan();
                }
                Err(e) => {
                    eprintln!("Scan failed: {:?}", e);
                    sensor.book_next_scan();
                }
            }
            
            wait_for_next_tick().await;
        }
    });
}
```

### 並行ペリフェラル操作

```rust
fn concurrent_peripherals_example() {
    let inventory = find_imm::<Inventory>().unwrap();
    let modem = find_imm::<Modem>().unwrap();
    let speaker = find_imm::<Speaker>().unwrap();
    
    block_on(async {
        loop {
            // 複数のペリフェラルに同時にリクエスト
            inventory.book_next_size();
            modem.book_next_receive_raw();
            speaker.book_next_play_note(Note::C4, 1.0);
            
            wait_for_next_tick().await;
            
            // 各結果を取得
            let size = inventory.read_last_size();
            let msg = modem.read_last_receive_raw();
            let _ = speaker.read_last_play_note();
            
            // 処理...
        }
    });
}
```

## .tick境界と同期

### tick境界の概念

```
GT N   [Rust]  → book_next_*() でリクエスト予約
GT N   [Rust]  → wait_for_next_tick().await で FFI 一括発行
GT N+1 [Java]  → リクエスト実行、結果バッファに保存
GT N+1 [Rust]  → read_last_*() で結果取得
```

### タイミングの重要性

```rust
// ❌ 間違い：book と read が同じ tick
inventory.book_next_size();
let size = inventory.read_last_size(); // None または古い結果

// ✅ 正しい：book → wait → read の順序
inventory.book_next_size();
wait_for_next_tick().await;
let size = inventory.read_last_size(); // 正しい結果
```

## エラーハンドリング

```rust
fn error_handling_example() {
    let peripheral = find_imm::<SomePeripheral>().unwrap();
    
    block_on(async {
        loop {
            peripheral.book_next_operation();
            wait_for_next_tick().await;
            
            match peripheral.read_last_operation() {
                Ok(result) => {
                    // 正常処理
                    process_result(result);
                }
                Err(PeripheralError::NotFound) => {
                    // ペリフェラルが見つからない
                    eprintln!("Peripheral not found, retrying...");
                }
                Err(PeripheralError::MethodNotFound) => {
                    // メソッドが存在しない
                    eprintln!("Method not available");
                    break;
                }
                Err(e) => {
                    // その他のエラー
                    eprintln!("Error: {:?}", e);
                }
            }
        }
    });
}
```

## ベストプラクティス

### 1. 適切なメソッドタイプの選択

- **取得系（情報取得）** → `book_read!` を使用
- **反映系（操作実行）** → `book_action!` を使用
- **イベント系** → `book_read!` を使用し、`Vec<Option<T>>` を返す

### 2. エラーハンドリングの徹底

```rust
// ❌ エラーを無視しない
let size = inventory.read_last_size().unwrap();

// ✅ エラーを適切に処理
let size = match inventory.read_last_size() {
    Ok(s) => s,
    Err(e) => {
        eprintln!("Failed to get size: {:?}", e);
        return;
    }
};
```

### 3. 適切なループ間隔

```rust
block_on(async {
    loop {
        // 何かしらの処理
        
        // 次のtickまで待つ（必須）
        wait_for_next_tick().await;
    }
});
```

### 4. キャンセル処理の考慮

```rust
use futures::future::Aborted;

block_on(async {
    let mut interval = interval(Duration::from_millis(100));
    
    loop {
        tokio::select! {
            _ = interval.tick() => {
                // 定期処理
                peripheral.book_next_get_data();
                wait_for_next_tick().await;
                let _ = peripheral.read_last_get_data();
            }
            _ = shutdown_signal() => {
                // シャットダウン処理
                break;
            }
        }
    }
});
```

## トラブルシューティング

### 結果が取得できない

```rust
// 原因1: wait_for_next_tick() を忘れている
// ❌
peripheral.book_next_size();
let size = peripheral.read_last_size(); // None

// ✅
peripheral.book_next_size();
wait_for_next_tick().await;
let size = peripheral.read_last_size(); // 正しい結果

// 原因2: ペリフェラルが接続されていない
let peripheral = match find_imm::<SomePeripheral>() {
    Some(p) => p,
    None => {
        eprintln!("Peripheral not found!");
        return;
    }
};
```

### 複数のリクエストが上書きされる

```rust
// 取得系メソッドは上書き動作
// ❌ 2回目のリクエストで1回目が上書きされる
peripheral.book_next_get_data();
peripheral.book_next_get_data(); // 上書きされる
wait_for_next_tick().await;
let result = peripheral.read_last_get_data(); // 2回目の結果のみ

// 反映系メソッドを使用するか、1つずつ処理
peripheral.book_next_action1();
wait_for_next_tick().await;
let _ = peripheral.read_last_action1();

peripheral.book_next_action2();
wait_for_next_tick().await;
let _ = peripheral.read_last_action2();
```

## 関連ドキュメント

- [Rust非同期パターンチュートリアル](TUTORIAL_RUST_ASYNC_PATTERN_JA.md)
- [Rustイベント処理チュートリアル](TUTORIAL_RUST_EVENT_HANDLING_JA.md)
- [Rust複数ペリフェラル並行操作チュートリアル](TUTORIAL_RUST_CONCURRENT_PERIPHERALS_JA.md)
- [APIリファレンス](../api_ja/)