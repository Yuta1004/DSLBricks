// Prelude, macros
use compiler::build_dsl;
use compiler::bricks::combine_bricks;
use compiler::bricks::prelude::*;

// Bricks
use lib::catalog::primitive::number::integer::DecimalInteger;
use lib::catalog::primitive::number::fraction::DecimalFraction;
use lib::catalog::primitive::identifier::CStyleIdentifier;
use lib::catalog::expression::Expression;
use lib::catalog::statement::c::{Block, ExprStatement, If, For, While};
use lib::catalog::function::c::Function;
use lib::catalog::r#struct::c::Struct;
use lib::catalog::base::DeclaringBaseLanguage;

#[combine_bricks]
fn main() {
    // プリミティブ
    let integer = DecimalInteger {};
    let fraction = DecimalFraction {};
    let identifier = CStyleIdentifier {};

    // 算術式
    let expr = Expression {
        unit: [integer, fraction, identifier],
    };

    // 式-文
    let expr_stmt = ExprStatement {
        expr: expr,
    };

    // if 文
    let if_stmt = If {
        cond: expr,
        stmt: [block_stmt],
    };

    // for 文
    let for_stmt = For {
        init: expr,
        cond: expr,
        incr: expr,
        stmt: [block_stmt],
    };

    // while 文
    let while_stmt = While {
        cond: expr,
        stmt: [block_stmt],
    };

    // ブロック
    let block_stmt = Block {
        stmt: [expr_stmt, if_stmt, for_stmt, while_stmt],
    };

    // 関数
    let function = Function {
        id: identifier,
        stmt: [block_stmt],
    };

    // 構造体
    let r#struct = Struct {
        id: identifier,
    };

    DeclaringBaseLanguage {
        declare: [function, r#struct],
    }
}
