use compiler::build_dsl;
use compiler::designer::design::syntax::{Rule, RuleSet, SyntaxElem};
use compiler::designer::design::DSLGeneratable;

#[derive(Debug)]
struct MyDSL;

impl DSLGeneratable for MyDSL {
    fn design(self) -> RuleSet {
        vec![
            Rule::from(("expr", vec![SyntaxElem::NonTerm("expr"), SyntaxElem::Term(r"\+"), SyntaxElem::NonTerm("term")])),
            Rule::from(("expr", vec![SyntaxElem::NonTerm("expr"), SyntaxElem::Term(r"-"), SyntaxElem::NonTerm("term")])),
            Rule::from(("expr", vec![SyntaxElem::NonTerm("term")])),
            Rule::from(("term", vec![SyntaxElem::NonTerm("term"), SyntaxElem::Term(r"\*"), SyntaxElem::NonTerm("fact")])),
            Rule::from(("term", vec![SyntaxElem::NonTerm("term"), SyntaxElem::Term(r"/"), SyntaxElem::NonTerm("fact")])),
            Rule::from(("term", vec![SyntaxElem::NonTerm("fact")])),
            Rule::from(("fact", vec![SyntaxElem::Term(r"\("), SyntaxElem::NonTerm("expr"), SyntaxElem::Term(r"\)")])),
            Rule::from(("fact", vec![SyntaxElem::Term(r"(0|[1-9][0-9]*)")])),
        ]
        .into()
    }
}

fn main() {
    build_dsl!(MyDSL);
}
