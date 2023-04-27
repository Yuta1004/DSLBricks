use std::marker::PhantomData;

use crate::target::Target;
use crate::env::Environment;

pub struct Interpreter<E: Environment> (PhantomData<E>);

impl<E> From<()> for Interpreter<E>
where
    E: Environment
{
    fn from(_lang: ()) -> Self {
        Interpreter(PhantomData)
    }
}

impl<E> Target<E> for Interpreter<E>
where
    E: Environment
{
    fn build(&self) {
        println!("===== BuildInfo ===== ");
        println!("* Target : Interpreter");
        println!("* Env : {}", <E as Environment>::name());
        println!("===================== ");
    }
}
