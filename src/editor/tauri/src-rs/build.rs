use std::env;
use std::process::Command;

fn main() {
    // Catalog (lib)
    let catalog = catalog::catalog();
    blockly::front::gen_ts_files("../src-js/src/custom", catalog.as_slice()).unwrap();

    // Catalog (bin)
    let current_dir = env::current_dir().unwrap();
    Command::new("make")
        .args(&["bin"])
        .args(&[format!("OUT_DIR={}", current_dir.display())])
        .current_dir("../../../../docs/rustdoc/tools/rustdoc_web")
        .status()
        .unwrap();

    // React project
    Command::new("make")
        .args(&["build"])
        .current_dir("../src-js")
        .status()
        .unwrap();

    // Tauri
    tauri_build::build();

    println!("cargo:rerun-if-changed=../src-js/src/components");
    println!("cargo:rerun-if-changed=../src-js/src/pages");
    println!("cargo:rerun-if-changed=../src-js/src/tauri");
    println!("cargo:rerun-if-changed=../src-js/next.config.js");
    println!("cargo:rerun-if-changed=../src-js/package.json");
    println!("cargo:rerun-if-changed=../src-js/tsconfig.json");
}
