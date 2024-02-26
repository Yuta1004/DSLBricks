// 共通定義など
mod common;
pub mod prelude {
    pub use std::rc::Rc;

    pub use compiler::designer::design::DSLGeneratable;
    pub use compiler::build_dsl;

    pub use crate::common::*;
}

// 制約
pub mod constraints;

// 基礎構造
pub mod base;

// プリミティブ
pub mod primitive;

// 式
pub mod expression;

// 文
pub mod statement;

// 関数
pub mod function;

// 構造体
pub mod r#struct;
