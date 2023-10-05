use interpreter::Interpreter;
use lib::dev::langpart::*;

#[test]
fn interpreter() {
    let langpart = expr_langpart();
    Interpreter::<ExprNode, ExprSyntax, ExprToken>::new(langpart);
}
