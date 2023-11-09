pub(crate) mod checked;
pub(crate) mod unchecked;

pub use unchecked::{SyntaxElem, Rule, RuleSet};

pub fn check(uc_ruleset: unchecked::RuleSet) -> checked::RuleSet {
    vec![]
}
