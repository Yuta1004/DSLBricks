use std::process::Command;

fn main() {
    // Catalog
    let catalog = catalog::catalog();
    blockly::front::gen_ts_files("../src-js/src/custom", catalog.as_slice()).unwrap();

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
