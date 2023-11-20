use compiler::load_dsl;
use compiler::executor::Interpreter;

load_dsl!();

fn main() -> anyhow::Result<()> {
    let dsl = DSL::gen()?;
    Interpreter::from(dsl).exec()
}
