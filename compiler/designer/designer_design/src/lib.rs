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
    pub tokens: Vec<&'static str>,
    syntax: checked::RuleSet,
}

impl DSLDesign {
    pub fn bnf(&self) -> String {
        bnf::convert(&self.syntax)
    }
}

impl<T: DSLGeneratable> From<T> for DSLDesign {
    fn from(def: T) -> Self {
        let full_name = type_name::<T>();
        let name = full_name.split("::").last().unwrap().to_string();

        let (tokens, syntax) = syntax::check(def.design());

        DSLDesign {
            name,
            tokens,
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
