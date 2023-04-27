use depagerpp::builder::env::Windows_X86_64;
use depagerpp::builder::target::Compiler;
use depagerpp::builder::Builder;

#[test]
fn simplelang_compiler() {
    Builder::new()
        .add(Compiler::<Windows_X86_64>::new())
        .build()
}
