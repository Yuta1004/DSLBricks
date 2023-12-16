use std::process::Command;

use catalog::catalog;

fn main() {
    // Catalog
    blockly_front::gen_ts_files("../src-js/src/custom", catalog()).unwrap();

    // React project
    Command::new("make")
        .args(&["build"])
        .current_dir("../src-js")
        .status()
        .unwrap();

    // Tauri
    tauri_build::build();

    println!("cargo:rerun-if-changed=../src-js/src");
    println!("cargo:rerun-if-changed=../src-js/next.config.js");
    println!("cargo:rerun-if-changed=../src-js/package.json");
    println!("cargo:rerun-if-changed=../src-js/tsconfig.json");
}
