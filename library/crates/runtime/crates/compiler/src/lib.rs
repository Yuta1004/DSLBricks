use clap::Parser;

use compiler::processor::lexer::TokenSet;
use compiler::processor::parser::syntax::{post, pre};
use compiler::processor::DSL;

#[derive(Parser)]
#[command(author, version, about)]
struct CompilerCLI {}

pub struct Compiler<T, PreS, PostS>(DSL<T, PreS, PostS>)
where
    T: TokenSet + 'static,
    PreS: pre::Syntax<T, PostS> + 'static,
    PostS: post::Syntax<T, PreS>;

impl<T, PreS, PostS> From<DSL<T, PreS, PostS>> for Compiler<T, PreS, PostS>
where
    T: TokenSet + 'static,
    PreS: pre::Syntax<T, PostS> + 'static,
    PostS: post::Syntax<T, PreS>,

{
    fn from(dsl: DSL<T, PreS, PostS>) -> Self {
        Compiler(dsl)
    }
}

impl<T, PreS, PostS> Compiler<T, PreS, PostS>
where
    T: TokenSet,
    PreS: pre::Syntax<T, PostS>,
    PostS: post::Syntax<T, PreS>,
{
    pub fn exec(self) -> anyhow::Result<()> {
        let _ = CompilerCLI::parse();
        unimplemented!();
    }
}
