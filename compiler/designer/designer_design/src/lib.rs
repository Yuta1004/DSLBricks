mod bnf;
pub mod syntax;

use std::any::type_name;
use std::fmt::Debug;

use syntax::{checked, unchecked};

pub trait DSLGeneratable
where
    Self: Debug,
{
    fn design(self) -> unchecked::RuleSet;
}

pub struct DSLDesign {
    pub name: String,
    pub token_defs: Vec<&'static str>,
    pub syntax_defs: Vec<String>,
    syntax: checked::RuleSet,
}

impl DSLDesign {
    pub fn bnf(&self) -> String {
        bnf::gen(&self.syntax)
    }
}

impl<T: DSLGeneratable> From<T> for DSLDesign {
    fn from(def: T) -> Self {
        let full_name = type_name::<T>();
        let name = full_name.split("::").last().unwrap().to_string();

        let (token_defs, syntax_defs, syntax) = syntax::check(def.design());

        DSLDesign {
            name,
            token_defs,
            syntax_defs,
            syntax,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::RuleSet;
    use crate::{DSLDesign, DSLGeneratable};

    #[derive(Debug)]
    struct MyDSL;

    impl DSLGeneratable for MyDSL {
        fn design(self) -> RuleSet {
            vec![].into()
        }
    }

    #[test]
    fn name() {
        assert_eq!(DSLDesign::from(MyDSL).name, "MyDSL")
    }
}
