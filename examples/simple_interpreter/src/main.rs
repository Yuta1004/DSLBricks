use blockdsl::driver::Interpreter;

include!(concat!(env!("OUT_DIR"), "/dsl.rs"));

fn main() -> anyhow::Result<()> {
    let dsl = MyDSL::gen()?;
    Interpreter::new(dsl).exec()
}
