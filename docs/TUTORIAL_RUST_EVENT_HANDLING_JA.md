# Rustイベント処理チュートリアル

## はじめに

このチュートートリアルでは、RustComputersプロジェクトにおけるイベント処理の基本から応用までを解説します。CC:Tweakedとの互換性を保ちつつ、Rustの強力な型システムと非同期プログラミングを活用したイベント駆動プログラミングを学びます。

## 目次

1. [イベント処理の基本概念](#基本概念)
2. [3つの関数パターン](#3つの関数パターン)
3. [イベント処理パターン](#イベント処理パターン)
4. [実践例: モデム通信](#実践例-モデム通信)
5. [実践例: モニタータッチイベント](#実践例-モニタータッチイベント)
6. [エラーハンドリングとリトライ](#エラーハンドリングとリトライ)
7. [パフォーマンス最適化](#パフォーマンス最適化)
8. [ベストプラクティス](#ベストプラクティス)

## 基本概念

### 1.1 イベント駆動プログラミング

RustComputersでは、CC:Tweakedのイベントシステムと互換性を保ちつつ、Rustの強力な型システムを活用したイベント処理を提供します。

```rust
use rust_computers_api::computer_craft::{Modem, Monitor};
use rust_computers_api::peripheral::find_imm;

async fn event_loop() -> Result<(), rust_computers_api::error::PeripheralError> {
    // ペリフェラルの検索
    let mut modem = find_imm::<Modem>().pop().unwrap();
    let mut monitor = find_imm::<Monitor>().pop().unwrap();
    
    // イベントループ
    loop {
        // モデムメッセージの受信
        if let Ok(message) = modem.async_receive_raw().await {
            println!("受信メッセージ: {:?}", message);
        }
        
        // モニターのタッチイベント
        if let Ok(touch) = monitor.async_poll_touch().await {
            println!("タッチ位置: ({}, {})", touch.x, touch.y);
        }
    }
}
```

### 1.2 1ティック遅延の原則

RustComputersでは、すべてのペリフェラル操作に「1ティック遅延」の原則が適用されます：

```rust
async fn example() {
    let mut inventory = find_imm::<Inventory>().pop().unwrap();
    
    // ティックN: リクエスト予約
    inventory.book_next_size();
    
    // ティックN: リクエスト送信
    wait_for_next_tick().await;  // ティックN+1
    
    // ティックN+1: 結果取得
    let size = inventory.read_last_size()?;
    println!("インベントリサイズ: {}", size);
}
```

## 3つの関数パターン

RustComputersでは、各ペリフェラルメソッドが3つの形式で提供されます。

### 2.1 基本パターン

```rust
impl Modem {
    // 1. リクエスト予約（非ブロッキング）
    pub fn book_next_receive_raw(&mut self) {
        peripheral::book_request(self.addr, "receive_raw", &[]);
    }
    
    // 2. 結果読み取り
    pub fn read_last_receive_raw(&self) -> Result<Option<ReceiveData>, PeripheralError> {
        // 結果の読み取り
    }
    
    // 3. 非同期メソッド（推奨）
    pub async fn async_receive_raw(&self) -> Result<ReceiveData, PeripheralError> {
        loop {
            self.book_next_receive_raw();
            wait_for_next_tick().await;
            if let Some(data) = self.read_last_receive_raw()? {
                return Ok(data);
            }
        }
    }
}
```

### 2.2 使用例

```rust
// 方法1: 明示的なbook/readパターン
modem.book_next_receive_raw();
wait_for_next_tick().await;
if let Some(message) = modem.read_last_receive_raw()? {
    process_message(message);
}

// 方法2: 非同期メソッド（推奨）
let message = modem.async_receive_raw().await?;
process_message(message);
```

## イベント処理パターン

### 3.1 イベントループ

```rust
async fn event_loop() -> Result<(), PeripheralError> {
    let mut modem = find_imm::<Modem>().pop().unwrap();
    let mut monitor = find_imm::<Monitor>().pop().unwrap();
    
    loop {
        // 複数イベントの並行待機
        let (modem_msg, touch_event) = parallel!(
            modem.async_receive_raw(),
            monitor.async_poll_touch()
        ).await?;
        
        if let Ok(message) = modem_msg {
            handle_modem_message(message);
        }
        
        if let Ok(touch) = touch_event {
            handle_touch_event(touch);
        }
    }
}
```

### 3.2 イベントフィルタリング

```rust
async fn filtered_events() {
    let mut modem = find_imm::<Modem>().pop().unwrap();
    let mut channel_filter = ChannelFilter::new(1); // チャンネル1のみ受信
    
    loop {
        let message = modem.async_receive_raw().await?;
        
        // チャンネルフィルタリング
        if channel_filter.filter(&message) {
            process_message(message);
        }
    }
}
```

## 実践例: モデム通信

### 4.1 基本的な送受信

```rust
async fn chat_server() -> Result<(), PeripheralError> {
    let mut modem = find_imm::<Modem>().pop().unwrap();
    
    // チャンネルを開く
    modem.book_next_open(1);
    wait_for_next_tick().await;
    modem.read_last_open()?;
    
    // メッセージ送信
    modem.book_next_transmit_raw(1, 0, "Hello, World!");
    wait_for_next_tick().await;
    
    // メッセージ受信
    loop {
        match modem.async_receive_raw().await {
            Ok(message) => {
                println!("受信: {:?}", message);
                // エコーバック
                modem.book_next_transmit_raw(
                    message.reply_channel, 
                    message.channel, 
                    format!("Echo: {}", message.payload)
                );
            }
            Err(e) => {
                eprintln!("受信エラー: {:?}", e);
                break;
            }
        }
    }
}
```

### 4.2 マルチキャスト通信

```rust
struct ChatRoom {
    modem: Modem,
    channel: u32,
    participants: Vec<u32>,
}

impl ChatRoom {
    async fn broadcast(&mut self, message: &str) -> Result<(), PeripheralError> {
        for participant in &self.participants {
            self.modem.book_next_transmit_raw(
                *participant,
                0, // 返信チャンネルなし
                message
            );
        }
        wait_for_next_tick().await;
        Ok(())
    }
    
    async fn listen(&mut self) -> Result<(), PeripheralError> {
        loop {
            let message = self.modem.async_receive_raw().await?;
            println!("[{}] 受信: {:?}", 
                message.channel, message.payload);
        }
    }
}
```

## 実践例: モニタータッチイベント

### 5.1 タッチイベント処理

```rust
struct TouchInterface {
    monitor: Monitor,
    touch_handlers: Vec<Box<dyn Fn(TouchEvent)>>,
}

impl TouchInterface {
    async fn run(&mut self) -> Result<(), PeripheralError> {
        loop {
            match self.monitor.async_poll_touch().await {
                Ok(touch) => {
                    for handler in &self.touch_handlers {
                        handler(touch);
                    }
                }
                Err(e) => {
                    eprintln!("タッチイベントエラー: {:?}", e);
                    break;
                }
            }
        }
    }
    
    fn on_touch<F>(&mut self, handler: F) 
    where
        F: Fn(TouchEvent) + 'static,
    {
        self.touch_handlers.push(Box::new(handler));
    }
}
```

### 5.2 タッチベースUI

```rust
struct TouchButton {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    label: String,
    callback: Box<dyn Fn()>,
}

impl TouchButton {
    async fn handle_touch(&self, touch: TouchEvent) -> bool {
        let in_bounds = touch.x >= self.x && 
                       touch.x < self.x + self.width as i32 &&
                       touch.y >= self.y && 
                       touch.y < self.y + self.height as i32;
        
        if in_bounds {
            (self.callback)();
            true
        } else {
            false
        }
    }
}
```

## エラーハンドリングとリトライ

### 6.1 エラーハンドリング

```rust
async fn robust_event_loop() {
    let mut modem = find_imm::<Modem>().pop().unwrap();
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 3;
    
    loop {
        match modem.async_receive_raw().await {
            Ok(message) => {
                retry_count = 0; // 成功したらリトライカウントリセット
                process_message(message);
            }
            Err(e) if retry_count < MAX_RETRIES => {
                eprintln!("エラー (リトライ {}/{}): {:?}", 
                    retry_count + 1, MAX_RETRIES, e);
                retry_count += 1;
                wait_for_next_tick().await;
                continue;
            }
            Err(e) => {
                eprintln!("最大リトライ回数超過: {:?}", e);
                break;
            }
        }
    }
}
```

### 6.2 タイムアウト処理

```rust
use std::time::{Duration, Instant};

async fn receive_with_timeout(
    modem: &mut Modem, 
    timeout: Duration
) -> Result<ReceiveData, TimeoutError> {
    let start = Instant::now();
    
    while start.elapsed() < timeout {
        match modem.async_receive_raw().await {
            Ok(message) => return Ok(message),
            Err(_) => {
                // 短い待機後に再試行
                wait_for_next_tick().await;
                continue;
            }
        }
    }
    Err(TimeoutError)
}
```

## パフォーマンス最適化

### 7.1 バッチ処理

```rust
async fn batch_operations() {
    let mut inventory = find_imm::<Inventory>().pop().unwrap();
    let target = find_imm::<Inventory>().nth(1).unwrap();
    
    // 複数操作を1ティックで実行
    for slot in 1..=10 {
        inventory.book_next_push_items(&target, slot, Some(16), None);
    }
    
    // 1ティックで全操作を実行
    wait_for_next_tick().await;
    
    // 全結果を一括取得
    let results = inventory.read_last_push_items();
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(count) => println!("スロット {}: {}個移動", i+1, count),
            Err(e) => eprintln!("エラー: {:?}", e),
        }
    }
}
```

### 7.2 キャッシュの活用

```rust
struct CachedInventory {
    inventory: Inventory,
    size_cache: Option<(Instant, u32)>,
    cache_duration: Duration,
}

impl CachedInventory {
    async fn get_size_cached(&mut self) -> Result<u32, PeripheralError> {
        let now = Instant::now();
        
        if let Some((cached_time, size)) = self.size_cache {
            if now.duration_since(cached_time) < self.cache_duration {
                return Ok(size);
            }
        }
        
        // キャッシュミス: 実際に取得
        let size = self.inventory.async_size().await?;
        self.size_cache = Some((now, size));
        Ok(size)
    }
}
```

## ベストプラクティス

### 8.1 リソース管理

```rust
struct ManagedPeripheral<T: Peripheral> {
    peripheral: T,
    // リソース管理用の追加フィールド
}

impl<T: Peripheral> ManagedPeripheral<T> {
    async fn with_peripheral<F, R, Fut>(&mut self, f: F) -> Result<R, PeripheralError>
    where
        F: FnOnce(&mut T) -> Fut,
        Fut: Future<Output = Result<R, PeripheralError>>,
    {
        // リソースの確保
        let result = f(&mut self.peripheral).await;
        
        // クリーンアップ
        if result.is_err() {
            self.cleanup().await;
        }
        
        result
    }
}
```

### 8.2 エラーハンドリングの統一

```rust
trait EventHandler {
    async fn handle_event(&mut self, event: Event) -> Result<(), Error>;
}

struct EventProcessor {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventProcessor {
    async fn process_event(&mut self, event: Event) {
        for handler in &mut self.handlers {
            if let Err(e) = handler.handle_event(event.clone()).await {
                eprintln!("ハンドラエラー: {:?}", e);
                // エラー処理
            }
        }
    }
}
```

## 高度なパターン

### 9.1 イベントバスパターン

```rust
struct EventBus {
    subscribers: HashMap<EventType, Vec<Box<dyn EventHandler>>>,
}

impl EventBus {
    async fn publish(&self, event: Event) {
        if let Some(handlers) = self.subscribers.get(&event.event_type) {
            for handler in handlers {
                if let Err(e) = handler.handle(&event).await {
                    eprintln!("ハンドラエラー: {:?}", e);
                }
            }
        }
    }
}
```

### 9.2 ステートマシン

```rust
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(Error),
}

struct Connection {
    state: ConnectionState,
    modem: Modem,
}

impl Connection {
    async fn connect(&mut self) -> Result<(), Error> {
        match self.state {
            ConnectionState::Disconnected => {
                self.state = ConnectionState::Connecting;
                // 接続処理
                self.state = ConnectionState::Connected;
                Ok(())
            }
            _ => Err(Error::InvalidState),
        }
    }
}
```

## まとめ

RustComputersのイベント処理システムは、CC:Tweakedとの互換性を保ちつつ、Rustの型安全性と非同期プログラミングの利点を活かした設計になっています。3つの関数パターン（book/read/async）を適切に使い分けることで、効率的で堅牢なイベント駆動アプリケーションを構築できます。

主なポイント：
1. **1ティック遅延**を理解し、適切に待機する
2. **3つの関数パターン**を適切に使い分ける
3. **エラーハンドリング**を適切に行い、リトライメカニズムを実装する
4. **パフォーマンス**を考慮し、バッチ処理やキャッシュを活用する
5. **型安全性**を活かし、コンパイル時に多くのエラーを検出する

このチュートリアルで紹介したパターンとベストプラクティスを活用して、堅牢で効率的なRustComputersアプリケーションを開発してください。

## 付録A: 実践的なコード例

### A.1 チャットクライアント

```rust
use rust_computers_api::computer_craft::{Modem, Monitor};
use rust_computers_api::peripheral::find_imm;
use rust_computers_api::wait_for_next_tick;

struct ChatClient {
    modem: Modem,
    monitor: Monitor,
    username: String,
    channel: u32,
}

impl ChatClient {
    async fn new() -> Result<Self, PeripheralError> {
        let modem = find_imm::<Modem>().pop().ok_or(PeripheralError::NotFound)?;
        let monitor = find_imm::<Monitor>().pop().ok_or(PeripheralError::NotFound)?;
        
        Ok(Self {
            modem,
            monitor,
            username: "Player".to_string(),
            channel: 1,
        })
    }
    
    async fn connect(&mut self) -> Result<(), PeripheralError> {
        // チャンネルを開く
        self.modem.book_next_open(self.channel);
        wait_for_next_tick().await;
        self.modem.read_last_open()?;
        
        println!("チャンネル {} に接続しました", self.channel);
        Ok(())
    }
    
    async fn send_message(&mut self, message: &str) -> Result<(), PeripheralError> {
        let full_message = format!("[{}] {}", self.username, message);
        self.modem.book_next_transmit_raw(self.channel, 0, &full_message);
        wait_for_next_tick().await;
        self.modem.read_last_transmit_raw()?;
        Ok(())
    }
    
    async fn receive_messages(&mut self) -> Result<(), PeripheralError> {
        loop {
            match self.modem.async_receive_raw().await {
                Ok(message) => {
                    println!("受信: {}", message.payload);
                }
                Err(e) => {
                    eprintln!("受信エラー: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }
}
```

### A.2 タッチベースのメニューシステム

```rust
struct MenuItem {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    label: String,
    action: Box<dyn Fn()>,
}

struct TouchMenu {
    monitor: Monitor,
    items: Vec<MenuItem>,
}

impl TouchMenu {
    async fn run(&mut self) -> Result<(), PeripheralError> {
        // メニューを描画
        self.draw_menu().await?;
        
        // タッチイベントを待機
        loop {
            let touch = self.monitor.async_poll_touch().await?;
            
            for item in &self.items {
                if self.is_touch_in_item(&touch, item) {
                    (item.action)();
                    self.draw_menu().await?; // メニューを再描画
                    break;
                }
            }
        }
    }
    
    fn is_touch_in_item(&self, touch: &TouchEvent, item: &MenuItem) -> bool {
        touch.x >= item.x && 
        touch.x < item.x + item.width as i32 &&
        touch.y >= item.y && 
        touch.y < item.y + item.height as i32
    }
    
    async fn draw_menu(&mut self) -> Result<(), PeripheralError> {
        // 画面をクリア
        self.monitor.book_next_clear();
        wait_for_next_tick().await;
        
        // 各アイテムを描画
        for (i, item) in self.items.iter().enumerate() {
            self.monitor.book_next_set_cursor_pos(
                MonitorPosition { x: item.x as u32, y: item.y as u32 }
            );
            self.monitor.book_next_write(&format!("[{}] {}", i + 1, item.label));
        }
        
        wait_for_next_tick().await;
        Ok(())
    }
}
```

## 付録B: デバッグとトラブルシューティング

### B.1 デバッグログ

```rust
#[derive(Debug)]
struct DebugPeripheral<T: Peripheral> {
    peripheral: T,
    name: String,
    last_operation: Option<Instant>,
}

impl<T: Peripheral> DebugPeripheral<T> {
    async fn debug_operation<F, R, Fut>(&mut self, operation: &str, f: F) -> Result<R, PeripheralError>
    where
        F: FnOnce(&mut T) -> Fut,
        Fut: Future<Output = Result<R, PeripheralError>>,
    {
        println!("[DEBUG] {}: 操作開始: {}", self.name, operation);
        let start = Instant::now();
        
        let result = f(&mut self.peripheral).await;
        
        let duration = start.elapsed();
        match &result {
            Ok(_) => println!("[DEBUG] {}: 成功 ({}ms)", self.name, duration.as_millis()),
            Err(e) => println!("[DEBUG] {}: 失敗 - {:?} ({}ms)", 
                self.name, e, duration.as_millis()),
        }
        
        result
    }
}
```

### B.2 パフォーマンスモニタリング

```rust
struct PerformanceMonitor {
    operations: Vec<(String, Duration)>,
    start_time: Instant,
}

impl PerformanceMonitor {
    fn new() -> Self {
        Self {
            operations: Vec::new(),
            start_time: Instant::now(),
        }
    }
    
    fn record_operation(&mut self, name: &str, duration: Duration) {
        self.operations.push((name.to_string(), duration));
    }
    
    fn print_summary(&self) {
        println!("=== パフォーマンスサマリー ===");
        println!("総実行時間: {}ms", self.start_time.elapsed().as_millis());
        
        for (name, duration) in &self.operations {
            println!("{}: {}ms", name, duration.as_millis());
        }
    }
}
```

## 付録C: テストと検証

### C.1 単体テスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_touch_detection() {
        let menu = TouchMenu {
            monitor: Monitor::new(PeriphAddr::from_raw(0)),
            items: vec![
                MenuItem {
                    x: 1,
                    y: 1,
                    width: 10,
                    height: 1,
                    label: "Test".to_string(),
                    action: Box::new(|| {}),
                }
            ],
        };
        
        let touch = TouchEvent { x: 5, y: 1 };
        assert!(menu.is_touch_in_item(&touch, &menu.items[0]));
        
        let touch_outside = TouchEvent { x: 15, y: 1 };
        assert!(!menu.is_touch_in_item(&touch_outside, &menu.items[0]));
    }
}
```

### C.2 統合テスト

```rust
async fn integration_test() -> Result<(), PeripheralError> {
    // テスト環境のセットアップ
    let mut modem = find_imm::<Modem>().pop().unwrap();
    let mut test_modem = find_imm::<Modem>().nth(1).unwrap();
    
    // チャンネルを開く
    modem.book_next_open(1);
    test_modem.book_next_open(1);
    wait_for_next_tick().await;
    
    // メッセージ送信
    test_modem.book_next_transmit_raw(1, 0, "Test Message");
    wait_for_next_tick().await;
    
    // メッセージ受信
    let message = modem.async_receive_raw().await?;
    assert_eq!(message.payload, "Test Message");
    
    println!("統合テスト成功");
    Ok(())
}
```

## 付録D: リファレンス

### D.1 主要なイベントタイプ

| イベントタイプ | ペリフェラル | 説明 |
|--------------|------------|------|
| `modem_message` | Modem | モデムメッセージ受信 |
| `monitor_touch` | Monitor | モニタータッチ |
| `inventory_changed` | Inventory | インベントリ変更 |
| `redstone_changed` | Redstone | レッドストーン信号変更 |

### D.2 エラーコード

| エラーコード | 説明 | 推奨アクション |
|------------|------|---------------|
| `NotFound` | ペリフェラルが見つからない | ペリフェラルの再検索 |
| `MethodNotFound` | メソッドが見つから���い | APIバージョンの確認 |
| `ExecutionFailed` | 実行失敗 | ログ出力と再試行 |
| `Disconnected` | 接続切断 | ペリフェラルの再接続 |
| `Timeout` | タイムアウト | リクエストの再発行 |

## 結論

RustComputersのイベント処理システムは、CC:Tweakedとの互換性を保ちつつ、Rustの現代的なプログラミングパラダイムを提供します。このチュートリアルで学んだパターンとテクニックを活用して、堅牢で効率的なMinecraft Modを作成してください。

重要なポイントを再確認します：

1. **1ティック遅延**を理解し、適切に待機する
2. **3つの関数パターン**を適切に使い分ける
3. **エラーハンドリング**を適切に行い、リトライメカニズムを実装する
4. **パフォーマンス**を考慮し、バッチ処理やキャッシュを活用する
5. **型安全性**を活かし、コンパイル時に多くのエラーを検出する

さらに詳しい情報や最新のAPIについては、RustComputersの公式ドキュメントを参照してください。