use depagerpp::driver::env::Windows_X86_64;
use depagerpp::driver::prelude::*;
use depagerpp::driver::Interpreter;

#[test]
fn interpreter() {
    let _ = Interpreter::<Windows_X86_64>::build(&());
}
