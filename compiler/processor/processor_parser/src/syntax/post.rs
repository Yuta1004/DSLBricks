#[cfg(feature = "with-serde")]
use serde::Serialize;
use thiserror::Error;

use lexer::TokenSet;
use util_macros::cfg_where;

use super::pre;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Mapping process is not implemented")]
    NotImplemented,
}

#[cfg_where(feature = "with-serde", Self: Serialize)]
pub trait Syntax<PreS, T>
where
    Self: Sized,
    PreS: pre::Syntax<Self, T>,
    T: TokenSet,
{
    #[allow(unused_variables)]
    fn mapping(
        syntax: PreS,
        tokens: Vec<(Option<Box<Self>>, Option<&str>)>,
    ) -> anyhow::Result<Box<Self>>;
}
