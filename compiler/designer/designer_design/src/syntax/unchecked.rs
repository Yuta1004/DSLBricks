use crate::part::DSLPart;

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
    Hole(Box<dyn DSLPart>, ()),
}
