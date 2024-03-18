use compiler::runtime::main;
use lib::runner::Interpreter;

#[main]
fn main(dsl: DSL) -> anyhow::Result<()> {
    Interpreter::from(dsl).exec()
}
