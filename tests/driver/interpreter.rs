use compiler::driver::env::Windows_X86_64;
use compiler::driver::prelude::*;
use compiler::driver::Interpreter;
use compiler_lib_dev::langpart::*;

#[test]
fn interpreter() {
    let langpart = expr_langpart();
    let _ = Interpreter::<Windows_X86_64, ExprNode, ExprSyntax, ExprToken>::build(langpart);
}
