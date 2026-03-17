//! ペリフェラルラッパー生成マクロ。
//! Macro for generating peripheral wrapper structs.
//!
//! ## 概要 / Overview
//!
//! ペリフェラルラッパー構造体とその基本実装を生成します。
//!
//! Generates peripheral wrapper structs and their basic implementations.
//!
//! ## 使い方 / Usage
//!
//! ```rust,ignore
//! peripheral_wrapper!(Inventory, "inventory");
//! ```
//!
//! 展開例 / Expansion example:
//! ```rust,ignore
//! // peripheral_wrapper!(Inventory, "inventory");
//! pub struct Inventory {
//!     addr: crate::peripheral::PeriphAddr,
//! }
//!
//! impl crate::peripheral::Peripheral for Inventory {
//!     const NAME: &'static str = "inventory";
//!     
//!     fn new(addr: crate::peripheral::PeriphAddr) -> Self {
//!         Self { addr }
//!     }
//!     
//!     fn periph_addr(&self) -> crate::peripheral::PeriphAddr {
//!         self.addr
//!     }
//! }
//!
//! impl Inventory {
//!     /// 指定アドレスから Inventory を取得します。
//!     /// Get an Inventory at the specified address.
//!     pub fn wrap(addr: impl Into<crate::peripheral::PeriphAddr>) -> Result<Self, crate::peripheral::PeripheralError> {
//!         crate::peripheral::wrap_imm(addr)
//!     }
//!     
//!     /// すべての Inventory を検索します。
//!     /// Find all Inventory peripherals.
//!     pub fn find_all() -> alloc::vec::Vec<Self> {
//!         crate::peripheral::find_imm()
//!     }
//! }
//! ```

/// ペリフェラルラッパー構造体とその基本実装を生成します。
/// Generates peripheral wrapper structs and their basic implementations.
///
/// # 使い方 / Usage
/// ```rust,ignore
/// peripheral_wrapper!(Inventory, "inventory");
/// ```
#[macro_export]
macro_rules! peripheral_wrapper {
    ($name:ident, $type_name:expr) => {
        /// ペリフェラルラッパー構造体。
        /// Peripheral wrapper struct.
        pub struct $name {
            addr: crate::peripheral::PeriphAddr,
        }

        impl crate::peripheral::Peripheral for $name {
            const NAME: &'static str = $type_name;
            
            fn new(addr: crate::peripheral::PeriphAddr) -> Self {
                Self { addr }
            }
            
            fn periph_addr(&self) -> crate::peripheral::PeriphAddr {
                self.addr
            }
        }

        impl $name {
            /// 指定アドレスからペリフェラルを取得します。
            /// Get a peripheral at the specified address.
            pub fn wrap(addr: impl Into<crate::peripheral::PeriphAddr>) -> Result<Self, crate::peripheral::PeripheralError> {
                crate::peripheral::wrap_imm(addr)
            }
            
            /// すべてのペリフェラルを検索します。
            /// Find all peripherals of this type.
            pub fn find_all() -> alloc::vec::Vec<Self> {
                crate::peripheral::find_imm()
            }
            
            /// アドレスを取得します。
            /// Get the address.
            pub fn addr(&self) -> crate::peripheral::PeriphAddr {
                self.addr
            }
        }
    };
}