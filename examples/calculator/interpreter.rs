use interpreter::Interpreter;
use processor_lib_dev::langpart::*;

fn main() -> anyhow::Result<()> {
    let langpart = expr_langpart();
    Interpreter::<ExprNode, ExprSyntax, ExprToken>::new(langpart).exec()
}
