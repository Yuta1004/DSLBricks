mod lr1;

use serde::{Serialize, Deserialize};

use lexer::{Token, LexIterator};

use super::syntax::{ASyntax, Syntax};
pub use lr1::LR1;

pub trait ParserImpl<A, S, T>
where
    Self: Sized + Serialize + for<'de> Deserialize<'de>,
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    fn setup() -> anyhow::Result<Self>;
    fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl LexIterator<'b, T>,
    ) -> anyhow::Result<Box<A>>;
}
