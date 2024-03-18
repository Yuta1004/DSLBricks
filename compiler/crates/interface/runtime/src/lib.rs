use processor::lexer::TokenSet;
use processor::parser::syntax::{post, pre};
use processor::DSL;

pub use macros::main;

pub trait Runnable<PostS, PreS, T>
where
    Self: From<DSL<PostS, PreS, T>>,
    PostS: post::Syntax<PreS, T>,
    PreS: pre::Syntax<PostS, T>,
    T: TokenSet,
{
    fn run(self) -> anyhow::Result<()>;
}
