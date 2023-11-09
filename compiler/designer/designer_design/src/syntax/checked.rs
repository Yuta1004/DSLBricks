use crate::syntax::unchecked;

#[derive(Debug)]
pub struct Rule {
    pub(crate) left: &'static str,
    pub(crate) rights: Vec<SyntaxElem>,
}

impl From<(&'static str, Vec<SyntaxElem>)> for Rule {
    fn from((left, rights): (&'static str, Vec<SyntaxElem>)) -> Self {
        Rule { left, rights }
    }
}

#[derive(Debug, Default)]
pub struct RuleSet (
    pub(crate) Vec<Rule>,
);

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        RuleSet(rules)
    }
}

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
