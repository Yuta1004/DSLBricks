pub mod prelude;
pub mod target;
pub mod env;

use env::Environment;
use target::Target;

pub struct Builder {
    queue: Vec<Box<dyn FnOnce() -> ()>>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { queue: vec![] }
    }

    pub fn target<T, E>(mut self, target: T) -> Builder
    where
        T: Target<E> + 'static,
        E: Environment,
    {
        self.queue.push(Box::new(move || target.build(&())));
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
    use super::Builder;
    use crate::env::Windows_X86_64;
    use crate::target::{Compiler, Interpreter};

    #[test]
    fn builder_single() {
        Builder::new()
            .target(Compiler::<Windows_X86_64>::new())
            .build()
    }

    #[test]
    fn builder_multiple() {
        Builder::new()
            .target(Compiler::<Windows_X86_64>::new())
            .target(Interpreter::<Windows_X86_64>::new())
            .build()
    }
}
