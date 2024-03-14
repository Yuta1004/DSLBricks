// Prelude, macros
use compiler::build_dsl;
use compiler::bricks::combine_bricks;
use compiler::bricks::prelude::*;

// Bricks
use catalog::expression::Expression;
use catalog::primitive::number::integer::DecimalInteger;
use catalog::primitive::number::fraction::DecimalFraction;
use catalog::base::ExpressionBaseLanguage;

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
