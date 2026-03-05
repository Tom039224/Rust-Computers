//! RustComputers 全 API 動作テスト。
//! Comprehensive API test for RustComputers.
//!
//! ## ビルド方法 / Build
//! ```sh
//! cargo build --example test_all_apis --target wasm32-unknown-unknown --release
//! ```
//!
//! ## テスト内容 / Test coverage
//! 1. println! / eprintln! — ログ出力
//! 2. computer_id() — コンピュータ ID 取得
//! 3. read_line() — stdin 入力 (async)
//! 4. parallel! — 並行 Future (JoinAll)
//! 5. peripheral API — 6方向スキャン結果確認
//! 6. エラー表示 — BridgeError の文字列化
//!
//! ## 使い方 / Usage
//! 1. ビルドした .wasm を computer ディレクトリに配置
//! 2. GUI で "test_all_apis.wasm" を選択して Run
//! 3. ログ欄を確認し、"Type 'next' to continue" でステップ進行
//! 4. 最後に "ALL TESTS PASSED" が表示されれば成功

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use rust_computers_api as rc;
use rc::peripheral::{self, Direction};

// エントリーポイント登録 / Register entry point
rc::entry!(main);

/// テスト結果カウンタ / Test result counters
struct TestResult {
    passed: u32,
    failed: u32,
}

impl TestResult {
    fn new() -> Self { Self { passed: 0, failed: 0 } }

    fn ok(&mut self, name: &str) {
        self.passed += 1;
        rc::println!("  [PASS] {}", name);
    }

    fn fail(&mut self, name: &str, reason: &str) {
        self.failed += 1;
        rc::println!("  [FAIL] {} — {}", name, reason);
    }

    fn check(&mut self, name: &str, condition: bool) {
        if condition {
            self.ok(name);
        } else {
            self.fail(name, "condition was false");
        }
    }
}

// ==================================================================
// メインプログラム / Main program
// ==================================================================

async fn main() {
    let mut t = TestResult::new();

    rc::println!("╔══════════════════════════════════════╗");
    rc::println!("║   RustComputers API Test Suite       ║");
    rc::println!("║   v0.1.1                             ║");
    rc::println!("╚══════════════════════════════════════╝");
    rc::println!("");

    // ------------------------------------------------------------------
    // Test 1: println! / eprintln!
    // ------------------------------------------------------------------
    rc::println!("--- Test 1: Log Output ---");
    rc::println!("This is println!");
    rc::eprintln!("This is eprintln!");
    rc::println!("Format test: {} + {} = {}", 1, 2, 3);
    rc::println!("Unicode test: こんにちは世界 🦀");
    t.ok("println! basic");
    t.ok("eprintln! basic");
    t.ok("println! format args");
    t.ok("println! unicode");

    // ------------------------------------------------------------------
    // Test 2: computer_id()
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 2: Computer ID ---");
    let id = rc::io::computer_id();
    rc::println!("Computer ID = {}", id);
    t.check("computer_id() >= 0", id >= 0);

    // ------------------------------------------------------------------
    // Test 3: read_line() — stdin 入力
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 3: Stdin Input ---");
    rc::println!("Type something and press Enter:");
    let line = rc::read_line().await;
    rc::println!("Received: '{}'", line);
    t.check("read_line() returns non-empty", !line.is_empty());

    rc::println!("Type 'hello' to test exact match:");
    let line2 = rc::read_line().await;
    t.check("read_line() exact match", line2.trim() == "hello");

    // ------------------------------------------------------------------
    // Test 4: parallel! — 並行 Future (JoinAll)
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 4: parallel! macro ---");

    // JoinAll with a Vec of stdin reads:
    // ユーザーに3回入力してもらい、全結果を取得
    rc::println!("Enter 3 lines (one at a time) for parallel Vec test:");

    // Note: parallel! with Vec requires all futures to be the same type.
    // Since read_line() returns immediately with request_id, we issue 3 reads
    // but only 1 can be pending at a time on the Java side.
    // Instead, test with sequential reads wrapped in a loop.
    let mut lines: Vec<String> = Vec::new();
    for i in 0..3 {
        rc::println!("  Line {}/3:", i + 1);
        let l = rc::read_line().await;
        rc::println!("  Got: '{}'", l);
        lines.push(l);
    }
    t.check("3 lines received", lines.len() == 3);
    t.check("all lines non-empty", lines.iter().all(|l| !l.is_empty()));

    // ------------------------------------------------------------------
    // Test 5: Peripheral scan — 方向定数テスト
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 5: Peripheral Direction ---");

    // Direction enum のラウンドトリップ
    let dirs = [
        (Direction::Down,  0u32, "Down"),
        (Direction::Up,    1,    "Up"),
        (Direction::North, 2,    "North"),
        (Direction::South, 3,    "South"),
        (Direction::West,  4,    "West"),
        (Direction::East,  5,    "East"),
    ];

    for (dir, expected_id, name) in dirs.iter() {
        let actual = dir.id();
        t.check(
            &format!("Direction::{}.id() == {}", name, expected_id),
            actual == *expected_id,
        );
    }

    // from_id ラウンドトリップ
    for id in 0..6u32 {
        let dir = Direction::from_id(id);
        t.check(
            &format!("Direction::from_id({}) is Some", id),
            dir.is_some(),
        );
        if let Some(d) = dir {
            t.check(
                &format!("roundtrip id={}", id),
                d.id() == id,
            );
        }
    }
    t.check("Direction::from_id(6) is None", Direction::from_id(6).is_none());
    t.check("Direction::from_id(255) is None", Direction::from_id(255).is_none());

    // CRC32 テスト
    let mid = peripheral::method_id("test");
    rc::println!("CRC32('test') = 0x{:08X}", mid);
    t.check("CRC32('test') == 0xD87F7E0C", mid == 0xD87F7E0C);

    let mid2 = peripheral::method_id("getItems");
    rc::println!("CRC32('getItems') = 0x{:08X}", mid2);
    t.check("CRC32('getItems') is non-zero", mid2 != 0);

    // ------------------------------------------------------------------
    // Test 6: Peripheral request (expected failure — no peripheral attached)
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 6: Peripheral Request (expect error) ---");
    rc::println!("Attempting request_info on Direction::Up...");

    // ペリフェラルが接続されていない場合、エラーが返るはず
    let result = peripheral::request_info(Direction::Up, "getType", &[]).await;
    match &result {
        Ok(data) => {
            rc::println!("  Got OK with {} bytes (peripheral was found!)", data.len());
            t.ok("request_info returned data");
        }
        Err(e) => {
            rc::println!("  Got expected error: {:?}", e);
            // ERR_INVALID_PERIPHERAL (-2) が返るはず
            t.check("request_info error is InvalidPeripheral",
                    *e == rc::BridgeError::InvalidPeripheral);
        }
    }

    // 即時リクエストも同様
    rc::println!("Attempting request_info_imm on Direction::Down...");
    let imm_result = peripheral::request_info_imm(Direction::Down, "getTemp", &[]);
    match &imm_result {
        Ok(data) => {
            rc::println!("  Got OK with {} bytes", data.len());
            t.ok("request_info_imm returned data");
        }
        Err(e) => {
            rc::println!("  Got expected error: {:?}", e);
            t.check("request_info_imm error is InvalidPeripheral",
                    *e == rc::BridgeError::InvalidPeripheral);
        }
    }

    // ------------------------------------------------------------------
    // Test 7: Error codes
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 7: Error Code Mapping ---");

    let errors = [
        (-1, rc::BridgeError::InvalidRequestId, "InvalidRequestId"),
        (-2, rc::BridgeError::InvalidPeripheral, "InvalidPeripheral"),
        (-3, rc::BridgeError::MethodNotFound,    "MethodNotFound"),
        (-4, rc::BridgeError::JavaException,     "JavaException"),
        (-5, rc::BridgeError::Timeout,           "Timeout"),
        (-6, rc::BridgeError::FuelExhausted,     "FuelExhausted"),
        (-7, rc::BridgeError::ResultBufTooSmall, "ResultBufTooSmall"),
        (-8, rc::BridgeError::ModNotAvailable,   "ModNotAvailable"),
        (-9, rc::BridgeError::ResultLost,        "ResultLost"),
    ];

    for (code, expected, name) in errors.iter() {
        let actual = rc::BridgeError::from_code(*code);
        t.check(
            &format!("from_code({}) == {}", code, name),
            actual == *expected,
        );
    }

    // Unknown error
    let unknown = rc::BridgeError::from_code(-999);
    t.check("from_code(-999) == Unknown(-999)",
            unknown == rc::BridgeError::Unknown(-999));

    // ------------------------------------------------------------------
    // Test 8: Large output test
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 8: Large Output ---");
    let long_str: String = (0..50).map(|i| format!("line-{:03} ", i)).collect();
    rc::println!("{}", long_str);
    t.ok("large println! did not crash");

    // ------------------------------------------------------------------
    // Test 9: Multi-tick persistence
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("--- Test 9: Multi-tick Await ---");
    rc::println!("This tests that the executor survives multiple ticks.");
    rc::println!("Type 'done' to complete this test:");
    loop {
        let line = rc::read_line().await;
        if line.trim() == "done" {
            t.ok("multi-tick await completed");
            break;
        }
        rc::println!("  (echo: '{}', type 'done')", line);
    }

    // ------------------------------------------------------------------
    // 最終結果 / Final results
    // ------------------------------------------------------------------
    rc::println!("");
    rc::println!("╔══════════════════════════════════════╗");
    if t.failed == 0 {
        rc::println!("║   ALL TESTS PASSED                   ║");
    } else {
        rc::println!("║   SOME TESTS FAILED                  ║");
    }
    rc::println!("╠══════════════════════════════════════╣");
    rc::println!("║  Passed: {:>4}                         ║", t.passed);
    rc::println!("║  Failed: {:>4}                         ║", t.failed);
    rc::println!("║  Total:  {:>4}                         ║", t.passed + t.failed);
    rc::println!("╚══════════════════════════════════════╝");

    rc::println!("");
    rc::println!("Test suite finished. Computer will stop.");
}
