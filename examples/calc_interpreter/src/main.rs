use compiler::entrypoint::main as entrypoint;
use lib::runtime::Interpreter;

#[entrypoint]
fn main(dsl: DSL) -> anyhow::Result<()> {
    Interpreter::from(dsl).exec()
}
