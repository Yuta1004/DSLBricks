mod pimpl;
pub mod kind {
    pub use crate::pimpl::*;
}
pub mod rule;
pub mod syntax;

use std::fmt::Display;
use std::marker::PhantomData;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use lexer::{LexIterator, TokenSet};

use pimpl::ParserImpl;
use syntax::{post, pre};

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
pub struct Parser<T, PreS, PostS>
where
    T: TokenSet,
    PreS: pre::Syntax<T, PostS>,
    PostS: post::Syntax<T, PreS>,
{
    // PhantomData
    syntax: PhantomData<PreS>,

    // Parser Body
    p_impl: PreS::Parser,
}

#[allow(clippy::new_without_default)]
impl<T, PreS, PostS> Parser<T, PreS, PostS>
where
    T: TokenSet,
    PreS: pre::Syntax<T, PostS>,
    PostS: post::Syntax<T, PreS>,
{
    pub fn new() -> anyhow::Result<Parser<T, PreS, PostS>> {
        Ok(Parser {
            syntax: PhantomData,
            p_impl: PreS::Parser::setup()?,
        })
    }

    pub fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl LexIterator<'b, T>,
    ) -> anyhow::Result<Box<PostS>, ParseError> {
        self.p_impl.parse(lexer)
    }
}
