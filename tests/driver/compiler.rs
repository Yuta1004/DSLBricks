use depagerpp::driver::env::Windows_X86_64;
use depagerpp::driver::prelude::*;
use depagerpp::driver::target::Compiler;

#[test]
fn compiler() {
    Compiler::<Windows_X86_64>::build(&())
}
