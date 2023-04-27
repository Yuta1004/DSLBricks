pub mod prelude;
pub mod target;
pub mod env;

use target::Target;
use env::Environment;

pub struct Builder {
    queue: Vec<Box<dyn FnOnce() -> ()>>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            queue: vec![],
        }
    }

    pub fn add<T, E>(mut self, target: T) -> Builder
    where
        T: Target<E> + 'static,
        E: Environment
    {
        self.queue.push(Box::new(move || target.build()));
        self
    }

    pub fn build(self) {
        for target in self.queue {
            target()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::target::{Compiler, Interpreter};
    use crate::env::Windows_X86_64;
    use super::Builder;

    #[test]
    fn builder_single() {
        Builder::new()
            .add(Compiler::<Windows_X86_64>::from(()))
            .build()
    }

    #[test]
    fn builder_multiple() {
        Builder::new()
            .add(Compiler::<Windows_X86_64>::from(()))
            .add(Interpreter::<Windows_X86_64>::from(()))
            .build()
    }
}
