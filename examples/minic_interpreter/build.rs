use std::rc::Rc;

use catalog::expression::util::Arithmetic;
use catalog::primitive::number::{Float, Integer};
use catalog::statement::c::{Block, ExprStatement, If};
use catalog::statement::StatementSet;
use catalog::prelude::*;
use catalog::macros::combine_bricks;
use compiler::designer::design::DSLGeneratable;
use compiler::build_dsl;

#[combine_bricks]
fn main() {
    // プリミティブ
    let integer = Integer {};
    let float = Float {};

    // 算術式
    let arithmetic = Arithmetic {
        unit: [integer, float],
    };

    // 式-文
    let expr_stmt = ExprStatement {
        expr: arithmetic,
    };

    // ブロック
    let block_stmt = Block {
        stmt: [expr_stmt],
    };

    // if 文
    let if_stmt = If {
        cond: arithmetic,
        stmt: [block_stmt],
    };

    StatementSet {
        stmt: [if_stmt, block_stmt, expr_stmt],
    }
}
