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
        let rules = vec![
            rule!{ top -> "Hello" names },
            rule!{ names -> names "," name },
            rule!{ names -> name },
            rule!{ name -> "[a-zA-Z]+" },
        ];
        ("MyDSL", rules).into()
    }
}

fn main() {
    build_dsl!(MyDSL)
}
