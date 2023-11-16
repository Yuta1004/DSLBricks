use compiler::build_dsl;
use compiler::designer::design::syntax::{Rule, RuleSet, SyntaxElem};
use compiler::designer::design::DSLGeneratable;

#[derive(Debug)]
struct Expression;

impl DSLGeneratable for Expression {
    fn name(&self) -> &'static str {
        "Expression"
    }

    fn start(&self) -> &'static str {
        "expr"
    }

    fn design(&self) -> RuleSet {
        vec![
            Rule::from(("expr", vec![SyntaxElem::NonTerm("expr"), SyntaxElem::Term(r"\+"), SyntaxElem::NonTerm("term")])),
            Rule::from(("expr", vec![SyntaxElem::NonTerm("expr"), SyntaxElem::Term(r"-"), SyntaxElem::NonTerm("term")])),
            Rule::from(("expr", vec![SyntaxElem::NonTerm("term")])),
            Rule::from(("term", vec![SyntaxElem::NonTerm("term"), SyntaxElem::Term(r"\*"), SyntaxElem::NonTerm("fact")])),
            Rule::from(("term", vec![SyntaxElem::NonTerm("term"), SyntaxElem::Term(r"/"), SyntaxElem::NonTerm("fact")])),
            Rule::from(("term", vec![SyntaxElem::NonTerm("fact")])),
            Rule::from(("fact", vec![SyntaxElem::Term(r"\("), SyntaxElem::NonTerm("expr"), SyntaxElem::Term(r"\)")])),
            Rule::from(("fact", vec![SyntaxElem::Hole(Box::new(CalcUnit))])),
        ]
        .into()
    }
}

#[derive(Debug)]
struct CalcUnit;

impl DSLGeneratable for CalcUnit {
    fn name(&self) -> &'static str {
        "CalcUnit"
    }

    fn start(&self) -> &'static str {
        "unit"
    }

    fn design(&self) -> RuleSet {
        vec![
            Rule::from(("unit", vec![SyntaxElem::Term(r"(0|[1-9][0-9]*)")])),
            Rule::from(("unit", vec![SyntaxElem::Term(r"[a-z](_|[a-z0-9])+")])),
        ]
        .into()
    }
}

fn main() {
    build_dsl!(Expression);
}
