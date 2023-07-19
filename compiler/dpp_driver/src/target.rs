mod compiler;
mod interperter;

pub use compiler::Compiler;
pub use interperter::Interpreter;

use super::env::Environment;

pub trait Target<E: Environment> {
    fn build(lang: &()) -> Self;
    fn exec(self: Self) -> anyhow::Result<()>;
}
