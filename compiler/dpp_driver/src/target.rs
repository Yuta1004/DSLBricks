mod compiler;
mod interperter;

pub use compiler::Compiler;
pub use interperter::Interpreter;

use crate::env::Environment;

pub trait Target<E: Environment> {
    fn build(&self, _lang: &());
}

#[cfg(test)]
mod test {
    use super::{Compiler, Interpreter};
    use crate::env::Windows_X86_64;

    #[test]
    fn compiler() {
        let _ = Compiler::<Windows_X86_64>::new();
    }

    #[test]
    fn interperter() {
        let _ = Interpreter::<Windows_X86_64>::new();
    }
}
