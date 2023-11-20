use compiler::build_dsl;
use library::block::expression::Arithmetic;
use library::block::primitive::{Integer, Float};

fn main() {
    let arithmetic = Arithmetic::new()
        .add_unit(Integer)
        .add_unit(Float);

    build_dsl!(arithmetic)
}
