use blockdsl::executor::Interpreter;
use blockdsl::lib::dev::dsl::*;

#[test]
fn interpreter() {
    let dsl = expr_dsl();
    let _ = Interpreter::<ExprNode, ExprSyntax, ExprToken>::from(dsl);
}
