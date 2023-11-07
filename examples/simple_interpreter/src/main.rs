use blockdsl::driver::Interpreter;

include!(concat!(env!("OUT_DIR"), "/MyDSL.rs"));

fn main() -> anyhow::Result<()> {
    let dsl = MyDSL::gen()?;
    Interpreter::new(dsl).exec()
}
