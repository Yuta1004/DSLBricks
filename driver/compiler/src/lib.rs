use clap::Parser;

use processor::parser::syntax::ASyntax;
use processor::prelude::*;
use processor::LangPart;

#[derive(Parser)]
#[command(author, version, about)]
struct CompilerCLI {}

pub struct Compiler<A, S, T> (LangPart<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token;

impl<A, S, T> Compiler<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    pub fn new(langpart: LangPart<A, S, T>) -> Self {
        Compiler(langpart)
    }

    pub fn exec(self) -> anyhow::Result<()> {
        let _ = CompilerCLI::parse();
        unimplemented!();
    }
}
