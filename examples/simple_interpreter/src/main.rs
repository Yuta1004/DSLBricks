use blockdsl_compiler::designer::codegen::load_dsl;
use blockdsl_compiler::executor::Interpreter;

load_dsl!(MyDSL);

fn main() -> anyhow::Result<()> {
    let dsl = MyDSL::gen()?;
    Interpreter::from(dsl).exec()
}
