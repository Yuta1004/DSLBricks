use interpreter::Interpreter;
use processor_lib_dev::langpart::*;

#[test]
fn interpreter() {
    let langpart = expr_langpart();
    Interpreter::<ExprNode, ExprSyntax, ExprToken>::new(langpart);
}
