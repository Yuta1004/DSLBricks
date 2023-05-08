use std::marker::PhantomData;

use crate::env::Environment;
use crate::target::Target;

pub struct Compiler<E: Environment>(PhantomData<E>);

impl<E> Compiler<E>
where
    E: Environment,
{
    pub fn new() -> Self {
        Compiler(PhantomData)
    }
}

impl<E> Target<E> for Compiler<E>
where
    E: Environment,
{
    fn build(&self, _lang: &()) {
        println!("===== BuildInfo ===== ");
        println!("* Target : Compiler");
        println!("* Env : {}", <E as Environment>::name());
        println!("===================== ");
    }
}
