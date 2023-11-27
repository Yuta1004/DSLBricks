use std::rc::Rc;

use compiler::build_dsl;
use library::prelude::*;
use library::expression::util::Arithmetic;
use library::primitive::{Integer, Float};
use library::statement::c::{Block, ExprStatement, If};
use library::statement::StatementSet;

fn main() {
    // 算術式
    let arithmetic = Arithmetic::new()
        .add_unit(Integer::new())
        .add_unit(Float::new());

    // 式-文
    let expr_stmt = ExprStatement::new()
        .set_expr(Rc::clone(&arithmetic));

    // ブロック
    let block_stmt = Block::new()
        .add_stmt(Rc::clone(&expr_stmt));

    // if 文
    let if_stmt = If::new()
        .set_cond(Rc::clone(&arithmetic))
        .add_stmt(Rc::clone(&block_stmt));

    build_dsl! {
        StatementSet::new()
            .add_stmt(if_stmt)
            .add_stmt(block_stmt)
            .add_stmt(expr_stmt)
            .unwrap()
    }
}
