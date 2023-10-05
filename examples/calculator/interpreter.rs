use processor::driver::env::Auto;
use processor::driver::prelude::*;
use processor::driver::Interpreter;
use processor_lib_dev::langpart::*;

fn main() -> anyhow::Result<()> {
    let langpart = expr_langpart();
    Interpreter::<Auto, ExprNode, ExprSyntax, ExprToken>::build(langpart).exec()
}
