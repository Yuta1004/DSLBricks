use std::marker::PhantomData;

use crate::env::Environment;
use crate::target::Target;

pub struct Compiler<E: Environment>(PhantomData<E>);

impl<E> Target<E> for Compiler<E>
where
    E: Environment,
{
    fn build(_lang: &()) {
        println!("===== BuildInfo ===== ");
        println!("* Target : Compiler");
        println!("* Env : {}", <E as Environment>::name());
        println!("===================== ");
    }
}
