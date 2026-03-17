//! book-read パターンマクロ。
//! Macros for the book-read pattern.
//!
//! ## 概要 / Overview
//!
//! 以下の3種類のメソッドを生成します：
//!
//! Generates three types of methods:
//!
//! 1. **book_next_*()** - リクエストを予約する（非ブロッキング）
//! 2. **read_last_*()** - 前回の結果を読み取る
//! 3. **async_*()** - 非同期メソッド（book + wait + read）

/// 取得系メソッド（上書き動作）用の book-next/read-last ペアを生成します。
/// Generates book-next/read-last pair for info/query methods (overwrite behavior).
///
/// # 使い方 / Usage
/// ```rust,ignore
/// book_read!(size, "size", (), u32);
/// ```
///
/// 展開例 / Expansion example:
/// ```rust,ignore
/// // book_read!(size, "size", (), u32);
/// pub fn book_next_size(&mut self) {
///     crate::peripheral::book_request(self.addr, "size", &[]);
/// }
/// pub fn read_last_size(&self) -> Result<u32, crate::peripheral::PeripheralError> {
///     let bytes = crate::peripheral::read_result(self.addr, "size")?;
///     crate::peripheral::decode(&bytes)
/// }
/// ```
#[macro_export]
macro_rules! book_read {
    // 引数なしの場合 / No arguments
    ($method:ident, $lua_method:expr, (), $ret:ty) => {
        book_read!($method, $lua_method, (), $ret, ());
    };
    
    // 引数ありの場合 / With arguments
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty) => {
        book_read!($method, $lua_method, $arg, $ret, $arg);
    };
    
    // 内部実装 / Internal implementation
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty, $arg_inner:ty) => {
        /// リクエストを予約する（非ブロッキング）。
        /// Book a request (non-blocking).
        pub fn book_next_$method(&mut self $(, args: $arg_inner)?) {
            let bytes = if stringify!($arg_inner) == "()" {
                vec![]
            } else {
                crate::peripheral::encode(&args).unwrap_or_default()
            };
            crate::peripheral::book_request(self.addr, $lua_method, &bytes);
        }
        
        /// 前回の結果を読み取る。
        /// Read the last result.
        pub fn read_last_$method(&self) -> Result<$ret, crate::peripheral::PeripheralError> {
            let bytes = crate::peripheral::read_result(self.addr, $lua_method)?;
            crate::peripheral::decode(&bytes)
        }
    };
}

/// 反映系メソッド（蓄積動作）用の book-next/read-last ペアを生成します。
/// Generates book-next/read-last pair for action methods (accumulate behavior).
///
/// # 使い方 / Usage
/// ```rust,ignore
/// book_action!(push_item, "pushItem", PushItemArgs, ());
/// ```
///
/// 展開例 / Expansion example:
/// ```rust,ignore
/// // book_action!(push_item, "pushItem", PushItemArgs, ());
/// pub fn book_next_push_item(&mut self, args: PushItemArgs) {
///     let bytes = crate::peripheral::encode(&args).unwrap_or_default();
///     crate::peripheral::book_action(self.addr, "pushItem", &bytes);
/// }
/// pub fn read_last_push_item(&self) -> Vec<Result<(), crate::peripheral::PeripheralError>> {
///     let results = crate::peripheral::read_action_results(self.addr, "pushItem");
///     results.into_iter()
///         .map(|r| r.and_then(|bytes| crate::peripheral::decode(&bytes)))
///         .collect()
/// }
/// ```
#[macro_export]
macro_rules! book_action {
    // 引数なしの場合 / No arguments
    ($method:ident, $lua_method:expr, (), $ret:ty) => {
        book_action!($method, $lua_method, (), $ret, ());
    };
    
    // 引数ありの場合 / With arguments
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty) => {
        book_action!($method, $lua_method, $arg, $ret, $arg);
    };
    
    // 内部実装 / Internal implementation
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty, $arg_inner:ty) => {
        /// アクションを予約する（非ブロッキング、蓄積動作）。
        /// Book an action (non-blocking, accumulate behavior).
        pub fn book_next_$method(&mut self $(, args: $arg_inner)?) {
            let bytes = if stringify!($arg_inner) == "()" {
                vec![]
            } else {
                crate::peripheral::encode(&args).unwrap_or_default()
            };
            crate::peripheral::book_action(self.addr, $lua_method, &bytes);
        }
        
        /// 前回のアクション結果を読み取る。
        /// Read the last action results.
        pub fn read_last_$method(&self) -> Vec<Result<$ret, crate::peripheral::PeripheralError>> {
            let results = crate::peripheral::read_action_results(self.addr, $lua_method);
            results.into_iter()
                .map(|r| r.and_then(|bytes| crate::peripheral::decode(&bytes)))
                .collect()
        }
    };
}

/// 非同期メソッド（book + wait + read）を生成します。
/// Generates async method (book + wait + read).
///
/// # 使い方 / Usage
/// ```rust,ignore
/// async_method!(async_size, "size", (), u32);
/// ```
///
/// 展開例 / Expansion example:
/// ```rust,ignore
/// // async_method!(async_size, "size", (), u32);
/// pub async fn async_size(&mut self) -> Result<u32, crate::peripheral::PeripheralError> {
///     self.book_next_size();
///     crate::wait_for_next_tick().await;
///     self.read_last_size()
/// }
/// ```
#[macro_export]
macro_rules! async_method {
    // 引数なしの場合 / No arguments
    ($method:ident, $lua_method:expr, (), $ret:ty) => {
        async_method!($method, $lua_method, (), $ret, ());
    };
    
    // 引数ありの場合 / With arguments
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty) => {
        async_method!($method, $lua_method, $arg, $ret, $arg);
    };
    
    // 内部実装 / Internal implementation
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty, $arg_inner:ty) => {
        /// 非同期メソッド（book + wait + read）。
        /// Async method (book + wait + read).
        pub async fn async_$method(&mut self $(, args: $arg_inner)?) -> Result<$ret, crate::peripheral::PeripheralError> {
            if stringify!($arg_inner) == "()" {
                self.book_next_$method();
            } else {
                self.book_next_$method(args);
            }
            crate::wait_for_next_tick().await;
            self.read_last_$method()
        }
    };
}

/// 即時メソッド（同 tick 内で完結）を生成します。
/// Generates immediate method (completes in the same tick).
///
/// # 使い方 / Usage
/// ```rust,ignore
/// imm_method!(size_imm, "size", (), u32);
/// ```
///
/// 展開例 / Expansion example:
/// ```rust,ignore
/// // imm_method!(size_imm, "size", (), u32);
/// pub fn size_imm(&self) -> Result<u32, crate::peripheral::PeripheralError> {
///     let bytes = crate::peripheral::request_info_imm(self.addr, "size", &[])?;
///     crate::peripheral::decode(&bytes)
/// }
/// ```
#[macro_export]
macro_rules! imm_method {
    // 引数なしの場合 / No arguments
    ($method:ident, $lua_method:expr, (), $ret:ty) => {
        imm_method!($method, $lua_method, (), $ret, ());
    };
    
    // 引数ありの場合 / With arguments
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty) => {
        imm_method!($method, $lua_method, $arg, $ret, $arg);
    };
    
    // 内部実装 / Internal implementation
    ($method:ident, $lua_method:expr, $arg:ty, $ret:ty, $arg_inner:ty) => {
        /// 即時メソッド（同 tick 内で完結）。
        /// Immediate method (completes in the same tick).
        ///
        /// ⚠️ このメソッドは `@LuaFunction(immediate=true)` として実装された
        /// Java メソッド専用です。
        ///
        /// ⚠️ This method is only for Java methods implemented as
        /// `@LuaFunction(immediate=true)`.
        pub fn $method(&self $(, args: $arg_inner)?) -> Result<$ret, crate::peripheral::PeripheralError> {
            let bytes = if stringify!($arg_inner) == "()" {
                vec![]
            } else {
                crate::peripheral::encode(&args).unwrap_or_default()
            };
            let result_bytes = crate::peripheral::request_info_imm(self.addr, $lua_method, &bytes)?;
            crate::peripheral::decode(&result_bytes)
        }
    };
}