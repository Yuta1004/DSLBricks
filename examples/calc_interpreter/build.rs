use compiler::build_dsl;
use library::prelude::*;
use library::expression::util::Arithmetic;
use library::primitive::{Integer, Float};

fn main() {
    build_dsl! {
        Arithmetic::new()
            .add_unit(Integer::new())
            .add_unit(Float::new())
    }
}
