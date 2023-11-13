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
    syntax: checked::RuleSet,
}

impl DSLDesign {
    pub fn token_defs(&self) -> Vec<&'static str> {
        self.syntax.token_defs()
    }

    pub fn syntax_defs(&self) -> Vec<String> {
        self.syntax.syntax_defs()
    }

    pub fn bnf(&self) -> String {
        bnf::gen(&self.syntax)
    }
}

impl<T: DSLGeneratable> From<T> for DSLDesign {
    fn from(def: T) -> Self {
        let full_name = type_name::<T>();
        let name = full_name.split("::").last().unwrap().to_string();

        let ruleset = syntax::check(def.design()).unwrap();

        DSLDesign {
            name,
            syntax: ruleset,
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
