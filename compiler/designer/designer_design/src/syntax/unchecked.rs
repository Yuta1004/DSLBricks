pub type Rule = (&'static str, Vec<SyntaxElem>);
pub type RuleSet = Vec<Rule>;

#[derive(Debug)]
pub enum SyntaxElem {
    Term(&'static str),
    NonTerm(&'static str),
    Hole(&'static str, ()),
}
