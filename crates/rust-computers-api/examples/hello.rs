//! RustComputers サンプルプログラム — Hello World + stdin エコー。
//! Sample RustComputers program — Hello World + stdin echo.
//!
//! ## ビルド方法 / Build
//! ```sh
//! cargo build --example hello --target wasm32-unknown-unknown --release
//! ```
//!
//! ## 使い方 / Usage
//! 1. ビルドした .wasm ファイルをコンピューターにアップロード
//! 2. GUI で "Run" を押す
//! 3. ログ欄と入力欄で対話
//!
//! 1. Upload the built .wasm file to the computer
//! 2. Press "Run" in the GUI
//! 3. Interact via the log panel and input field

#![no_std]
#![no_main]

extern crate alloc;

use rust_computers_api as rc;

// エントリーポイント登録 / Register entry point
rc::entry!(main);

/// メインプログラム / Main program
async fn main() {
    let id = rc::io::computer_id();
    rc::println!("=== Hello from Rust Computer #{} ===", id);
    rc::println!("Type something and press Enter:");

    loop {
        let line = rc::read_line().await;

        if line.is_empty() {
            continue;
        }

        if line == "exit" || line == "quit" {
            rc::println!("Goodbye!");
            break;
        }

        rc::println!("Echo: {}", line);
    }

    rc::println!("Program finished.");
}
