use blockdsl::designer::codegen::rbuild_dsl;
use blockdsl::designer::design::{DSLDesign, DSLPart};

#[derive(Debug, Default)]
struct MyDSL;

impl DSLDesign for MyDSL {
    fn design() -> Vec<Box<dyn DSLPart>> {
        vec![]
    }
}

fn main() {
    rbuild_dsl!(MyDSL);
}
