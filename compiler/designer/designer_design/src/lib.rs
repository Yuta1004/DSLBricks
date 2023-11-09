mod bnf;

use std::any::type_name;
use std::collections::HashMap;

pub use bnf::SyntaxElem;

pub trait DSLDesign {
    fn design(self) -> HashMap<&'static str, Vec<SyntaxElem>>;

    fn name() -> String {
        let full_name = type_name::<Self>();
        full_name.split("::").last().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{DSLDesign, SyntaxElem};
    use crate::bnf::convert;

    struct MyDSL;

    impl DSLDesign for MyDSL {
        fn design(self) -> HashMap<&'static str, Vec<SyntaxElem>> {
            vec![
                ("top", vec![SyntaxElem::Const("aaa")])
            ]
            .into_iter()
            .collect()
        }
    }

    #[test]
    fn det_name() {
        assert_eq!(MyDSL::name(), "MyDSL")
    }

    #[test]
    fn convert_bnf() {
        let _bnf = convert(MyDSL);
    }
}
