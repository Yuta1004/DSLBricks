use processor::driver::env::Windows_X86_64;
use processor::driver::prelude::*;
use processor::driver::Interpreter;
use processor_lib_dev::langpart::*;

#[test]
fn interpreter() {
    let langpart = expr_langpart();
    let _ = Interpreter::<Windows_X86_64, ExprNode, ExprSyntax, ExprToken>::build(langpart);
}
