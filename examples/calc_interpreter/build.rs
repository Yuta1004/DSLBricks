use std::rc::Rc;

use catalog::expression::Expression;
use catalog::primitive::number::integer::DecimalInteger;
use catalog::primitive::number::fraction::DecimalFraction;
use catalog::base::ExpressionBaseLanguage;
use catalog::prelude::*;
use catalog::macros::combine_bricks;

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
