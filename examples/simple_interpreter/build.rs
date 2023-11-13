use blockdsl::designer::codegen::rbuild_dsl;
use blockdsl::designer::design::syntax::{Rule, RuleSet, SyntaxElem};
use blockdsl::designer::design::DSLGeneratable;

#[derive(Debug)]
struct MyDSL;

impl DSLGeneratable for MyDSL {
    fn design(self) -> RuleSet {
        vec![
            Rule::from(("t", vec![SyntaxElem::NonTerm("t"), SyntaxElem::NonTerm("c")])),
            Rule::from(("t", vec![SyntaxElem::NonTerm("c")])),
            Rule::from(("c", vec![SyntaxElem::Term("a")])),
            Rule::from(("c", vec![SyntaxElem::Term("b")])),
            Rule::from(("c", vec![SyntaxElem::Term("c")])),
        ]
        .into()
    }
}

fn main() {
    rbuild_dsl!(MyDSL);
}
