use depagerpp::driver::env::Auto;
use depagerpp::driver::prelude::*;
use depagerpp::driver::Interpreter;
use depagerpp_lib_dev::langpart::*;

fn main() -> anyhow::Result<()> {
    let langpart = expr_langpart();
    Interpreter::<Auto, ExprNode, ExprSyntax, ExprToken>::build(langpart).exec()
}
