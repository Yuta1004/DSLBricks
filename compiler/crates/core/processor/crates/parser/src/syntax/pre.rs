#[cfg(feature = "with-serde")]
use serde::Serialize;

use lexer::TokenSet;
use utils::macros::cfg_where;

use super::post;
use crate::rule::{Rule, RuleElem, RuleSet};
use crate::ParserImpl;

#[cfg_where(feature = "with-serde", Self: Serialize)]
pub trait Syntax<T, PostS>
where
    Self: Clone + Copy + Sized,
    T: TokenSet,
    PostS: post::Syntax<T, Self>,
{
    type Parser: ParserImpl<T, Self, PostS>;

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
