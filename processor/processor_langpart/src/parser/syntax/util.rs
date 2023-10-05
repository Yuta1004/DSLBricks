use super::{ASyntax, Syntax};

use crate::lexer::Token;

pub struct VoidSemantics;

impl<S, T> ASyntax<S, T> for VoidSemantics
where
    S: Syntax<Self, T>,
    T: Token,
{
    fn mapping(_: S, _: Vec<(Option<Box<Self>>, Option<&str>)>) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(VoidSemantics {}))
    }
}
