use depagerpp::builder::prelude::*;
use depagerpp::builder::env::Windows_X86_64;
use depagerpp::builder::target::Compiler;

#[test]
fn simplelang_compiler() {
    langbuild!()
        .target(Compiler::<Windows_X86_64>::new())
        .build()
}
