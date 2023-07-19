use depagerpp::driver::prelude::*;
use depagerpp::driver::env::Windows_X86_64;
use depagerpp::driver::target::Interpreter;

#[test]
fn interpreter() {
    Interpreter::<Windows_X86_64>::build(&())
}
