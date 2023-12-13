use catalog::expression::util::Arithmetic;
use catalog::primitive::number::{Float, Integer};
use compiler::build_dsl;

fn main() {
    build_dsl! {
        Arithmetic::new()
            .add_unit(Integer::new())
            .add_unit(Float::new())
            .unwrap()
    }
}
