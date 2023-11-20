use compiler::build_dsl;
use library::block::expression::util::Arithmetic;
use library::block::primitive::{Integer, Float};

fn main() {
    build_dsl! {
        Arithmetic::new()
            .add_unit(Integer)
            .add_unit(Float)
    }
}
