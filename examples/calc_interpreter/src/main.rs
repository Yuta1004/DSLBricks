use compiler::load_dsl;
use compiler::executor::Interpreter;

load_dsl!(Expression);

fn main() -> anyhow::Result<()> {
    let dsl = Expression::gen()?;
    Interpreter::from(dsl).exec()
}
