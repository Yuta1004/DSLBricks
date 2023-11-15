use clap::Parser;

use processor::lexer::TokenSet;
use processor::parser::syntax::{ASyntax, Syntax};
use processor::DSL;

#[derive(Parser)]
#[command(author, version, about)]
struct CompilerCLI {}

pub struct Compiler<A, S, T>(DSL<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: TokenSet + 'static;

impl<A, S, T> From<DSL<A, S, T>> for Compiler<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: TokenSet + 'static,
{
    fn from(dsl: DSL<A, S, T>) -> Self {
        Compiler(dsl)
    }
}

impl<A, S, T> Compiler<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: TokenSet,
{
    pub fn exec(self) -> anyhow::Result<()> {
        let _ = CompilerCLI::parse();
        unimplemented!();
    }
}
