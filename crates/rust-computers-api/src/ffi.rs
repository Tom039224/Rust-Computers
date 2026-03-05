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
    /// - `result_ptr`: 結果バッファのアドレス / result buffer address
    /// - `result_buf_size`: 結果バッファのサイズ / result buffer size
    /// - 戻り値: request_id (>0) / return: request_id (>0)
    pub fn host_stdin_read_line(result_ptr: i32, result_buf_size: i32) -> i64;

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
    /// - `result_ptr`: 結果バッファ / result buffer
    /// - `result_buf_size`: 結果バッファサイズ / result buffer size
    /// - 戻り値: request_id (>0) | error (<0) / return: request_id (>0) | error (<0)
    pub fn host_request_info(
        periph_id: u32,
        method_id: u32,
        args_ptr: i32,
        args_len: i32,
        result_ptr: i32,
        result_buf_size: i32,
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
        result_ptr: i32,
        result_buf_size: i32,
    ) -> i64;

    /// 即時ペリフェラル情報取得（同 tick 内即返）。
    /// Immediate peripheral info request (returns within the same tick).
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

    /// 保留結果をポーリングする。
    /// Poll a pending result.
    ///
    /// - `request_id`: リクエスト ID / request ID
    /// - `written_bytes_ptr`: 書き込みバイト数の格納先アドレス / address to store written byte count
    /// - 戻り値: 0=pending, 1=ready, <0=error
    /// - return: 0=pending, 1=ready, <0=error
    pub fn host_poll_result(request_id: i64, written_bytes_ptr: i32) -> i32;

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
}
