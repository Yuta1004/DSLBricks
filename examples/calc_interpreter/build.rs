use std::rc::Rc;

use catalog::expression::util::Arithmetic;
use catalog::primitive::number::{Float, Integer};
use catalog::prelude::*;
use catalog::macros::combine_bricks;

#[combine_bricks]
fn main() {
    let integer = Integer {};
    let float = Float {};

    Arithmetic {
        unit: [integer, float],
    }
}
