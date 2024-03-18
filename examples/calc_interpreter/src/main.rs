use compiler::entrypoint;
use lib::runtime::Interpreter;

#[entrypoint::main]
fn main(dsl: DSL) -> anyhow::Result<()> {
    Interpreter::from(dsl).exec()
}
