use strum::IntoEnumIterator;

use super::abst::ASyntax;

use crate::lexer::Token;
use crate::parser::rule::{Rule, RuleElem, RuleSet};
use crate::parser::ParserImpl;

pub trait Syntax<A, T>
where
    A: ASyntax<Self, T>,
    T: Token,
    Self: IntoEnumIterator + Clone + Copy + Sized,
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
