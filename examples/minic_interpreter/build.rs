use compiler::build_dsl;
use library::block::expression::util::Arithmetic;
use library::block::primitive::{Integer, Float};
use library::block::statement::c::{Block, ExprStatement, If};
use library::block::statement::StatementSet;

fn main() {
    // 算術式
    let arithmetic = Arithmetic::new()
        .add_unit(Integer)
        .add_unit(Float);

    // 式-文
    let expr_stmt = ExprStatement::new()
        .set_expr(arithmetic.clone());

    // ブロック
    let block_stmt = Block::new()
        .add_stmt(expr_stmt.clone());

    // if 文
    let if_stmt = If::new()
        .set_cond(arithmetic)
        .add_stmt(block_stmt.clone());

    build_dsl! {
        StatementSet::new()
            .add_stmt(if_stmt)
            .add_stmt(block_stmt)
            .add_stmt(expr_stmt)
    }
}
