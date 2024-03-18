// Prelude, macros
use compiler::bricks::combine_bricks;
use compiler::bricks::prelude::*;
use compiler::entrypoint;

// Bricks
use lib::catalog::base::ExpressionBaseLanguage;
use lib::catalog::expression::Expression;
use lib::catalog::primitive::number::fraction::DecimalFraction;
use lib::catalog::primitive::number::integer::DecimalInteger;

#[entrypoint::build]
#[combine_bricks]
fn build() -> ExpressionBaseLanguage {
    // プリミティブ
    let integer = DecimalInteger {};
    let fraction = DecimalFraction {};

    // 算術式
    let expr = Expression {
        unit: [integer, fraction],
    };

    ExpressionBaseLanguage { expr: [expr] }
}
