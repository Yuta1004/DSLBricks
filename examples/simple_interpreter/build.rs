use blockdsl::designer::codegen::rbuild_dsl;
use blockdsl::designer::design::syntax::{Rule, RuleSet, SyntaxElem};
use blockdsl::designer::design::DSLGeneratable;

struct MyDSL;

impl DSLGeneratable for MyDSL {
    fn design(self) -> RuleSet {
        vec![
            Rule::from(("top", vec![SyntaxElem::Term("a")])),
        ].into()
    }
}

fn main() {
    rbuild_dsl!(MyDSL);
}
