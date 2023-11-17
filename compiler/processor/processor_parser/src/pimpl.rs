mod lr1;

#[cfg(feature = "with-serde")]
use serde::Serialize;
use serde::Deserialize;

use lexer::{TokenSet, LexIterator};
use util_macros::cfg_where;

use super::syntax::{ASyntax, Syntax};
pub use lr1::LR1;

#[cfg_where(feature = "with-serde", Serialize)]
pub trait ParserImpl<A, S, T>
where
    Self: Sized + for<'de> Deserialize<'de>,
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: TokenSet,
{
    fn setup() -> anyhow::Result<Self>;
    fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl LexIterator<'b, T>,
    ) -> anyhow::Result<Box<A>>;
}
