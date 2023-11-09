use crate::syntax::unchecked;

pub type Rule = (&'static str, Vec<SyntaxElem>);
pub type RuleSet = Vec<Rule>;

#[derive(Debug)]
pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
    Hole(&'static str),
}

impl From<unchecked::SyntaxElem> for SyntaxElem {
    fn from(selem: unchecked::SyntaxElem) -> Self {
        match selem {
            unchecked::SyntaxElem::Term(s) => SyntaxElem::Term(s),
            unchecked::SyntaxElem::NonTerm(s) => SyntaxElem::NonTerm(s),
            _ => unimplemented!(),
        }
    }
}
