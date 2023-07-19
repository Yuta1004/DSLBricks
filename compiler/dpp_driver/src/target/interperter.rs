use std::marker::PhantomData;

use clap::Parser;

use crate::env::Environment;
use crate::target::Target;

#[derive(Parser)]
#[command(author, version, about)]
struct InterpreterCLI {}

pub struct Interpreter<E: Environment>(PhantomData<E>);

impl<E> Target<E> for Interpreter<E>
where
    E: Environment,
{
    fn build(_lang: &()) -> Self {
        Interpreter(PhantomData)
    }

    fn exec(self: Self) -> anyhow::Result<()> {
        let _ = InterpreterCLI::parse();
        Ok(())
    }
}
