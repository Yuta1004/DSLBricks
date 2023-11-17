mod pimpl;
pub mod rule;
pub mod syntax;

use std::fmt::Display;
use std::marker::PhantomData;

use anyhow::Error;
use serde::{Serialize, Deserialize};
use serde_json;
use thiserror::Error;

use lexer::{TokenSet, LexIterator};

use pimpl::ParserImpl;
pub use pimpl::LR1;
use syntax::{ASyntax, Syntax};

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct ParseError {
    pub pos: (u32, u32),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl From<(u32, u32)> for ParseError {
    fn from(pos: (u32, u32)) -> Self {
        ParseError { pos }
    }
}

impl From<Error> for ParseError {
    fn from(err: Error) -> Self {
        serde_json::from_str(&err.to_string()).unwrap()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Parser<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: TokenSet,
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
    T: TokenSet,
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
