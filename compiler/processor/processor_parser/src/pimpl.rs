mod lr1;

use lexer::Token;

pub use lr1::LR1;
use super::syntax::{ASyntax, Syntax};

pub trait ParserImpl<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    fn setup() -> Self;
    fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl Iterator<Item = (&'b str, T)>,
    ) -> anyhow::Result<Box<A>>;
}
