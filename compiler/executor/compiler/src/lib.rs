use clap::Parser;

use processor::lexer::TokenSet;
use processor::parser::syntax::{pre, post};
use processor::DSL;

#[derive(Parser)]
#[command(author, version, about)]
struct CompilerCLI {}

pub struct Compiler<PostS, PreS, T>(DSL<PostS, PreS, T>)
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T> + 'static,
    T: TokenSet + 'static;

impl<PostS, PreS, T> From<DSL<PostS, PreS, T>> for Compiler<PostS, PreS, T>
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T> + 'static,
    T: TokenSet + 'static,
{
    fn from(dsl: DSL<PostS, PreS, T>) -> Self {
        Compiler(dsl)
    }
}

impl<PostS, PreS, T> Compiler<PostS, PreS, T>
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T>,
    T: TokenSet,
{
    pub fn exec(self) -> anyhow::Result<()> {
        let _ = CompilerCLI::parse();
        unimplemented!();
    }
}
