use blockdsl::designer::codegen::load_dsl;
use blockdsl::driver::Interpreter;

load_dsl!(MyDSL);

fn main() -> anyhow::Result<()> {
    let dsl = MyDSL::gen()?;
    Interpreter::new(dsl).exec()
}
