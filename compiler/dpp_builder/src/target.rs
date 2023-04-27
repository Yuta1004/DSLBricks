mod compiler;
mod interperter;

pub use compiler::Compiler;
pub use interperter::Interpreter;

use crate::env::Environment;

pub trait Target<E: Environment> {
    fn build(&self);
}

#[cfg(test)]
mod test {
    use crate::env::Windows_X86_64;
    use super::{Compiler, Interpreter};

    #[test]
    fn compiler() {
        let _ = Compiler::<Windows_X86_64>::from(());
    }

    #[test]
    fn interperter() {
        let _ = Interpreter::<Windows_X86_64>::from(());
    }
}
