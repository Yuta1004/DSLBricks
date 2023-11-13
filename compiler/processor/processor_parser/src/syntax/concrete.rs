use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;

use lexer::Token;

use super::abst::ASyntax;
use crate::rule::{Rule, RuleElem, RuleSet};
use crate::ParserImpl;

pub trait Syntax<A, T>
where
    Self: IntoEnumIterator + Clone + Copy + Sized + Serialize + for<'de> Deserialize<'de>,
    A: ASyntax<Self, T>,
    T: Token,
{
    type Parser: ParserImpl<A, Self, T>;

    fn to_rule(&self) -> Rule<T>;

    fn syntax() -> RuleSet<T> {
        let rules: Vec<Rule<T>> = Self::iter().map(|rtoken| rtoken.to_rule()).collect();
        let start = if let RuleElem::NonTerm(s) = &rules[0].left {
            s.clone()
        } else {
            "".to_string()
        };

        RuleSet::from((start, rules))
    }
}
