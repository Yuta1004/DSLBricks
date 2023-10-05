use compiler::driver::env::Windows_X86_64;
use compiler::driver::prelude::*;
use compiler::driver::Compiler;
use compiler_lib_dev::langpart::*;

#[test]
fn compiler() {
    let langpart = expr_langpart();
    let _ = Compiler::<Windows_X86_64, ExprNode, ExprSyntax, ExprToken>::build(langpart);
}
