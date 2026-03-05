/// Phase 2 スパイク: Java ホスト関数の呼び出し + メモリ API 検証
///
/// 検証項目:
/// 1. Rust->Java コール: `env::log_str(ptr, len)` - ポインタ経由で文字列を渡す
/// 2. Rust->Java コール (戻り値): `env::compute(a, b) -> i32` - Java が計算して返す
/// 3. Java->WASM メモリ書き込み: `env::fill_buf(out_ptr, max_len) -> i32`
///    - Java が WASM の線形メモリに文字列を書き込む

// ホスト関数の宣言（"env" モジュールからインポート）
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn log_str(ptr: *const u8, len: u32);
    fn compute(a: i32, b: i32) -> i32;
    fn fill_buf(out_ptr: *mut u8, max_len: u32) -> u32;
}

fn main() {
    println!("=== Phase 2: Host Functions + Memory ===");

    // --- Test 1: Rust -> Java へ文字列ポインタ渡し ---
    println!("[Test 1] log_str: Rust->Java string via pointer");
    let msg = "Hello from Rust WASM (via pointer)!";
    unsafe {
        log_str(msg.as_ptr(), msg.len() as u32);
    }
    println!("  log_str called (check Java side for output)");

    // --- Test 2: Java に計算させて戻り値を受け取る ---
    println!("[Test 2] compute: Rust->Java->Rust value passing");
    let a: i32 = 7;
    let b: i32 = 6;
    let result = unsafe { compute(a, b) };
    println!("  compute({}, {}) = {}", a, b, result);
    assert_eq!(result, a * b + 1, "compute should return a*b+1");
    println!("  ✅ compute result correct (expected {})", a * b + 1);

    // --- Test 3: Java が WASM メモリに書き込み、Rust が読む ---
    println!("[Test 3] fill_buf: Java->WASM memory write");
    let mut buf = [0u8; 256];
    let written = unsafe { fill_buf(buf.as_mut_ptr(), buf.len() as u32) };
    let received = std::str::from_utf8(&buf[..written as usize])
        .unwrap_or("<invalid utf8>");
    println!("  Java wrote {} bytes: \"{}\"", written, received);
    assert!(received.contains("from_java"), "should contain 'from_java'");
    println!("  ✅ Java->WASM memory write correct");

    println!("\n=== Phase 2 PASSED ===");
}
