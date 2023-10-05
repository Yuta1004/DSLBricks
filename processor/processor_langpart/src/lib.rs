use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use crate::parser::syntax::{ASyntax, Syntax};

pub mod prelude;
pub mod lexer;
pub mod parser;

pub struct LangPart<A, S, T>(Lexer<T>, Parser<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token;

impl<A, S, T> LangPart<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static,
{
    pub fn gen() -> anyhow::Result<LangPart<A, S, T>> {
        let lexer = Lexer::<T>::new()?;
        let parser = Parser::<A, S, T>::new();
        Ok(LangPart(lexer, parser))
    }

    pub fn process(&self, input: &str) -> anyhow::Result<Box<A>> {
        let LangPart(lexer, parser) = self;
        parser.parse(&mut lexer.lex(input))
    }
}
