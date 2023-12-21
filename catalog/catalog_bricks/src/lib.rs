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

// プリミティブ (int, float, ...)
pub mod primitive;

// 式 (arithmetic, ...)
pub mod expression;

// 文 (if, for, ...)
pub mod statement;
