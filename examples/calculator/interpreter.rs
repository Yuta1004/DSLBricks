use interpreter::Interpreter;
use lib::dev::langpart::*;

fn main() -> anyhow::Result<()> {
    let langpart = expr_langpart();
    Interpreter::<ExprNode, ExprSyntax, ExprToken>::new(langpart).exec()
}
