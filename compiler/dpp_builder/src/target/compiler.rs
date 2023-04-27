use std::marker::PhantomData;

use crate::target::Target;
use crate::env::Environment;

pub struct Compiler<E: Environment> (PhantomData<E>);

impl<E> From<()> for Compiler<E>
where
    E: Environment
{
    fn from(_lang: ()) -> Self {
        Compiler(PhantomData)
    }
}

impl<E> Target<E> for Compiler<E>
where
    E: Environment
{
    fn build(&self) {
        println!("===== BuildInfo ===== ");
        println!("* Target : Compiler");
        println!("* Env : {}", <E as Environment>::name());
        println!("===================== ");
    }
}
