use std::fmt::Debug;

pub trait DSLDesign
where
    Self: Debug,
{
    fn design(&self) -> Vec<Box<dyn DSLPart>>;
}

pub trait DSLPart
where
    Self: Debug,
{
    fn syntax(&self) -> Vec<SyntaxElem>;

    fn tokens(&self) -> Vec<&'static str> {
        self.syntax()
            .into_iter()
            .map(|elem| {
                match elem {
                    SyntaxElem::Const(s) => vec![s],
                }
            })
            .flatten()
            .collect()
    }
}

pub enum SyntaxElem {
    Const(&'static str),
}

#[cfg(test)]
mod test {
    use crate::{DSLDesign, DSLPart, SyntaxElem};

    #[derive(Debug)]
    struct MyDSL;

    impl DSLDesign for MyDSL {
        fn design(&self) -> Vec<Box<dyn DSLPart>> {
            vec![Box::new(Function)]
        }
    }

    #[derive(Debug)]
    struct Function;

    impl DSLPart for Function {
        fn syntax(&self) -> Vec<SyntaxElem> {
            vec![
                SyntaxElem::Const("func"),
                SyntaxElem::Const("("),
                SyntaxElem::Const(")"),
                SyntaxElem::Const("{"),
                SyntaxElem::Const("}"),
            ]
        }
    }

    #[test]
    fn tokens() {
        let tokens: Vec<&str> = MyDSL.design()
            .into_iter()
            .map(|part| part.tokens())
            .flatten()
            .collect();
        assert_eq!(tokens, vec!["func", "(", ")", "{", "}"])
    }
}
