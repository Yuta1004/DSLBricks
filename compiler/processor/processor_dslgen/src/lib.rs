#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use lexer::{LexIterator, Lexer, TokenSet};
use parser::syntax::{ASyntax, Syntax};
use parser::Parser;

#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct DSL<A, S, T>(Lexer<T>, Parser<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: TokenSet;

impl<A, S, T> DSL<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: TokenSet + 'static,
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
