use depagerpp::driver::env::Windows_X86_64;
use depagerpp::driver::prelude::*;
use depagerpp::driver::Compiler;

#[test]
fn compiler() {
    let _ = Compiler::<Windows_X86_64>::build(&());
}
