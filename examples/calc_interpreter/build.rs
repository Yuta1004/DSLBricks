use std::rc::Rc;

use catalog::expression::Expression;
use catalog::primitive::number::integer::DecimalInteger;
use catalog::primitive::number::fraction::DecimalFraction;
use catalog::prelude::*;
use catalog::macros::combine_bricks;

#[combine_bricks]
fn main() {
    let integer = DecimalInteger {};
    let fraction = DecimalFraction {};

    Expression {
        unit: [integer, fraction],
    }
}
