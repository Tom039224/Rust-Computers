# Rust asyncパターンチュートリアル

## 概要

RustComputers APIは、CC:Tweakedとの互換性を保ちながら、Rustの非同期プログラミングの利点を活かすために、3つの関数パターンを採用しています。このチュートリルでは、以下のパターンについて説明します：

1. **book_next_* / read_last_* / async_* パターン**
2. **情報取得パターン**
3. **ワールド干渉パターン**
4. **イベント駆動プログラミング**
5. **エラーハンドリングとベストプラクティス**
6. **パフォーマンス考慮事項**

## 1. 3つの関数パターン

RustComputers APIでは、各ペリフェラルメソッドが以下の3つの形式で提供されます：

### 1.1 `book_next_*(&mut self, args) -> ()`
- **目的**: リクエストを予約する（FFI呼び出しなし）
- **動作**:
  - 情報取得系: 最後のリクエストのみ有効（上書き）
  - ワールド干渉系: 全て保存（追記）
- **特徴**: 非ブロッキング、即時返却

### 1.2 `read_last_*(&self) -> Result<T, PeripheralError>`
- **目的**: 前tickの結果を読み取る
- **動作**:
  - 情報取得系: 1つの結果（最新）を返す
  - ワールド干渉系: 複数の結果（全操作の結果）を返す `Vec<Result<T, PeripheralError>>`
- **特徴**: ブロッキング、結果取得

### 1.3 `async_*(&self, args) -> Result<T, PeripheralError>`
- **目的**: `.await`で結果を取得
- **動作**: 内部的に `book → wait → read` をループ
- **特徴**: 非同期、イベント駆動メソッドに対応

## 2. 情報取得パターン

情報取得系メソッドは、ワールドの状態を読み取るメソッドです。例：センサー値の取得、インベントリサイズの確認など。

### 2.1 基本パターン

```rust
use rust_computers_api::computer_craft::Inventory;
use rust_computers_api::peripheral::find_imm;

async fn get_inventory_size() -> Result<(), rust_computers_api::error::PeripheralError> {
    // ペリフェラルを検索
    let mut inventory = find_imm::<Inventory>().unwrap();
    
    // 方法1: book-read パターン（明示的制御）
    inventory.book_next_size();
    rust_computers_api::wait_for_next_tick().await;
    let size = inventory.read_last_size()?;
    println!("インベントリサイズ: {}", size);
    
    // 方法2: async パターン（簡潔）
    let size = inventory.async_size().await?;
    println!("インベントリサイズ: {}", size);
    
    Ok(())
}
```

### 2.2 ループ処理

```rust
async fn monitor_inventory() -> Result<(), PeripheralError> {
    let mut inventory = find_imm::<Inventory>().unwrap();
    
    // 初回予約
    inventory.book_next_list();
    rust_computers_api::wait_for_next_tick().await;
    
    loop {
        // 前tickの結果を読み取り
        let items = inventory.read_last_list()?;
        
        // 処理
        for (slot, item) in &items {
            println!("スロット {}: {} x{}", slot, item.name, item.count);
        }
        
        // 次tickのリクエストを予約
        inventory.book_next_list();
        rust_computers_api::wait_for_next_tick().await;
    }
}
```

## 3. ワールド干渉パターン

ワールド干渉系メソッドは、ワールドに変更を加えるメソッドです。例：アイテムの移動、ブロックの設置など。

### 3.1 単一操作

```rust
async fn move_single_item(
    from: &mut Inventory,
    to: &Inventory,
    slot: u32,
) -> Result<u32, PeripheralError> {
    // async パターン（推奨）
    let moved = from.async_push_items(to, slot, Some(64), None).await?;
    println!("移動したアイテム数: {}", moved);
    Ok(moved)
}
```

### 3.2 バッチ処理

```rust
async fn batch_operations() -> Result<(), PeripheralError> {
    let mut inventory = find_imm::<Inventory>().unwrap();
    let target = find_imm::<Inventory>().unwrap();
    
    // 複数操作を予約
    for slot in 1..=3 {
        inventory.book_next_push_items(&target, slot, Some(16), Some(slot));
    }
    
    // 1tickで全操作を実行
    rust_computers_api::wait_for_next_tick().await;
    
    // 全操作の結果を取得
    let results = inventory.read_last_push_items();
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(count) => println!("操作 {}: {}個移動", i+1, count),
            Err(e) => println!("操作 {}: 失敗 - {:?}", i+1, e),
        }
    }
    
    Ok(())
}
```

## 4. イベント駆動プログラミング

イベント系メソッドは、非同期にイベントを待機するメソッドです。

### 4.1 イベント待機

```rust
use rust_computers_api::computer_craft::Modem;

async fn wait_for_message() -> Result<(), PeripheralError> {
    let mut modem = find_imm::<Modem>().unwrap();
    
    // メッセージを待機
    let message = modem.async_receive_raw().await?;
    println!("受信メッセージ: {:?}", message);
    Ok(())
}
```

### 4.2 複数イベントの並行待機

```rust
use futures::future::select;
use futures::pin_mut;

async fn wait_for_any_event() -> Result<(), PeripheralError> {
    let mut modem = find_imm::<Modem>().unwrap();
    let mut monitor = find_imm::<Monitor>().unwrap();
    
    let modem_future = modem.async_receive_raw();
    let monitor_future = monitor.async_wait_for_touch();
    
    pin_mut!(modem_future);
    pin_mut!(monitor_future);
    
    match select(modem_future, monitor_future).await {
        Either::Left((message, _)) => {
            println!("メッセージ受信: {:?}", message);
        }
        Either::Right((touch, _)) => {
            println!("タッチイベント: {:?}", touch);
        }
    }
    Ok(())
}
```

## 5. Luaのコルーチンモデルとの比較

### 5.1 Lua (CC:Tweaked)

```lua
-- Lua (CC:Tweaked) の例
while true do
    local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
    print("受信: " .. message)
end
```

### 5.2 Rust (RustComputers)

```rust
// Rust (RustComputers) の同等の実装
async fn modem_listener() -> Result<(), PeripheralError> {
    let mut modem = find_imm::<Modem>().unwrap();
    
    loop {
        let message = modem.async_receive_raw().await?;
        println!("受信: {:?}", message);
    }
}
```

## 6. エラーハンドリング

### 6.1 基本エラーハンドリング

```rust
use rust_computers_api::error::PeripheralError;

async fn safe_operation() -> Result<(), PeripheralError> {
    let mut inventory = match find_imm::<Inventory>() {
        Some(inv) => inv,
        None => return Err(PeripheralError::NotFound),
    };
    
    // リトライロジック付き操作
    for attempt in 1..=3 {
        match inventory.async_size().await {
            Ok(size) => {
                println!("インベントリサイズ: {}", size);
                return Ok(());
            }
            Err(e) if attempt < 3 => {
                println!("試行 {} 失敗、再試行します...", attempt);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(PeripheralError::Timeout)
}
```

## 7. パフォーマンス最適化

### 7.1 バッチ処理

```rust
async fn batch_operations() -> Result<(), PeripheralError> {
    let mut inventory = find_imm::<Inventory>().unwrap();
    let target = find_imm::<Inventory>().unwrap();
    
    // 複数操作を一度に予約
    for slot in 1..=10 {
        inventory.book_next_push_items(&target, slot, Some(1), None);
    }
    
    // 1回のtickで全操作を実行
    wait_for_next_tick().await;
    
    // 結果を一括取得
    let results = inventory.read_last_push_items();
    Ok(())
}
```

### 7.2 キャッシュの活用

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Instant, Duration};

struct CachedInventory {
    inventory: Inventory,
    cache: Arc<RwLock<HashMap<String, (Instant, u32)>>>,
}

impl CachedInventory {
    async fn get_size_cached(&mut self) -> Result<u32, PeripheralError> {
        let cache_key = "size".to_string();
        
        // キャッシュチェック
        {
            let cache = self.cache.read().await;
            if let Some((timestamp, value)) = cache.get(&cache_key) {
                if timestamp.elapsed() < Duration::from_secs(5) {
                    return Ok(*value);
                }
            }
        }
        
        // キャッシュミス、実際に取得
        let size = self.inventory.async_size().await?;
        
        // キャッシュ更新
        let mut cache = self.cache.write().await;
        cache.insert(cache_key, (Instant::now(), size));
        
        Ok(size)
    }
}
```

## 8. 実践例: 自動在庫管理システム

```rust
struct InventoryManager {
    chest: Inventory,
    modem: Modem,
    low_stock_threshold: u32,
}

impl InventoryManager {
    async fn monitor_inventory(&mut self) -> Result<(), PeripheralError> {
        loop {
            // 在庫チェック
            let items = self.chest.async_list().await?;
            let total_items: u32 = items.values().map(|item| item.count).sum();
            
            if total_items < self.low_stock_threshold {
                // 在庫が少ない場合、警告を送信
                self.send_alert("在庫が少なくなっています").await?;
            }
            
            // 詳細チェック
            for (slot, item) in &items {
                if item.count < 10 {
                    println!("警告: スロット {} の {} が少なくなっています", 
                            slot, item.name);
                }
            }
            
            // 5秒待機
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
    
    async fn send_alert(&mut self, message: &str) -> Result<(), PeripheralError> {
        self.modem.async_transmit(1, message.to_string()).await
    }
}
```

## 9. デバッグとトラブルシューティング

### 9.1 デバッグログ

```rust
#[derive(Debug)]
struct DebugPeripheral {
    name: String,
    last_operation: Option<Instant>,
}

impl DebugPeripheral {
    async fn debug_operation(&mut self, operation: &str) {
        println!("[DEBUG] {}: 操作開始: {}", self.name, operation);
        let start = Instant::now();
        
        // 操作実行...
        
        let duration = start.elapsed();
        println!("[DEBUG] {}: 完了 ({}ms)", 
                self.name, duration.as_millis());
    }
}
```

### 9.2 タイムアウト処理

```rust
use tokio::time::{timeout, Duration};

async fn operation_with_timeout() -> Result<(), PeripheralError> {
    let mut modem = find_imm::<Modem>().unwrap();
    
    // 5秒のタイムアウトを設定
    match timeout(Duration::from_secs(5), modem.async_receive_raw()).await {
        Ok(Ok(message)) => {
            println!("メッセージ受信: {:?}", message);
            Ok(())
        }
        Ok(Err(e)) => Err(e),
        Err(_) => {
            println!("タイムアウト: メッセージ受信待機中");
            Err(PeripheralError::Timeout)
        }
    }
}
```

## 10. ベストプラクティス

### 10.1 リソース管理

```rust
struct ManagedPeripheral<T> {
    peripheral: T,
    // リソース管理用のフィールド
}

impl<T: Peripheral> ManagedPeripheral<T> {
    async fn with_peripheral<F, Fut, R>(&mut self, f: F) -> Result<R, PeripheralError>
    where
        F: FnOnce(&mut T) -> Fut,
        Fut: Future<Output = Result<R, PeripheralError>>,
    {
        // リソースの確保と解放を管理
        let result = f(&mut self.peripheral).await;
        // クリーンアップ処理
        result
    }
}
```

## まとめ

RustComputersのasyncパターンは、CC:Tweakedとの互換性を保ちつつ、Rustの強力な型システムと非同期プログラミングの利点を活かすことができます。このチュートリアルで紹介したパターンとベストプラクティスを活用して、堅牢で効率的なMinecraft Modを作成してください。