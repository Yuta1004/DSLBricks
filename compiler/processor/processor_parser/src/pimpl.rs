mod lr1;

use lexer::Token;

use super::syntax::{ASyntax, Syntax};
pub use lr1::LR1;

pub trait ParserImpl<A, S, T>
where
    Self: Sized,
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    fn setup() -> anyhow::Result<Self>;
    fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl Iterator<Item = (&'b str, T)>,
    ) -> anyhow::Result<Box<A>>;
}
