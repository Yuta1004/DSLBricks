use blockdsl_compiler::executor::Interpreter;
use blockdsl_lib::dev::dsl::*;

#[test]
fn interpreter() {
    let dsl = expr_dsl();
    let _ = Interpreter::<ExprNode, ExprSyntax, ExprToken>::from(dsl);
}
