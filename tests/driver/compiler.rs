use processor::driver::env::Windows_X86_64;
use processor::driver::prelude::*;
use processor::driver::Compiler;
use processor_lib_dev::langpart::*;

#[test]
fn compiler() {
    let langpart = expr_langpart();
    let _ = Compiler::<Windows_X86_64, ExprNode, ExprSyntax, ExprToken>::build(langpart);
}
