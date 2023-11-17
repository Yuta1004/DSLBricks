#[cfg(feature = "with-serde")]
use serde::Serialize;

use lexer::TokenSet;
use util_macros::cfg_where;

use super::abst::ASyntax;
use crate::rule::{Rule, RuleElem, RuleSet};
use crate::ParserImpl;

#[cfg_where(feature = "with-serde", Serialize)]
pub trait Syntax<A, T>
where
    Self: Clone + Copy + Sized,
    A: ASyntax<Self, T>,
    T: TokenSet,
{
    type Parser: ParserImpl<A, Self, T>;

    // for Enum
    fn iter() -> Box<dyn Iterator<Item = Self>>;

    fn syntax() -> RuleSet<T> {
        let rules: Vec<Rule<T>> = Self::iter().map(|rtoken| rtoken.to_rule()).collect();
        let start = if let RuleElem::NonTerm(s) = &rules[0].left {
            s.clone()
        } else {
            "".to_string()
        };

        RuleSet::from((start, rules))
    }

    // for Variants
    fn to_rule(&self) -> Rule<T>;
}
