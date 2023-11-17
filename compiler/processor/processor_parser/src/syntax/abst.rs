#[cfg(feature = "with-serde")]
use serde::Serialize;
use thiserror::Error;

use lexer::TokenSet;
use util_macros::cfg_where;

use super::Syntax;

#[derive(Debug, Error)]
pub enum ASyntaxError {
    #[error("Mapping process is not implemented")]
    NotImplemented,
}

#[cfg_where(feature = "with-serde", Self: Serialize)]
pub trait ASyntax<S, T>
where
    Self: Sized,
    S: Syntax<Self, T>,
    T: TokenSet,
{
    #[allow(unused_variables)]
    fn mapping(
        syntax: S,
        tokens: Vec<(Option<Box<Self>>, Option<&str>)>,
    ) -> anyhow::Result<Box<Self>>;
}
