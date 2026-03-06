//! build.rs — ペリフェラル API 自動生成スクリプト
//! Peripheral API code-generation script.
//!
//! `peripherals/*.toml` を読み込み、`$OUT_DIR/<name>_gen.rs` を出力する。
//! Reads `peripherals/*.toml` and writes `$OUT_DIR/<name>_gen.rs`.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

// ==================================================================
// TOML スキーマ / TOML schema
// ==================================================================

/// ペリフェラル全体 (= 1 つの .toml ファイル)
#[derive(Debug, Deserialize)]
struct Peripheral {
    #[serde(default)]
    method: Vec<Method>,
}

/// メソッド 1 つ分の定義
#[derive(Debug, Deserialize, Clone)]
struct Method {
    /// Lua 側のメソッド名 (例: "setCursorPos")
    lua: String,
    /// true の場合は同期 (request_info_imm) を使う
    #[serde(default)]
    immediate: bool,
    /// 戻り値の型文字列。省略時は () を返す。
    /// "i32" | "bool" | "str" | "(i32, i32)"
    #[serde(default)]
    ret: Option<String>,
    /// 引数リスト
    #[serde(default)]
    args: Vec<Arg>,
}

/// メソッド引数 1 つ分の定義
#[derive(Debug, Deserialize, Clone)]
struct Arg {
    /// Rust で用いる引数名
    name: String,
    /// 型文字列: "i32" | "str" | "bool"
    ty: String,
}

// ==================================================================
// ヘルパー / Helpers
// ==================================================================

/// camelCase → snake_case 変換
///
/// - `getSize`       → `get_size`
/// - `setCursorPos`  → `set_cursor_pos`
/// - `isAdvanced`    → `is_advanced`
/// - `clear`         → `clear`
fn camel_to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// ファイルステム ("monitor") → 構造体名 ("Monitor")
fn struct_name_from_stem(stem: &str) -> String {
    let mut chars = stem.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// TOML 型文字列 → Rust 引数型文字列
fn arg_rust_type(ty: &str) -> &str {
    match ty {
        "i32"  => "i32",
        "str"  => "&str",
        "bool" => "bool",
        _      => "i32",
    }
}

/// TOML 型文字列 → msgpack エンコード式
fn encode_arg_expr(arg: &Arg) -> String {
    match arg.ty.as_str() {
        "i32"  => format!("m::int({})", arg.name),
        "str"  => format!("m::str({})", arg.name),
        "bool" => format!("m::bool_val({})", arg.name),
        _      => format!("m::int({})", arg.name),
    }
}

/// TOML ret 文字列 → Rust 戻り値型
fn return_type(ret: Option<&str>) -> String {
    match ret {
        None              => "()".to_string(),
        Some("i32")       => "i32".to_string(),
        Some("bool")      => "bool".to_string(),
        Some("str")       => "alloc::string::String".to_string(),
        Some("(i32, i32)") => "(i32, i32)".to_string(),
        Some(other)       => other.to_string(),
    }
}

/// `data` から戻り値をデコードするコードブロック (2 インデント = 8 空白)
fn decode_return(ret: Option<&str>) -> String {
    match ret {
        None => {
            "let _ = data;\n        Ok(())".to_string()
        }
        Some("i32") => {
            "let v = m::decode_int_at(&data, 0);\n        Ok(v)".to_string()
        }
        Some("bool") => {
            "let v = m::decode_bool_at(&data, 0);\n        Ok(v)".to_string()
        }
        Some("str") => {
            concat!(
                "let s = m::decode_str_at(&data, 0);\n",
                "        Ok(alloc::string::String::from(s))"
            ).to_string()
        }
        Some("(i32, i32)") => {
            concat!(
                "let v0 = m::decode_int_at(&data, 0);\n",
                "        let v1 = m::decode_int_at(&data, 1);\n",
                "        Ok((v0, v1))"
            ).to_string()
        }
        _ => "let _ = data;\n        Ok(())".to_string(),
    }
}

// ==================================================================
// コード生成 / Code generation
// ==================================================================

/// 1 メソッドぶんの Rust コードを返す
fn generate_method(method: &Method) -> String {
    let rust_name = camel_to_snake(&method.lua);

    // fn の追加引数部分 ", x: i32, text: &str, ..."
    let extra_params: Vec<String> = method
        .args
        .iter()
        .map(|a| format!("{}: {}", a.name, arg_rust_type(&a.ty)))
        .collect();
    let params_str = if extra_params.is_empty() {
        String::new()
    } else {
        format!(", {}", extra_params.join(", "))
    };

    // 引数エンコード行
    let encode_lines: Vec<String> = method
        .args
        .iter()
        .enumerate()
        .map(|(i, a)| format!("        let _a{i} = {};", encode_arg_expr(a)))
        .collect();
    let encode_block = if encode_lines.is_empty() {
        String::new()
    } else {
        encode_lines.join("\n") + "\n"
    };

    // m::array(...)
    let array_expr = if method.args.is_empty() {
        "m::array(&[])".to_string()
    } else {
        let refs: Vec<String> = (0..method.args.len()).map(|i| format!("_a{i}")).collect();
        format!("m::array(&[{}])", refs.join(", "))
    };

    let ret_type  = return_type(method.ret.as_deref());
    let decode    = decode_return(method.ret.as_deref());

    if method.immediate {
        // 同期バージョン
        format!(
            "    /// `{}` を同期呼び出しする。\n\
             \x20   pub fn {fn_name}(&self{params}) -> Result<{ret}, BridgeError> {{\n\
             {enc}\
             \x20       let data = peripheral::request_info_imm(self.dir, \"{lua}\", &{arr})?;\n\
             \x20       {dec}\n\
             \x20   }}\n",
            method.lua,
            fn_name = rust_name,
            params   = params_str,
            ret      = ret_type,
            enc      = encode_block,
            lua      = method.lua,
            arr      = array_expr,
            dec      = decode,
        )
    } else {
        // async バージョン
        format!(
            "    /// `{}` を非同期呼び出しする。\n\
             \x20   pub async fn {fn_name}(&self{params}) -> Result<{ret}, BridgeError> {{\n\
             {enc}\
             \x20       let data = peripheral::request_info(self.dir, \"{lua}\", &{arr}).await?;\n\
             \x20       {dec}\n\
             \x20   }}\n",
            method.lua,
            fn_name = rust_name,
            params   = params_str,
            ret      = ret_type,
            enc      = encode_block,
            lua      = method.lua,
            arr      = array_expr,
            dec      = decode,
        )
    }
}

/// `Peripheral` 全体から .rs ファイル内容を生成する
fn generate_peripheral_rs(struct_name: &str, stem: &str, peripheral: &Peripheral) -> String {
    let mut out = String::new();

    out += &format!(
        "// ============================================================\n\
         // Auto-generated by build.rs from peripherals/{stem}.toml\n\
         // DO NOT EDIT — edit the .toml manifest and rebuild instead.\n\
         // ============================================================\n\
         \n\
         use crate::peripheral::{{self, Direction}};\n\
         use crate::msgpack as m;\n\
         use crate::error::BridgeError;\n\
         extern crate alloc;\n\
         \n\
         /// CC:Tweaked `{name}` ペリフェラルのタイプドラッパー。\n\
         /// Typed wrapper for the CC:Tweaked `{name}` peripheral.\n\
         ///\n\
         /// # 使い方 / Usage\n\
         ///\n\
         /// ```rust,no_run\n\
         /// use rust_computers_api::monitor::Monitor;\n\
         /// use rust_computers_api::peripheral::Direction;\n\
         ///\n\
         /// // Direction::South のモニターに hello を書く\n\
         /// let mon = Monitor::new(Direction::South);\n\
         /// // mon.write(\"hello\").await?;\n\
         /// ```\n\
         pub struct {name} {{\n\
         \x20   /// 接続方向 / Peripheral direction\n\
         \x20   pub dir: Direction,\n\
         }}\n\
         \n\
         impl {name} {{\n\
         \x20   /// 新しいラッパーを作成する。\n\
         \x20   /// Create a new typed wrapper.\n\
         \x20   pub fn new(dir: Direction) -> Self {{ Self {{ dir }} }}\n\
         \n",
        stem = stem,
        name = struct_name,
    );

    for method in &peripheral.method {
        out += &generate_method(method);
        out += "\n";
    }

    out += "}\n";
    out
}

// ==================================================================
// エントリーポイント / Entry point
// ==================================================================

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let peripherals_dir = Path::new(&manifest_dir).join("peripherals");

    if !peripherals_dir.exists() {
        println!("cargo:warning=peripherals/ not found — skipping codegen");
        return;
    }

    // peripherals/ 内の変更を監視する
    println!("cargo:rerun-if-changed=peripherals/");

    let entries = fs::read_dir(&peripherals_dir).expect("read peripherals/");

    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        // 個別ファイルも監視
        println!("cargo:rerun-if-changed={}", path.display());

        let stem = path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let struct_name = struct_name_from_stem(&stem);

        let toml_content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));

        let peripheral: Peripheral = toml::from_str(&toml_content)
            .unwrap_or_else(|e| panic!("parse {}: {}", path.display(), e));

        let rs_content = generate_peripheral_rs(&struct_name, &stem, &peripheral);

        let out_path = PathBuf::from(&out_dir).join(format!("{stem}_gen.rs"));
        fs::write(&out_path, &rs_content)
            .unwrap_or_else(|e| panic!("write {}: {}", out_path.display(), e));

        println!(
            "cargo:warning=Generated {struct_name} → {}",
            out_path.display()
        );
    }
}
