mod lr1;

#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use lexer::{LexIterator, TokenSet};
use utils::macros::cfg_where;

use super::syntax::{post, pre};
use super::ParseError;
pub use lr1::LR1;

#[cfg_where(feature = "with-serde", Self: Serialize + for<'de> Deserialize<'de>)]
pub trait ParserImpl<T, PreS, PostS>
where
    Self: Sized,
    T: TokenSet,
    PreS: pre::Syntax<T, PostS>,
    PostS: post::Syntax<T, PreS>,
{
    fn setup() -> anyhow::Result<Self>;
    fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl LexIterator<'b, T>,
    ) -> anyhow::Result<Box<PostS>, ParseError>;
}
