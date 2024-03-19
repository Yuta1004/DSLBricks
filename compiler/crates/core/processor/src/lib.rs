pub mod lexer {
    pub use lexer::*;
    pub use lexer_macros as macros;
}
pub mod parser {
    pub use parser::*;
    pub use parser_macros as macros;
}

#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use lexer::{LexIterator, Lexer, TokenSet};
use parser::syntax::{post, pre};
use parser::Parser;

#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct DSL<T, PreS, PostS>(Lexer<T>, Parser<T, PreS, PostS>)
where
    T: TokenSet,
    PreS: pre::Syntax<T, PostS>,
    PostS: post::Syntax<T, PreS>;

impl<T, PreS, PostS> DSL<T, PreS, PostS>
where
    T: TokenSet + 'static,
    PreS: pre::Syntax<T, PostS> + 'static,
    PostS: post::Syntax<T, PreS>,
{
    pub fn gen() -> anyhow::Result<DSL<T, PreS, PostS>> {
        let lexer = Lexer::<T>::new()?;
        let parser = Parser::<T, PreS, PostS>::new()?;
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
