use std::{env, fs};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dst_path = Path::new(&out_dir).join("dsl.rs");
    fs::write(&dst_path, "").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
