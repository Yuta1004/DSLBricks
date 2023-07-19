use depagerpp::driver::env::Windows_X86_64;
use depagerpp::driver::prelude::*;
use depagerpp::driver::Compiler;
use depagerpp_lib_dev::langpart::*;

#[test]
fn compiler() {
    let langpart = expr_langpart();
    let _ = Compiler::<Windows_X86_64, ExprNode, ExprSyntax, ExprToken>::build(langpart);
}
