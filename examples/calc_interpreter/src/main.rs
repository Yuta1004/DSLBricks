use compiler::load_dsl;
use runner::Interpreter;

load_dsl!();

fn main() -> anyhow::Result<()> {
    let dsl = DSL::gen()?;
    Interpreter::from(dsl).exec()
}
