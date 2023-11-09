use crate::{DSLDesign, DSLPart};

pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
    Hole(Box<dyn DSLPart>),
}

impl Into<String> for SyntaxElem {
    fn into(self) -> String {
        match self {
            SyntaxElem::Term(s) => format!("\"{}\"", s),
            SyntaxElem::NonTerm(s) => format!("{}", s),
            _ => "".to_string()
        }
    }
}

pub(crate) fn convert(design: impl DSLDesign) -> String {
    design
        .design()
        .into_iter()
        .map(|(left, rights)| {
            let rights = rights.into_iter().map(Into::<String>::into).collect::<Vec<String>>();
            let right = rights.join(" ");
            format!("{}: {}", left, right)
        })
        .collect::<Vec<String>>()
        .join(";\n") + ";"
}
