use std::io;
use std::io::Write;
use std::marker::PhantomData;

use clap::Parser;

use crate::env::Environment;
use crate::target::Target;

use langpart::parser::syntax::ASyntax;
use langpart::prelude::*;
use langpart::LangPart;

#[derive(Parser)]
#[command(author, version, about)]
struct InterpreterCLI {}

pub struct Interpreter<E, A, S, T>
where
    E: Environment,
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    env: PhantomData<E>,
    langpart: LangPart<A, S, T>,
}

impl<E, A, S, T> Target<E, A, S, T> for Interpreter<E, A, S, T>
where
    E: Environment,
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static,
{
    fn build(langpart: LangPart<A, S, T>) -> Self {
        Interpreter {
            env: PhantomData,
            langpart,
        }
    }

    fn exec(self) -> anyhow::Result<()> {
        let _ = InterpreterCLI::parse();

        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            match self.langpart.process(&line) {
                Ok(_) => println!("Ok\n"),
                Err(_) => println!("Error!\n"),
            };
        }
    }
}
