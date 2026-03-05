/// RustComputers spike: Chicory Runtime Compiler 動作検証用
/// WASI stdout/stderr に出力し、Java 側でキャプチャできるか確認する
fn main() {
    println!("Hello from Rust WASM!");
    println!("RustComputers spike test - stdout capture works");

    eprintln!("This is stderr output");

    // 簡単な計算（最適化で消えないように）
    let sum: u64 = (1..=1000).sum();
    println!("Sum 1..1000 = {}", sum);

    // 環境変数の読み取りテスト（WASI env サポート確認）
    match std::env::var("RC_TEST") {
        Ok(val) => println!("RC_TEST = {}", val),
        Err(_) => println!("RC_TEST not set"),
    }
}
