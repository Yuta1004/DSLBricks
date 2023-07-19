use thiserror::Error;

use super::Syntax;

use crate::lexer::Token;

#[derive(Debug, Error)]
pub enum ASyntaxError {
    #[error("Mapping process is not implemented")]
    NotImplemented,
}

pub trait ASyntax<S, T>
where
    S: Syntax<Self, T>,
    T: Token,
    Self: Sized,
{
    #[allow(unused_variables)]
    fn mapping(
        syntax: S,
        tokens: Vec<(Option<Box<Self>>, Option<&str>)>,
    ) -> anyhow::Result<Box<Self>>;
}
