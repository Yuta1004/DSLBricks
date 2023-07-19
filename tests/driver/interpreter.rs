use depagerpp::driver::env::Windows_X86_64;
use depagerpp::driver::prelude::*;
use depagerpp::driver::Interpreter;
use depagerpp_lib_dev::langpart::*;

#[test]
fn interpreter() {
    let langpart = expr_langpart();
    let _ = Interpreter::<Windows_X86_64, ExprNode, ExprSyntax, ExprToken>::build(langpart);
}
