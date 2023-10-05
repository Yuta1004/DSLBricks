use blockdsl::driver::interpreter::Interpreter;
use blockdsl::lib::dev::langpart::*;

fn main() -> anyhow::Result<()> {
    let langpart = expr_langpart();
    Interpreter::<ExprNode, ExprSyntax, ExprToken>::new(langpart).exec()
}
