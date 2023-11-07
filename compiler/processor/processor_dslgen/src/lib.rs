use lexer::{Lexer, Token};
use parser::syntax::{ASyntax, Syntax};
use parser::Parser;

pub struct DSL<A, S, T>(Lexer<T>, Parser<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token;

impl<A, S, T> DSL<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static,
{
    pub fn gen() -> anyhow::Result<DSL<A, S, T>> {
        let lexer = Lexer::<T>::new()?;
        let parser = Parser::<A, S, T>::new()?;
        Ok(DSL(lexer, parser))
    }

    pub fn process(&self, input: &str) -> anyhow::Result<Box<A>> {
        let DSL(lexer, parser) = self;
        parser.parse(&mut lexer.lex(input))
    }
}
