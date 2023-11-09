mod bnf;

use std::any::type_name;

pub use bnf::SyntaxElem;

pub trait DSLDesign
where
    Self: Sized + Default,
{
    fn design(self) -> Vec<(&'static str, Vec<SyntaxElem>)>;

    fn name() -> String {
        let full_name = type_name::<Self>();
        full_name.split("::").last().unwrap().to_string()
    }

    fn bnf(self) -> String {
        bnf::convert(self)
    }
}

pub trait DSLPart {
    fn design(self) -> Vec<(&'static str, Vec<SyntaxElem>)>;
}

#[cfg(test)]
mod test {
    use crate::{DSLDesign, SyntaxElem};

    #[derive(Default)]
    struct MyDSL;

    impl DSLDesign for MyDSL {
        fn design(self) -> Vec<(&'static str, Vec<SyntaxElem>)> {
            vec![
                ("top", vec![SyntaxElem::NonTerm("top"), SyntaxElem::Term("A")]),
                ("top", vec![SyntaxElem::Term("A")]),
            ]
        }
    }

    #[test]
    fn det_name() {
        assert_eq!(MyDSL::name(), "MyDSL")
    }

    #[test]
    fn convert_bnf() {
        let except = vec![
            "top: top \"A\";",
            "top: \"A\";",
        ];

        let result = MyDSL
            .bnf()
            .split("\n")
            .into_iter()
            .zip(except.into_iter())
            .all(|(line, except)| line == except);
        assert!(result)
    }
}
