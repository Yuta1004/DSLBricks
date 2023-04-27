mod orig_target;

use depagerpp::builder::prelude::*;
use depagerpp::builder::target::{Compiler, Interpreter};
use depagerpp::builder::env::Windows_X86_64;

use orig_target::{OrigTarget, OrigEnv};

#[test]
fn check_orig_target() {
    langbuild!()
        .target(OrigTarget::<OrigEnv>::new())
        .target(OrigTarget::<Windows_X86_64>::new())
        .target(Compiler::<OrigEnv>::new())
        .target(Interpreter::<OrigEnv>::new())
        .build()
}
