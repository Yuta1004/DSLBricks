#[cfg(feature = "with-serde")]
use serde::Serialize;

use lexer::TokenSet;
use utils::macros::cfg_where;

use super::pre;

#[cfg_where(feature = "with-serde", Self: Serialize)]
pub trait Syntax<T, PreS>
where
    Self: Sized,
    T: TokenSet,
    PreS: pre::Syntax<T, Self>,
{
    #[allow(unused_variables)]
    fn mapping(
        syntax: PreS,
        tokens: Vec<(Option<Box<Self>>, Option<&str>)>,
    ) -> anyhow::Result<Box<Self>>;
}
