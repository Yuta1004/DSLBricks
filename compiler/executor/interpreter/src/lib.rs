use std::io;
use std::io::Write;

use clap::Parser;

use processor::lexer::TokenSet;
use processor::parser::syntax::{pre, post};
use processor::parser::ParseError;
use processor::DSL;

#[derive(Parser)]
#[command(author, version, about)]
struct InterpreterCLI {}

pub struct Interpreter<PostS, PreS, T>(DSL<PostS, PreS, T>)
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T> + 'static,
    T: TokenSet + 'static;

impl<PostS, PreS, T> From<DSL<PostS, PreS, T>> for Interpreter<PostS, PreS, T>
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T> + 'static,
    T: TokenSet + 'static,
{
    fn from(dsl: DSL<PostS, PreS, T>) -> Self {
        Interpreter(dsl)
    }
}

impl<PostS, PreS, T> Interpreter<PostS, PreS, T>
where
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T> + 'static,
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
