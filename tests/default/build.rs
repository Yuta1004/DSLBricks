use compiler::build_dsl;
use compiler::designer::design::macros::*;
use compiler::designer::design::syntax::RuleSet;
use compiler::designer::design::DSLGeneratable;

struct MyDSL;

impl DSLGeneratable for MyDSL {
    fn name(&self) -> &'static str {
        "MyDSL"
    }

    fn start(&self) -> &'static str {
        "top"
    }

    fn design(&self) -> RuleSet {
        vec![
            rule!{ hello -> "Hello" names },
            rule!{ names -> names "," name },
            rule!{ names -> name },
            rule!{ name -> "[a-zA-Z]+" },
        ]
        .into()
    }
}

fn main() {
    build_dsl!(MyDSL)
}
