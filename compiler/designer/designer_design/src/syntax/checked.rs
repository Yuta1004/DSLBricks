use crate::part::DSLPart;

pub type Rule = (&'static str, Vec<SyntaxElem>);
pub type RuleSet = Vec<Rule>;

pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
    Hole(Box<dyn DSLPart>),
}
