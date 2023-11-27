// 共通定義
pub(crate) mod common;

// 制約
pub mod constraints;

// プリミティブ (int, float, ...)
pub mod primitive;

// 式 (arithmetic, ...)
pub mod expression;

// 文 (if, for, ...)
pub mod statement;

// aaa
pub mod prelude {
    pub use super::common::DSLBlock;
}
