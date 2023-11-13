mod pimpl;
pub mod rule;
pub mod syntax;
pub mod prelude;

use std::fmt::Display;
use std::marker::PhantomData;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use lexer::{Token, LexIterator};

use pimpl::ParserImpl;
pub use pimpl::LR1;
use syntax::{ASyntax, Syntax};

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ParseError {
    fn from(remain: String) -> Self {
        ParseError(remain)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    // PhantomData
    syntax: PhantomData<S>,

    // Parser Body
    p_impl: S::Parser,
}

#[allow(clippy::new_without_default)]
impl<A, S, T> Parser<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    pub fn new() -> anyhow::Result<Parser<A, S, T>> {
        Ok(Parser {
            syntax: PhantomData,
            p_impl: S::Parser::setup()?,
        })
    }

    pub fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl LexIterator<'b, T>,
    ) -> anyhow::Result<Box<A>> {
        self.p_impl.parse(lexer)
    }
}
