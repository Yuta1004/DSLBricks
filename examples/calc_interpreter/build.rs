use compiler::build_dsl;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
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
            rule! { expr -> expr r"\+" term },
            rule! { expr -> expr r"-" term },
            rule! { expr -> term },
            rule! { term -> term r"\*" fact },
            rule! { term -> term r"/" fact },
            rule! { term -> fact },
            rule! { fact -> r"\(" expr r"\)" },
            rule! { fact -> [{ Box::new(CalcUnit) }] },
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
            rule! { unit -> "(0|[1-9][0-9]*)" },
            rule! { unit -> "[a-z](_|[a-z0-9])*" }
        ]
        .into()
    }
}

fn main() {
    build_dsl!(Expression);
}
