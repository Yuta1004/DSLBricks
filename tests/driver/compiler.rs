use compiler::Compiler;
use lib::dev::langpart::*;

#[test]
fn compiler() {
    let langpart = expr_langpart();
    let _ = Compiler::<ExprNode, ExprSyntax, ExprToken>::new(langpart);
}
