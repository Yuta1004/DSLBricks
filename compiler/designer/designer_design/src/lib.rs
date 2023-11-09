mod bnf;

use std::any::type_name;

pub use bnf::SyntaxElem;

pub trait DSLGeneratable
where
    Self: Sized,
{
    fn design(self) -> Vec<(&'static str, Vec<SyntaxElem>)>;
}

pub struct DSLDesign {
    pub name: String,
    syntax: Vec<(&'static str, Vec<SyntaxElem>)>,
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

        let syntax = def.design();

        DSLDesign { name, syntax }
    }
}

pub trait DSLPart {
    fn design(self) -> Vec<(&'static str, Vec<SyntaxElem>)>;
}

#[cfg(test)]
mod test {
    use crate::{DSLDesign, DSLGeneratable, SyntaxElem};

    #[derive(Default)]
    struct MyDSL;

    impl DSLGeneratable for MyDSL {
        fn design(self) -> Vec<(&'static str, Vec<SyntaxElem>)> {
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
