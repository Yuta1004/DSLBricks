mod pimpl;
pub mod kind {
    pub use super::pimpl::LR1;
}
pub mod rule;
pub mod syntax;

use std::marker::PhantomData;

use thiserror::Error;

use pimpl::ParserImpl;
use syntax::{ASyntax, Syntax};

use crate::lexer::Token;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse!! (unknown)")]
    Unknown,
}

pub struct Parser<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    syntax: PhantomData<S>,
    p_impl: S::Parser,
}

#[allow(clippy::new_without_default)]
impl<A, S, T> Parser<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    pub fn new() -> Parser<A, S, T> {
        Parser {
            syntax: PhantomData,
            p_impl: S::Parser::setup(),
        }
    }

    pub fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl Iterator<Item = (&'b str, T)>,
    ) -> anyhow::Result<Box<A>> {
        self.p_impl.parse(lexer)
    }
}
