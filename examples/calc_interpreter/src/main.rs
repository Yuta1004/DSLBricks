use compiler::entrypoint::main;
use lib::runtime::Interpreter;

#[main]
fn main(dsl: DSL) -> anyhow::Result<()> {
    Interpreter::from(dsl).exec()
}
