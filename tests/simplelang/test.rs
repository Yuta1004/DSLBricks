use depagerpp::driver::prelude::*;
use depagerpp::driver::env::Windows_X86_64;
use depagerpp::driver::target::Compiler;

#[test]
fn simplelang_compiler() {
    langbuild!()
        .target(Compiler::<Windows_X86_64>::new())
        .build()
}
