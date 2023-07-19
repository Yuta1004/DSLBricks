use std::marker::PhantomData;

use clap::Parser;

use crate::env::Environment;
use crate::target::Target;

use langpart::parser::syntax::ASyntax;
use langpart::prelude::*;
use langpart::LangPart;

#[derive(Parser)]
#[command(author, version, about)]
struct CompilerCLI {}

pub struct Compiler<E, A, S, T>
where
    E: Environment,
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    env: PhantomData<E>,
    _langpart: LangPart<A, S, T>,
}

impl<E, A, S, T> Target<E, A, S, T> for Compiler<E, A, S, T>
where
    E: Environment,
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    fn build(langpart: LangPart<A, S, T>) -> Self {
        Compiler {
            env: PhantomData,
            _langpart: langpart,
        }
    }

    fn exec(self: Self) -> anyhow::Result<()> {
        let _ = CompilerCLI::parse();
        Ok(())
    }
}
