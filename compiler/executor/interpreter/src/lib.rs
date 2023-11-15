use std::io;
use std::io::Write;

use clap::Parser;

use processor::lexer::TokenSet;
use processor::parser::syntax::{ASyntax, Syntax};
use processor::parser::ParseError;
use processor::DSL;

#[derive(Parser)]
#[command(author, version, about)]
struct InterpreterCLI {}

pub struct Interpreter<A, S, T>(DSL<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: TokenSet + 'static;

impl<A, S, T> From<DSL<A, S, T>> for Interpreter<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: TokenSet + 'static,
{
    fn from(dsl: DSL<A, S, T>) -> Self {
        Interpreter(dsl)
    }
}

impl<A, S, T> Interpreter<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: TokenSet + 'static,
{
    pub fn exec(self) -> anyhow::Result<()> {
        let _ = InterpreterCLI::parse();

        loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            let dsl = &self.0;
            match dsl.process(&line) {
                Ok(_) => println!("Ok\n"),
                Err(err) => {
                    let pos = ParseError::from(err).pos;
                    println!("   {}^ here", " ".repeat(pos.1 as usize));
                    println!("Error at line {}\n", pos.0 + 1);
                }
            };
        }
    }
}
