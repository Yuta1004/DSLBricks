use blockdsl::driver::Compiler;
use blockdsl::lib::dev::dsl::*;

#[test]
fn compiler() {
    let dsl = expr_dsl();
    let _ = Compiler::<ExprNode, ExprSyntax, ExprToken>::from(dsl);
}
