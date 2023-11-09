use crate::DSLPart;

pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
    Hole(Box<dyn DSLPart>),
}

impl From<&SyntaxElem> for String {
    fn from(value: &SyntaxElem) -> Self {
        match value {
            SyntaxElem::Term(s) => format!("\"{}\"", s),
            SyntaxElem::NonTerm(s) => format!("{}", s),
            _ => "".to_string()
        }
    }
}

pub(crate) fn convert(design: &Vec<(&'static str, Vec<SyntaxElem>)>) -> String {
    design
        .into_iter()
        .map(|(left, rights)| {
            let rights = rights.into_iter().map(Into::<String>::into).collect::<Vec<String>>();
            let right = rights.join(" ");
            format!("{}: {}", left, right)
        })
        .collect::<Vec<String>>()
        .join(";\n") + ";"
}
