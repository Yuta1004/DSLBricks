use langpart::LangPart;
use langpart::lexer::Token;
use langpart::parser::syntax::{ASyntax, Syntax};
pub struct DSL<A, S, T> (LangPart<A, S, T>)
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static;

impl<A, S, T> DSL<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T> + 'static,
    T: Token + 'static,
{
    pub fn gen() -> anyhow::Result<DSL<A, S, T>> {
        Ok(DSL(LangPart::<A, S, T>::gen()?))
    }

    pub fn process(&self, input: &str) -> anyhow::Result<Box<A>> {
        self.0.process(input)
    }
}
