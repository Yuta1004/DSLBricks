mod bnf;
mod part;
pub mod syntax;

use std::any::type_name;

use syntax::{checked, unchecked};

pub trait DSLGeneratable
where
    Self: Sized,
{
    fn design(self) -> unchecked::RuleSet;
}

pub struct DSLDesign {
    pub name: String,
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

        let syntax = syntax::check(def.design());

        DSLDesign { name, syntax }
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::{SyntaxElem, RuleSet};
    use crate::{DSLDesign, DSLGeneratable};

    #[derive(Default)]
    struct MyDSL;

    impl DSLGeneratable for MyDSL {
        fn design(self) -> RuleSet {
            vec![
                ("top", vec![SyntaxElem::NonTerm("top"), SyntaxElem::Term("A")]),
                ("top", vec![SyntaxElem::Term("A")]),
            ]
        }
    }

    #[test]
    fn det_name() {
        assert_eq!(DSLDesign::from(MyDSL).name, "MyDSL")
    }

    #[test]
    fn convert_bnf() {
        let except = vec![
            "top: top \"A\";",
            "top: \"A\";",
        ];

        let result = DSLDesign::from(MyDSL)
            .bnf()
            .split("\n")
            .into_iter()
            .zip(except.into_iter())
            .all(|(line, except)| line == except);
        assert!(result)
    }
}
