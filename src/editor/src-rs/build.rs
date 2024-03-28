use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Catalog (lib)
    blockly::front::gen_ts_files("../src-js/src/custom", catalog_reimpl::catalog()).unwrap();

    // Catalog (doc)
    let cur_dir = env::current_dir().unwrap();
    let doc_dir = Path::new(&cur_dir).join("../src-js/public/rustdoc");
    if !doc_dir.exists() {
        panic!("Bundle files not found. Please run `make setup` from the root directory first.");
    }

    // React project
    Command::new("make")
        .args(["build"])
        .current_dir("../src-js")
        .status()
        .unwrap();

    // Tauri
    tauri_build::build();

    println!("cargo:rerun-if-changed=../src-js/src/components");
    println!("cargo:rerun-if-changed=../src-js/src/pages");
    println!("cargo:rerun-if-changed=../src-js/src/tauri");
    println!("cargo:rerun-if-changed=../src-js/public");
    println!("cargo:rerun-if-changed=../src-js/next.config.js");
    println!("cargo:rerun-if-changed=../src-js/package.json");
    println!("cargo:rerun-if-changed=../src-js/tsconfig.json");
}
