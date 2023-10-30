use clap::Parser;

use processor::lexer::Token;
use processor::parser::{ASyntax, Syntax};
use processor::DSL;

#[derive(Parser)]
#[command(author, version, about)]
struct CompilerCLI {}

pub struct Compiler<A, S, T>(DSL<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static;

impl<A, S, T> Compiler<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    pub fn new(langpart: DSL<A, S, T>) -> Self {
        Compiler(langpart)
    }

    pub fn exec(self) -> anyhow::Result<()> {
        let _ = CompilerCLI::parse();
        unimplemented!();
    }
}
