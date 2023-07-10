mod compiler;
mod interperter;

pub use compiler::Compiler;
pub use interperter::Interpreter;

use crate::env::Environment;

pub trait Target<E: Environment> {
    fn build(lang: &());
}

#[cfg(test)]
mod test {
    use super::{Compiler, Interpreter, Target};
    use crate::env::Windows_X86_64;

    #[test]
    fn compiler() {
        Compiler::<Windows_X86_64>::build(&());
    }

    #[test]
    fn interperter() {
        Interpreter::<Windows_X86_64>::build(&());
    }
}
