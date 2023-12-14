use std::rc::Rc;

use catalog::expression::util::Arithmetic;
use catalog::primitive::number::{Float, Integer};
use catalog::prelude::*;
use catalog::macros::combine_bricks;
use compiler::designer::design::DSLGeneratable;
use compiler::build_dsl;

#[combine_bricks]
fn main() {
    let integer = Integer {};
    let float = Float {};

    Arithmetic {
        unit: [integer, float],
    }
}
