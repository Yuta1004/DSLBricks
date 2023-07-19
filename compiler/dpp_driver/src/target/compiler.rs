use std::marker::PhantomData;

use clap::Parser;

use crate::env::Environment;
use crate::target::Target;

#[derive(Parser)]
#[command(author, version, about)]
struct CompilerCLI {}

pub struct Compiler<E: Environment>(PhantomData<E>);

impl<E> Target<E> for Compiler<E>
where
    E: Environment,
{
    fn build(_lang: &()) -> Self {
        Compiler(PhantomData)
    }

    fn exec(self: Self) -> anyhow::Result<()> {
        let _ = CompilerCLI::parse();
        Ok(())
    }
}
