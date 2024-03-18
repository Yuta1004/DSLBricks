// Prelude, macros
use compiler::build_dsl;
use compiler::bricks::combine_bricks;
use compiler::bricks::prelude::*;

// Bricks
use lib::catalog::expression::Expression;
use lib::catalog::primitive::number::integer::DecimalInteger;
use lib::catalog::primitive::number::fraction::DecimalFraction;
use lib::catalog::base::ExpressionBaseLanguage;

#[combine_bricks]
fn main() {
    // プリミティブ
    let integer = DecimalInteger {};
    let fraction = DecimalFraction {};

    // 算術式
    let expr = Expression {
        unit: [integer, fraction],
    };

    ExpressionBaseLanguage {
        expr: [expr],
    }
}
