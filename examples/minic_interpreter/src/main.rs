use compiler::executor::Interpreter;
use compiler::load_dsl;

load_dsl!();

fn main() -> anyhow::Result<()> {
    let dsl = DSL::gen()?;
    Interpreter::from(dsl).exec()
}
