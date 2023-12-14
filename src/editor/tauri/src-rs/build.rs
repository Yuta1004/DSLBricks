use std::process::Command;

fn main() {
    Command::new("make")
        .args(&["build"])
        .current_dir("../src-js")
        .status()
        .unwrap();

    tauri_build::build();

    println!("cargo:rerun-if-changed=../src-js/src");
    println!("cargo:rerun-if-changed=../src-js/next.config.js");
    println!("cargo:rerun-if-changed=../src-js/package.json");
    println!("cargo:rerun-if-changed=../src-js/tsconfig.json");
}
