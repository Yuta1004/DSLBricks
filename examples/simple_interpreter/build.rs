use std::{env, fs};
use std::path::Path;

use blockdsl::designer::codegen;
use blockdsl::designer::design::{DSLDesign, DSLPart};

#[derive(Debug, Default)]
struct MyDSL;

impl DSLDesign for MyDSL {
    fn design() -> Vec<Box<dyn DSLPart>> {
        vec![]
    }
}

fn main() {
    let dsl_rcode = codegen::gen_rust::<MyDSL>().unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dst_path = Path::new(&out_dir).join("MyDSL.rs");
    fs::write(&dst_path, dsl_rcode).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
