use compiler::build_dsl;
use library::expression::util::Arithmetic;
use library::primitive::number::{Float, Integer};

fn main() {
    build_dsl! {
        Arithmetic::new()
            .add_unit(Integer::new())
            .add_unit(Float::new())
            .unwrap()
    }
}
