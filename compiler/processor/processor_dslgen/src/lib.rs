use serde::{Serialize, Deserialize};

use lexer::{Lexer, Token, LexIterator};
use parser::syntax::{ASyntax, Syntax};
use parser::Parser;

#[derive(Serialize, Deserialize)]
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

    pub fn process<'a>(&self, input: &'a str) -> anyhow::Result<(Box<A>, Option<&'a str>)> {
        let DSL(lexer, parser) = self;
        let mut lexer = lexer.lex(input);
        match parser.parse(&mut lexer) {
            Ok(result) => Ok((result, lexer.remain())),
            Err(err) => Err(err),
        }
    }
}
