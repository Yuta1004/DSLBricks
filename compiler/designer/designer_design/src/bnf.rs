use crate::DSLDesign;

pub enum SyntaxElem {
    Const(&'static str),
}

pub fn convert(design: impl DSLDesign) -> String {
    let _design = design.design();
    "".to_string()
}
