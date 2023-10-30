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

impl<A, S, T> Interpreter<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static,
{
    pub fn new(dsl: DSL<A, S, T>) -> Self {
        Interpreter(dsl)
    }

    pub fn exec(self: Self) -> anyhow::Result<()> {
        let _ = InterpreterCLI::parse();

        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            match self.0.process(&line) {
                Ok(_) => println!("Ok\n"),
                Err(_) => println!("Error!\n"),
            };
        }
    }
}
