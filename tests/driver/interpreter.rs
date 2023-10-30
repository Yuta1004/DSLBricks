use blockdsl::driver::interpreter::Interpreter;
use blockdsl::lib::dev::dsl::*;

#[test]
fn interpreter() {
    let dsl = expr_dsl();
    Interpreter::<ExprNode, ExprSyntax, ExprToken>::new(dsl);
}
