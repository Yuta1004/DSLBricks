use std::io;
use std::io::Write;

use clap::Parser;

use processor::lexer::Token;
use processor::parser::syntax::{ASyntax, Syntax};
use processor::DSL;

#[derive(Parser)]
#[command(author, version, about)]
struct InterpreterCLI {}

pub struct Interpreter<A, S, T>(DSL<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static;

impl<A, S, T> From<DSL<A, S, T>> for Interpreter<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static,
{
    fn from(dsl: DSL<A, S, T>) -> Self {
        Interpreter(dsl)
    }
}

impl<A, S, T> Interpreter<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static,
{
    pub fn exec(self) -> anyhow::Result<()> {
        let _ = InterpreterCLI::parse();

        loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            match self.0.process(&line) {
                Ok((_, remain)) => println!("Ok (remain => {:?})\n", remain),
                Err(err) => println!("Error at \"{}\"\n", format!("{}", err).trim()),
            };
        }
    }
}
