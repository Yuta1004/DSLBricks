use std::marker::PhantomData;

use crate::env::Environment;

pub trait Target<E>
where
    E: Environment
{
    fn build(&self);
}

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
        println!("* Env : {}", <E as Environment>::name());
        println!("===================== ");
    }
}
