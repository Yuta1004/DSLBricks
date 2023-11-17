mod lr1;

#[cfg(feature = "with-serde")]
use serde::{Serialize, Deserialize};

use lexer::{TokenSet, LexIterator};
use util_macros::cfg_where;

use super::syntax::{ASyntax, Syntax};
pub use lr1::LR1;

#[cfg_where(feature = "with-serde", Self: Serialize + for<'de> Deserialize<'de>)]
pub trait ParserImpl<A, S, T>
where
    Self: Sized,
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
