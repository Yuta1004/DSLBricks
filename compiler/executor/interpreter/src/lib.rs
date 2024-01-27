use std::io;
use std::io::Write;

use clap::Parser;
use crossterm::cursor;
use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyModifiers};
use crossterm::style;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
        let dsl = &self.0;

        println!("Type 'Ctrl-D' to evaluate inputs, 'Ctrl-C' to exit.");

        loop {
            let input = read_lines()?;
            match dsl.process(&input) {
                Ok(_) => println!("Ok\n"),
                Err(err) => {
                    let pos = ParseError::from(err).pos;
                    print_pretty_error(&input, pos);
                }
            };
        }
    }
}

fn read_lines() -> anyhow::Result<String> {
    enable_raw_mode()?;

    let mut buf = Vec::new();
    loop {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
            // Ctrl Event
            if modifiers.contains(KeyModifiers::CONTROL) {
                match code {
                    KeyCode::Char('c') => {
                        disable_raw_mode()?;
                        std::process::exit(0);
                    }
                    KeyCode::Char('d') => {
                        break;
                    }
                    _ => {}
                }
                continue;
            }

            // Normal Event
            match code {
                KeyCode::Char(c) => {
                    buf.push(c);
                    queue!(io::stdout(), style::Print(c))?;
                }
                KeyCode::Backspace => {
                    buf.pop();
                    queue!(io::stdout(), cursor::MoveLeft(1))?;
                    queue!(io::stdout(), style::Print(' '))?;
                    queue!(io::stdout(), cursor::MoveLeft(1))?;
                }
                KeyCode::Tab => {
                    buf.extend_from_slice(&[' '; 4]);
                    queue!(io::stdout(), style::Print("    "))?;
                }
                KeyCode::Enter => {
                    buf.push('\n');
                    queue!(io::stdout(), style::Print('\n'))?;
                    queue!(io::stdout(), cursor::MoveToColumn(0))?;
                }
                _ => {}
            }

            io::stdout().flush()?;
        }
    }

    disable_raw_mode()?;
    println!();

    Ok(buf.into_iter().collect::<String>())
}

fn print_pretty_error(input: &str, (row, col): (u32, u32)) {
    let lines = input.split('\n').into_iter();
    let neighbor_lines = lines.skip(row as usize - 2).take(3);

    println!("-----");

    neighbor_lines
        .enumerate()
        .for_each(|(idx, line)| {
            println!("{:2}: {}", row - 1 + idx as u32, line);
        });

    println!("    {}^ here", " ".repeat(col as usize));
    println!("Error at line {}, columns {}.\n", row + 1, col + 1);
}
