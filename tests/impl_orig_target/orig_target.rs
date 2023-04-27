use std::marker::PhantomData;

use depagerpp::builder::prelude::*;

pub struct OrigTarget<E> (PhantomData<E>);

impl<E: Environment> OrigTarget<E> {
    pub fn new() -> OrigTarget<E> {
        OrigTarget(PhantomData)
    }
}

impl<E> Target<E> for OrigTarget<E>
where
    E: Environment
{
    fn build(&self, _lang: &()) {
        println!("~~~~~ BuildInfo(Original) ~~~~~");
        println!("* Target : Original Target");
        println!("* Env : {}", <E as Environment>::name());
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
}

pub struct OrigEnv;

impl Environment for OrigEnv {
    fn name() -> &'static str {
        "Original"
    }
}
