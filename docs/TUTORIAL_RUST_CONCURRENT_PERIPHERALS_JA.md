# Rust 複数ペリフェラル並行操作チュートリアル

## 概要

このチュートリアルでは、RustComputers APIを使用して複数のペリフェラルを並行操作する方法について説明します。CC:Tweakedとの互換性を保ちつつ、Rustの強力な非同期プログラミング機能を活用して、効率的で堅牢な並行処理システムを構築する方法を学びます。

## 目次

1. [並行操作の基本概念](#基本概念)
2. [複数ペリフェラルの同時操作](#複数ペリフェラルの同時操作)
3. [イベント処理と並行処理](#イベント処理と並行処理)
4. [エラーハンドリングとリトライ](#エラーハンドリングとリトライ)
5. [実践例: 自動化システム](#実践例-自動化システム)
6. [パフォーマンス最適化](#パフォーマンス最適化)
7. [ベストプラクティス](#ベストプラクティス)

## 基本概念

### 1.1 並行操作の重要性

Minecraftのワールドでは、複数のペリフェラルを同時に操作することで、より効率的な自動化システムを構築できます。例：
- インベントリ管理と通信の同時実行
- センサー監視とアクチュエータ制御の並行処理
- 複数デバイスからのイベント同時監視

### 1.2 3つの関数パターンと並行性

RustComputers APIの3つの関数パターンは、並行操作に最適化されています：

```rust
// 情報取得系: 複数ペリフェラルからの同時取得
async fn gather_data_concurrently() -> Result<(), PeripheralError> {
    let mut sensor1 = find_imm::<Sensor>().unwrap();
    let mut sensor2 = find_imm::<Sensor>().nth(1).unwrap();
    
    // 同時に予約
    sensor1.book_next_get_data();
    sensor2.book_next_get_data();
    
    // 1tickで両方のリクエストを実行
    wait_for_next_tick().await;
    
    // 同時に結果を取得
    let data1 = sensor1.read_last_get_data()?;
    let data2 = sensor2.read_last_get_data()?;
    
    Ok(())
}
```

## 複数ペリフェラルの同時操作

### 2.1 基本的な並行操作パターン

```rust
use rust_computers_api::computer_craft::{Inventory, Modem, Monitor};
use rust_computers_api::peripheral::find_imm;
use futures::future::join_all;

async fn concurrent_operations() -> Result<(), PeripheralError> {
    // 複数ペリフェラルの取得
    let mut inventory = find_imm::<Inventory>().unwrap();
    let mut modem = find_imm::<Modem>().unwrap();
    let mut monitor = find_imm::<Monitor>().unwrap();
    
    // 方法1: 順次実行（非効率）
    let size = inventory.async_size().await?;
    let message = modem.async_receive_raw().await?;
    let touch = monitor.async_poll_touch().await?;
    
    // 方法2: 並行実行（推奨）
    let (size_future, message_future, touch_future) = (
        inventory.async_size(),
        modem.async_receive_raw(),
        monitor.async_poll_touch(),
    );
    
    let (size, message, touch) = join_all(vec![
        size_future,
        message_future,
        touch_future,
    ]).await?;
    
    Ok(())
}
```

### 2.2 バッチ処理による最適化

```rust
async fn batch_concurrent_operations() -> Result<(), PeripheralError> {
    // 複数のインベントリを操作
    let inventories: Vec<Inventory> = find_imm::<Inventory>().collect();
    let target = find_imm::<Inventory>().nth(inventories.len()).unwrap();
    
    // 全インベントリに対して同時に操作を予約
    for (i, inv) in inventories.iter_mut().enumerate() {
        inv.book_next_push_items(&target, i as u32 + 1, Some(16), None);
    }
    
    // 1tickで全操作を実行
    wait_for_next_tick().await;
    
    // 全結果を収集
    let mut results = Vec::new();
    for inv in &inventories {
        let result = inv.read_last_push_items();
        results.push(result);
    }
    
    Ok(())
}
```

### 2.3 異なるタイプのペリフェラルを同時操作

```rust
struct MultiPeripheralSystem {
    inventory: Inventory,
    modem: Modem,
    monitor: Monitor,
    sensor: Sensor,
}

impl MultiPeripheralSystem {
    async fn concurrent_data_gathering(&mut self) -> Result<MultiData, PeripheralError> {
        // 各ペリフェラルから同時にデータを取得
        let inventory_future = self.inventory.async_list();
        let modem_future = self.modem.async_receive_raw();
        let monitor_future = self.monitor.async_poll_touch();
        let sensor_future = self.sensor.async_get_data();
        
        let (inventory_data, modem_data, monitor_data, sensor_data) = join_all(vec![
            inventory_future,
            modem_future,
            monitor_future,
            sensor_future,
        ]).await?;
        
        Ok(MultiData {
            inventory: inventory_data,
            modem: modem_data,
            monitor: monitor_data,
            sensor: sensor_data,
        })
    }
}
```

## イベント処理と並行処理

### 3.1 複数イベントソースの同時監視

```rust
use futures::future::select;
use futures::pin_mut;

async fn monitor_multiple_events() -> Result<(), PeripheralError> {
    let mut modem = find_imm::<Modem>().unwrap();
    let mut monitor = find_imm::<Monitor>().unwrap();
    let mut inventory = find_imm::<Inventory>().unwrap();
    
    loop {
        // 複数イベントを同時に待機
        let modem_event = modem.async_receive_raw();
        let monitor_event = monitor.async_poll_touch();
        let inventory_event = inventory.async_wait_for_change();
        
        pin_mut!(modem_event);
        pin_mut!(monitor_event);
        pin_mut!(inventory_event);
        
        match select(modem_event, select(monitor_event, inventory_event)).await {
            Either::Left((message, _)) => {
                println!("モデムメッセージ: {:?}", message);
            }
            Either::Right((Either::Left((touch, _)), _)) => {
                println!("モニタータッチ: {:?}", touch);
            }
            Either::Right((Either::Right((change, _)), _)) => {
                println!("インベントリ変更: {:?}", change);
            }
        }
    }
}
```

### 3.2 イベント駆動の並行処理

```rust
struct EventDrivenSystem {
    event_handlers: Vec<Box<dyn EventHandler>>,
}

impl EventDrivenSystem {
    async fn run_concurrent_event_loop(&mut self) -> Result<(), PeripheralError> {
        let mut event_futures = Vec::new();
        
        // 各イベントハンドラのfutureを作成
        for handler in &mut self.event_handlers {
            event_futures.push(handler.wait_for_event());
        }
        
        // イベントを並行待機
        loop {
            let (result, index, _) = select_all(event_futures).await;
            
            match result {
                Ok(event) => {
                    // イベントを処理
                    self.event_handlers[index].handle_event(event).await?;
                    
                    // 同じハンドラの次のイベントを待機
                    event_futures[index] = self.event_handlers[index].wait_for_event();
                }
                Err(e) => {
                    eprintln!("イベントハンドラ {} エラー: {:?}", index, e);
                    // エラーハンドリング
                }
            }
        }
    }
}
```

## エラーハンドリングとリトライ

### 4.1 並行操作でのエラーハンドリング

```rust
async fn robust_concurrent_operations() -> Result<(), PeripheralError> {
    let mut peripherals = vec![
        find_imm::<Inventory>().unwrap(),
        find_imm::<Modem>().unwrap(),
        find_imm::<Monitor>().unwrap(),
    ];
    
    // 各ペリフェラルに対してリトライ付き操作
    let mut futures = Vec::new();
    
    for peripheral in &mut peripherals {
        let future = retry_operation(peripheral, 3); // 最大3回リトライ
        futures.push(future);
    }
    
    // 並行実行（一部失敗しても継続）
    let results = join_all(futures).await;
    
    // 結果の分析
    let mut success_count = 0;
    let mut error_count = 0;
    
    for result in results {
        match result {
            Ok(_) => success_count += 1,
            Err(e) => {
                error_count += 1;
                eprintln!("操作失敗: {:?}", e);
            }
        }
    }
    
    println!("成功: {}, 失敗: {}", success_count, error_count);
    Ok(())
}

async fn retry_operation<T: Peripheral>(
    peripheral: &mut T,
    max_retries: u32,
) -> Result<(), PeripheralError> {
    for attempt in 1..=max_retries {
        match peripheral.async_operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_retries => {
                eprintln!("試行 {} 失敗、再試行します...", attempt);
                wait_for_next_tick().await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(PeripheralError::Timeout)
}
```

### 4.2 部分的な失敗への対応

```rust
struct PartialSuccessSystem {
    peripherals: Vec<Box<dyn Peripheral>>,
}

impl PartialSuccessSystem {
    async fn execute_with_partial_success(&mut self) -> Result<Vec<Result<(), PeripheralError>>, PeripheralError> {
        let mut futures = Vec::new();
        
        // 各ペリフェラルに対して操作を実行
        for peripheral in &mut self.peripherals {
            let future = peripheral.async_operation();
            futures.push(future);
        }
        
        // 全ての操作を並行実行
        let results = join_all(futures).await;
        
        // 部分的な成功を許容
        let mut partial_results = Vec::new();
        let mut has_critical_error = false;
        
        for result in results {
            match result {
                Ok(_) => partial_results.push(Ok(())),
                Err(e) if e.is_critical() => {
                    has_critical_error = true;
                    partial_results.push(Err(e));
                }
                Err(e) => {
                    // 非致命的なエラーは記録のみ
                    eprintln!("非致命的エラー: {:?}", e);
                    partial_results.push(Err(e));
                }
            }
        }
        
        if has_critical_error {
            Err(PeripheralError::PartialFailure(partial_results))
        } else {
            Ok(partial_results)
        }
    }
}
```

## 実践例: 自動化システム

### 5.1 自動在庫管理システム

```rust
struct AutomatedInventorySystem {
    source_chests: Vec<Inventory>,
    sorting_chest: Inventory,
    modem: Modem,
    monitor: Monitor,
}

impl AutomatedInventorySystem {
    async fn run_concurrent_sorting(&mut self) -> Result<(), PeripheralError> {
        loop {
            // 1. 全チェストの在庫を同時にチェック
            let inventory_futures: Vec<_> = self.source_chests
                .iter_mut()
                .map(|chest| chest.async_list())
                .collect();
            
            let all_items = join_all(inventory_futures).await?;
            
            // 2. 並行でアイテム移動を計画
            let mut move_futures = Vec::new();
            
            for (chest_index, items) in all_items.into_iter().enumerate() {
                for (slot, item) in items {
                    if self.needs_sorting(&item) {
                        let future = self.sort_item(chest_index, slot, item);
                        move_futures.push(future);
                    }
                }
            }
            
            // 3. 並行でアイテム移動を実行
            let move_results = join_all(move_futures).await;
            
            // 4. 結果を分析してログ出力
            self.analyze_results(move_results).await?;
            
            // 5. 次のチェックまで待機
            wait_for_next_tick().await;
        }
    }
    
    async fn sort_item(&mut self, chest_index: usize, slot: u32, item: Item) 
        -> Result<u32, PeripheralError> 
    {
        let chest = &mut self.source_chests[chest_index];
        chest.async_push_items(&self.sorting_chest, slot, Some(item.count), None).await
    }
}
```

### 5.2 監視・制御システム

```rust
struct MonitoringControlSystem {
    sensors: Vec<Sensor>,
    actuators: Vec<Actuator>,
    modem: Modem,
    database: Database,
}

impl MonitoringControlSystem {
    async fn concurrent_monitoring_loop(&mut self) -> Result<(), PeripheralError> {
        // センサーデータ収集タスク
        let sensor_task = self.collect_sensor_data();
        
        // イベント監視タスク
        let event_task = self.monitor_events();
        
        // 制御タスク
        let control_task = self.control_actuators();
        
        // ログ記録タスク
        let logging_task = self.log_to_database();
        
        // 全タスクを並行実行
        let (sensor_result, event_result, control_result, logging_result) = 
            join_all(vec![sensor_task, event_task, control_task, logging_task]).await?;
        
        Ok(())
    }
    
    async fn collect_sensor_data(&mut self) -> Result<Vec<SensorData>, PeripheralError> {
        let futures: Vec<_> = self.sensors
            .iter_mut()
            .map(|sensor| sensor.async_get_data())
            .collect();
        
        join_all(futures).await
    }
    
    async fn control_actuators(&mut self) -> Result<(), PeripheralError> {
        // センサーデータに基づいてアクチュエータを制御
        let sensor_data = self.collect_sensor_data().await?;
        
        let control_futures: Vec<_> = self.actuators
            .iter_mut()
            .enumerate()
            .map(|(i, actuator)| {
                let target_value = self.calculate_target_value(&sensor_data, i);
                actuator.async_set_value(target_value)
            })
            .collect();
        
        join_all(control_futures).await?;
        Ok(())
    }
}
```

## パフォーマンス最適化

### 6.1 バッチ処理と並行処理の組み合わせ

```rust
async fn optimized_concurrent_operations() -> Result<(), PeripheralError> {
    let peripherals: Vec<Inventory> = find_imm::<Inventory>().collect();
    
    // バッチサイズを制限（メモリ使用量とパフォーマンスのバランス）
    const BATCH_SIZE: usize = 5;
    
    for batch in peripherals.chunks(BATCH_SIZE) {
        let mut futures = Vec::new();
        
        // バッチ内の各ペリフェラルに対して操作
        for peripheral in batch {
            let future = peripheral.async_operation();
            futures.push(future);
        }
        
        // バッチを並行実行
        let results = join_all(futures).await;
        
        // 結果処理
        self.process_batch_results(results).await?;
        
        // 次のバッチまで少し待機（システム負荷軽減）
        wait_for_next_tick().await;
    }
    
    Ok(())
}
```

### 6.2 リソースプーリング

```rust
struct PeripheralPool {
    pool: Vec<Inventory>,
    semaphore: Arc<Semaphore>,
}

impl PeripheralPool {
    async fn with_peripheral<F, Fut, R>(&self, f: F) -> Result<R, PeripheralError>
    where
        F: FnOnce(&mut Inventory) -> Fut,
        Fut: Future<Output = Result<R, PeripheralError>>,
    {
        // セマフォで同時実行数を制限
        let permit = self.semaphore.acquire().await?;
        
        // プールからペリフェラルを取得
        let peripheral = self.get_available_peripheral().await?;
        
        // 操作実行
        let result = f(peripheral).await;
        
        // ペリフェラルをプールに返却
        self.return_peripheral(peripheral).await;
        
        // 許可証を解放
        drop(permit);
        
        result
    }
    
    async fn concurrent_operations(&self, operations: usize) -> Result<Vec<Result<(), PeripheralError>>, PeripheralError> {
        let mut futures = Vec::new();
        
        for _ in 0..operations {
            let future = self.with_peripheral(|peripheral| peripheral.async_operation());
            futures.push(future);
        }
        
        join_all(futures).await
    }
}
```

## ベストプラクティス

### 7.1 並行処理の設計原則

1. **タスクの分離**: 関連する操作をまとめて、独立したタスクとして実行
2. **リソース管理**: 同時実行数を制限してシステム負荷を管理
3. **エラー分離**: 一つのタスクの失敗が他のタスクに影響しないように設計
4. **優先度設定**: 重要な操作に高い優先度を設定

### 7.2 デバッグとモニタリング

```rust
struct ConcurrentSystemMonitor {
    operation_times: Arc<Mutex<HashMap<String, Vec<Duration>>>>,
    error_counts: Arc<Mutex<HashMap<String, u32>>>,
}

impl ConcurrentSystemMonitor {
    async fn monitored_operation<F, Fut, R>(
        &self,
        operation_name: &str,
        f: F,
    ) -> Result<R, PeripheralError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<R, PeripheralError>>,
    {
        let start = Instant::now();
        
        let result = f().await;
        
        let duration = start.elapsed();
        
        // 実行時間を記録
        {
            let mut times = self.operation_times.lock().await;
            times.entry(operation_name.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
        }
        
        // エラーを記録
        if result.is_err() {
            let mut errors = self.error_counts.lock().await;
            *errors.entry(operation_name.to_string()).or_insert(0) += 1;
        }
        
        result
    }
    
    async fn print_statistics(&self) {
        let times = self.operation_times.lock().await;
        let errors = self.error_counts.lock().await;
        
        println!("=== 並行処理統計 ===");
        for (name, durations) in &*times {
            let avg: Duration = durations.iter().sum::<Duration>() / durations.len() as u32;
            let max = durations.iter().max().unwrap_or(&Duration::ZERO);
            let error_count = errors.get(name).unwrap_or(&0);
            
            println!("{}: 平均 {}ms, 最大 {}ms, エラー {}回", 
                name, avg.as_millis(), max.as_millis(), error_count);
        }
    }
}
```

### 7.3 テスト戦略

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_concurrent_operations() {
        // テスト用のモックペリフェラルを作成
        let mock_peripherals = create_mock_peripherals(5);
        
        // 並行操作をテスト
        let system = ConcurrentSystem::new(mock_peripherals);
        let result = system.execute_concurrent_operations().await;
        
        assert!(result.is_ok());
        
        // 全ての操作が完了したことを確認
        let completed_count = system.get_completed_count().await;
        assert_eq!(completed_count, 5);
    }
    
    #[tokio::test]
    async fn test_error_handling_in_concurrent_operations() {
        // 一部失敗するモックペリフェラルを作成
        let mock_peripherals = create_partially_failing_mocks(5, 2); // 5個中2個失敗
        
        let system = ConcurrentSystem::new(mock_peripherals);
        let result = system.execute_concurrent_operations().await;
        
        // 部分的な失敗を許容
        match result {
            Ok(_) => (),
            Err(PeripheralError::PartialFailure(results)) => {
                let success_count = results.iter().filter(|r| r.is_ok()).count();
                let error_count = results.iter().filter(|r| r.is_err()).count();
                
                assert_eq!(success_count, 3);
                assert_eq!(error_count, 2);
            }
            Err(e) => panic!("予期しないエラー: {:?}", e),
        }
    }
}
```

## 高度なパターン

### 8.1 ワークフローエンジン

```rust
struct WorkflowEngine {
    steps: Vec<WorkflowStep>,
    peripheral_pool: PeripheralPool,
}

impl WorkflowEngine {
    async fn execute_concurrent_workflow(&self) -> Result<(), PeripheralError> {
        // 依存関係に基づいてステップを並行実行
        let mut futures = HashMap::new();
        
        for step in &self.steps {
            if step.dependencies.is_empty() {
                // 依存関係がないステップは即時開始
                let future = self.execute_step(step);
                futures.insert(step.id, future);
            }
        }
        
        // 完了したステップに依存するステップを開始
        while !futures.is_empty() {
            let (step_id, result) = select_all(futures).await;
            
            match result {
                Ok(_) => {
                    // このステップに依存する次のステップを開始
                    for next_step in &self.steps {
                        if next_step.dependencies.contains(&step_id) {
                            let all_deps_complete = next_step.dependencies
                                .iter()
                                .all(|dep_id| !futures.contains_key(dep_id));
                            
                            if all_deps_complete {
                                let future = self.execute_step(next_step);
                                futures.insert(next_step.id, future);
                            }
                        }
                    }
                }
                Err(e) => {
                    // エラーハンドリング
                    eprintln!("ステップ {} 失敗: {:?}", step_id, e);
                }
            }
            
            futures.remove(&step_id);
        }
        
        Ok(())
    }
}
```

### 8.2 リアクティブプログラミング

```rust
struct ReactiveSystem {
    event_streams: Vec<EventStream>,
    processors: Vec<Box<dyn EventProcessor>>,
}

impl ReactiveSystem {
    async fn reactive_event_processing(&mut self) -> Result<(), PeripheralError> {
        // 複数イベントストリームをマージ
        let merged_stream = merge_all(self.event_streams);
        
        // イベントを処理
        let mut stream = merged_stream;
        while let Some(event) = stream.next().await {
            // イベントを並行処理
            let processor_futures: Vec<_> = self.processors
                .iter_mut()
                .map(|processor| processor.process_event(event.clone()))
                .collect();
            
            // 全プロセッサで並行処理
            let results = join_all(processor_futures).await;
            
            // 結果を分析
            self.analyze_processing_results(results).await?;
        }
        
        Ok(())
    }
}
```

## まとめ

複数ペリフェラルの並行操作は、RustComputers APIの強力な機能の一つです。このチュートリアルで学んだパターンとテクニックを活用することで、以下のような利点が得られます：

1. **効率性**: 複数の操作を同時に実行して全体の処理時間を短縮
2. **応答性**: イベント駆動システムで迅速な応答を実現
3. **堅牢性**: エラーハンドリングとリトライメカニズムで信頼性を向上
4. **スケーラビリティ**: システムの成長に合わせて拡張可能な設計

重要なポイントを再確認します：

1. **適切な並行性レベル**: システムリソースを考慮して同時実行数を制限
2. **エラー分離**: 一つの失敗が全体に影響しないように設計
3. **モニタリング**: パフォーマンスとエラー率を継続的に監視
4. **テスト**: 並行処理の特性を考慮した包括的なテスト

Rustの強力な型システムと非同期プログラミング機能を活用して、効率的で堅牢なMinecraft自動化システムを構築してください。
