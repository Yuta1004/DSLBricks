use std::env;
use std::process::Command;

fn main() {
    // Catalog (lib)
    blockly::front::gen_ts_files("../src-js/src/custom", catalog::catalog()).unwrap();

    // Catalog (bin)
    let current_dir = env::current_dir().unwrap();
    Command::new("make")
        .args(&["bin"])
        .args(&[format!("OUT_DIR={}", current_dir.display())])
        .current_dir("../../../src/rustdoc_web")
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

    println!("cargo:rerun-if-changed=../../../src/rustdoc_web/src");
    println!("cargo:rerun-if-changed=../../../src/rustdoc_web/bundle/src");
    println!("cargo:rerun-if-changed=../../../src/rustdoc_web/bundle/build.rs");
    println!("cargo:rerun-if-changed=../../../src/rustdoc_web/bundle/style.css");
    println!("cargo:rerun-if-changed=../../../catalog");

    println!("cargo:rerun-if-changed=../src-js/src/components");
    println!("cargo:rerun-if-changed=../src-js/src/pages");
    println!("cargo:rerun-if-changed=../src-js/src/tauri");
    println!("cargo:rerun-if-changed=../src-js/next.config.js");
    println!("cargo:rerun-if-changed=../src-js/package.json");
    println!("cargo:rerun-if-changed=../src-js/tsconfig.json");
}
