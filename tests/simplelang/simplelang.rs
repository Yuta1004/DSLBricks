use depagerpp::builder::env::Windows_X86_64;
use depagerpp::builder::target::Compiler;
use depagerpp::builder::Builder;
use depagerpp::macros::langbuild;

#[test]
fn simplelang_compiler() {
    langbuild!()
        .target(Compiler::<Windows_X86_64>::new())
        .build()
}
