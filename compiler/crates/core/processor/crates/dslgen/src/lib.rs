#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use lexer::{LexIterator, Lexer, TokenSet};
use parser::syntax::{post, pre};
use parser::Parser;

#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct DSL<PostS, PreS, T>(Lexer<T>, Parser<PostS, PreS, T>)
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T>,
    T: TokenSet;

impl<PostS, PreS, T> DSL<PostS, PreS, T>
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T> + 'static,
    T: TokenSet + 'static,
{
    pub fn gen() -> anyhow::Result<DSL<PostS, PreS, T>> {
        let lexer = Lexer::<T>::new()?;
        let parser = Parser::<PostS, PreS, T>::new()?;
        Ok(DSL(lexer, parser))
    }

    pub fn process<'a>(&self, input: &'a str) -> anyhow::Result<(Box<PostS>, Option<&'a str>)> {
        let DSL(lexer, parser) = self;
        let mut lexer = lexer.lex(input);
        match parser.parse(&mut lexer) {
            Ok(result) => Ok((result, lexer.remain())),
            Err(err) => Err(err.into()),
        }
    }
}
