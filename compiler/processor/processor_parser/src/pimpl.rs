mod lr1;

#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use lexer::{LexIterator, TokenSet};
use util_macros::cfg_where;

use super::syntax::{pre, post};
pub use lr1::LR1;

#[cfg_where(feature = "with-serde", Self: Serialize + for<'de> Deserialize<'de>)]
pub trait ParserImpl<PostS, PreS, T>
where
    Self: Sized,
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T>,
    T: TokenSet,
{
    fn setup() -> anyhow::Result<Self>;
    fn parse<'a, 'b>(&self, lexer: &'a mut impl LexIterator<'b, T>) -> anyhow::Result<Box<PostS>>;
}
