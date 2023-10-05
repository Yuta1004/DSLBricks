mod compiler;
mod interperter;

pub use compiler::Compiler;
pub use interperter::Interpreter;

use super::env::Environment;

use langpart::parser::syntax::ASyntax;
use langpart::prelude::*;
use langpart::LangPart;

pub trait Target<E, A, S, T>
where
    E: Environment,
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    fn build(lang: LangPart<A, S, T>) -> Self;
    fn exec(self) -> anyhow::Result<()>;
}
