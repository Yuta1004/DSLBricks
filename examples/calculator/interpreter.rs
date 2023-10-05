use compiler::driver::env::Auto;
use compiler::driver::prelude::*;
use compiler::driver::Interpreter;
use compiler_lib_dev::langpart::*;

fn main() -> anyhow::Result<()> {
    let langpart = expr_langpart();
    Interpreter::<Auto, ExprNode, ExprSyntax, ExprToken>::build(langpart).exec()
}
