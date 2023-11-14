use blockdsl_compiler::executor::Compiler;
use blockdsl_lib::dev::dsl::*;

#[test]
fn compiler() {
    let dsl = expr_dsl();
    let _ = Compiler::<ExprNode, ExprSyntax, ExprToken>::from(dsl);
}
