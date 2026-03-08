//! ホスト関数の FFI 宣言。
//! FFI declarations for host functions.
//!
//! Java (Chicory) 側が提供するホスト関数を Rust から呼び出すための
//! `extern "C"` 宣言。全て `"env"` モジュールに属する。
//!
//! `extern "C"` declarations for calling host functions provided by
//! the Java (Chicory) side. All belong to the `"env"` module.

// 安全性: これらの関数は WASM ホスト環境によって提供される。
// Safety: These functions are provided by the WASM host environment.
#[link(wasm_import_module = "env")]
extern "C" {
    // ==================================================================
    // ログ出力 / Logging
    // ==================================================================

    /// ログ文字列を Java 側 GUI に出力する。
    /// Output a log string to the Java-side GUI.
    ///
    /// - `ptr`: UTF-8 文字列の先頭アドレス / pointer to UTF-8 string
    /// - `len`: バイト長 / byte length
    pub fn host_log(ptr: i32, len: i32);

    // ==================================================================
    // 標準入力 / Standard input
    // ==================================================================

    /// 行入力リクエストを発行する。Enter が押されるまで Pending。
    /// Issue a line-input request. Pending until Enter is pressed.
    ///
    /// 結果は `host_poll_result` / `host_fetch_result` の 2 フェーズで取得する。
    /// Retrieve the result using the two-phase `host_poll_result` / `host_fetch_result`.
    ///
    /// - 戻り値: request_id (>0) / return: request_id (>0)
    pub fn host_stdin_read_line() -> i64;

    // ==================================================================
    // ペリフェラル操作 / Peripheral operations
    // ==================================================================

    /// ペリフェラル情報取得リクエスト（非同期）。
    /// Peripheral info request (async).
    ///
    /// - `periph_id`: ペリフェラル ID / peripheral ID
    /// - `method_id`: メソッド ID (CRC32) / method ID (CRC32)
    /// - `args_ptr`: 引数バッファ（MessagePack）/ argument buffer (MessagePack)
    /// - `args_len`: 引数バイト長 / argument byte length
    /// - 戻り値: request_id (>0) | error (<0) / return: request_id (>0) | error (<0)
    ///
    /// 結果は `host_poll_result` / `host_fetch_result` の 2 フェーズで取得する。
    /// Retrieve the result using the two-phase `host_poll_result` / `host_fetch_result`.
    pub fn host_request_info(
        periph_id: u32,
        method_id: u32,
        args_ptr: i32,
        args_len: i32,
    ) -> i64;

    /// ペリフェラルアクション実行リクエスト（非同期）。
    /// Peripheral action request (async).
    ///
    /// シグネチャは `host_request_info` と同じ。
    /// Signature is the same as `host_request_info`.
    pub fn host_do_action(
        periph_id: u32,
        method_id: u32,
        args_ptr: i32,
        args_len: i32,
    ) -> i64;

    /// 即時ペリフェラル情報取得（同 tick 内即返）。
    /// Immediate peripheral info request (returns within the same tick).
    ///
    /// **用途**: `@LuaFunction(immediate=true)` として実装されたメソッド専用。
    /// 1tick 遅れ原則の**意図的な例外**であり、`_imm` サフィックスにより明示される。
    ///
    /// **Usage**: For methods implemented as `@LuaFunction(immediate=true)` only.
    /// This is an intentional exception to the 1-tick delay principle,
    /// made explicit by the `_imm` suffix.
    ///
    /// - 戻り値: written_bytes (>=0) | error (<0)
    /// - return: written_bytes (>=0) | error (<0)
    pub fn host_request_info_imm(
        periph_id: u32,
        method_id: u32,
        args_ptr: i32,
        args_len: i32,
        result_ptr: i32,
        result_buf_size: i32,
    ) -> i32;

    // ==================================================================
    // ポーリング / Polling
    // ==================================================================

    /// 保留結果をポーリングする（フェーズ 1: サイズ確認）。
    /// Poll a pending result (Phase 1: check size).
    ///
    /// - `request_id`: リクエスト ID / request ID
    /// - 戻り値:
    ///   - `0`           : まだ未完了 (Pending) / still pending
    ///   - 正値 (>0)     : 完了。結果データのバイト数 / ready; result size in bytes
    ///   - 負値 (<0)     : エラーコード / error code
    ///
    /// 完了 (正値) を受け取ったら、Rust 側でそのサイズ分のバッファを動的確保して
    /// `host_fetch_result` を呼び出す（フェーズ 2）。
    ///
    /// When a positive value is received, allocate a buffer of that size on the Rust
    /// side and call `host_fetch_result` (Phase 2).
    pub fn host_poll_result(request_id: i64) -> i64;

    /// 完了した結果データを WASM バッファに転送する（フェーズ 2: データ取得）。
    /// Transfer completed result data into a WASM buffer (Phase 2: fetch data).
    ///
    /// `host_poll_result` が正値を返した直後に呼び出す。
    /// Call immediately after `host_poll_result` returns a positive value.
    ///
    /// - `request_id`    : リクエスト ID / request ID
    /// - `result_ptr`    : Rust 側が動的確保したバッファのアドレス / address of Rust-allocated buffer
    /// - `result_buf_size`: バッファサイズ（`host_poll_result` が返した値以上） /
    ///                      buffer size (must be ≥ the value returned by `host_poll_result`)
    /// - 戻り値: 書き込みバイト数 (>=0) | エラーコード (<0) /
    ///           return: written bytes (>=0) | error code (<0)
    pub fn host_fetch_result(request_id: i64, result_ptr: i32, result_buf_size: i32) -> i32;

    // ==================================================================
    // メタ情報 / Meta information
    // ==================================================================

    /// 指定 Mod が利用可能か確認する。
    /// Check whether the specified mod is available.
    ///
    /// - 戻り値: 1=available, 0=not available
    /// - return: 1=available, 0=not available
    pub fn host_is_mod_available(mod_id: u16) -> i32;

    /// このコンピューターの ID を返す。
    /// Return the ID of this computer.
    pub fn host_get_computer_id() -> i32;

    // ==================================================================
    // ペリフェラル検索 / Peripheral search
    // ==================================================================

    /// 型名に一致するペリフェラルの periph_id 一覧を即時返す。
    /// Immediately return a list of periph_ids for all peripherals matching the given type name.
    ///
    /// 直結6方向 (periph_id 0–5) および有線モデム経由接続 (periph_id 6+) を含む。
    /// Includes directly-connected 6 directions (periph_id 0–5) and
    /// wired-modem-connected peripherals (periph_id 6+).
    ///
    /// CC:Tweaked の `peripheral.find("<type>")` に相当。
    /// Equivalent to CC:Tweaked's `peripheral.find("<type>")`.
    ///
    /// - `name_ptr`: 型名 UTF-8 文字列のポインタ / pointer to type name UTF-8 string
    /// - `name_len`: 型名バイト長 / byte length of type name
    /// - `result_ptr`: 結果バッファ (msgpack array of u32) / result buffer (msgpack array of u32)
    /// - `result_buf_size`: 結果バッファサイズ / result buffer size
    /// - 戻り値: 書き込みバイト数 (>=0) | エラー (<0)
    /// - return: written bytes (>=0) | error (<0)
    pub fn host_find_peripherals_by_type_imm(
        name_ptr: i32,
        name_len: i32,
        result_ptr: i32,
        result_buf_size: i32,
    ) -> i32;
}
