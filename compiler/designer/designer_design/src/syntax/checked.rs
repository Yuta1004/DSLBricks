use crate::syntax::unchecked;

#[derive(Debug)]
pub struct Rule {
    pub(crate) name: String,
    pub(crate) left: &'static str,
    pub(crate) rights: Vec<SyntaxElem>,
}

impl<T> From<(T, &'static str, Vec<SyntaxElem>)> for Rule
where
    T: Into<String>,
{
    fn from((name, left, rights): (T, &'static str, Vec<SyntaxElem>)) -> Self {
        Rule {
            name: name.into(),
            left,
            rights
        }
    }
}

#[derive(Debug)]
pub struct RuleSet(pub(crate) Vec<Rule>);

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        RuleSet(rules)
    }
}

#[derive(Debug)]
pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
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
