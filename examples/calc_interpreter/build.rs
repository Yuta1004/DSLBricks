// Prelude, macros
use compiler::entrypoint::build;
use compiler::bricks::combine_bricks;
use compiler::bricks::prelude::*;

// Bricks
use lib::catalog::expression::Expression;
use lib::catalog::primitive::number::integer::DecimalInteger;
use lib::catalog::primitive::number::fraction::DecimalFraction;
use lib::catalog::base::ExpressionBaseLanguage;

#[build]
#[combine_bricks]
fn main() -> ExpressionBaseLanguage {
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
