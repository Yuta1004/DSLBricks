use depagerpp::builder::prelude::*;
use depagerpp::builder::target::Compiler;
use depagerpp::builder::env::Windows_X86_64;

#[test]
fn simplelang() {
    Compiler::<Windows_X86_64>::from(()).build()
}
