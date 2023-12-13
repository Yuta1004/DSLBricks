use std::rc::Rc;

use catalog::expression::util::Arithmetic;
use catalog::primitive::number::{Float, Integer};
use catalog::statement::c::{Block, ExprStatement, If};
use catalog::statement::StatementSet;
use catalog::prelude::*;
use compiler::build_dsl;

fn main() {
    // 算術式
    let arithmetic = Arithmetic::new()
        .add_unit(Integer::new())
        .add_unit(Float::new());

    // 式-文
    let expr_stmt = ExprStatement::new().set_expr(Rc::clone(&arithmetic));

    // ブロック
    let block_stmt = Block::new().add_stmt(Rc::clone(&expr_stmt));

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
