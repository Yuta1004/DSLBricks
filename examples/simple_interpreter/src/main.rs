use compiler::designer::codegen::load_dsl;
use compiler::executor::Interpreter;

load_dsl!(MyDSL);

fn main() -> anyhow::Result<()> {
    let dsl = MyDSL::gen()?;
    Interpreter::from(dsl).exec()
}
