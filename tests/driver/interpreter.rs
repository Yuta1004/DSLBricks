use blockdsl::driver::interpreter::Interpreter;
use blockdsl::lib::dev::langpart::*;

#[test]
fn interpreter() {
    let langpart = expr_langpart();
    Interpreter::<ExprNode, ExprSyntax, ExprToken>::new(langpart);
}
