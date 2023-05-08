use std::marker::PhantomData;

use crate::env::Environment;
use crate::target::Target;

pub struct Interpreter<E: Environment>(PhantomData<E>);

impl<E> Interpreter<E>
where
    E: Environment,
{
    pub fn new() -> Self {
        Interpreter(PhantomData)
    }
}

impl<E> Target<E> for Interpreter<E>
where
    E: Environment,
{
    fn build(&self, _lang: &()) {
        println!("===== BuildInfo ===== ");
        println!("* Target : Interpreter");
        println!("* Env : {}", <E as Environment>::name());
        println!("===================== ");
    }
}
