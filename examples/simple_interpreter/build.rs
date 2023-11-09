use blockdsl::designer::codegen::rbuild_dsl;
use blockdsl::designer::design::syntax::RuleSet;
use blockdsl::designer::design::DSLGeneratable;

struct MyDSL;

impl DSLGeneratable for MyDSL {
    fn design(self) -> RuleSet {
        RuleSet::default()
    }
}

fn main() {
    rbuild_dsl!(MyDSL);
}
