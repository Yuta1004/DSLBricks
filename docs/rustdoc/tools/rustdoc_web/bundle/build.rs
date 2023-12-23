use std::env;
use std::process::Command;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let css_path = current_dir.join("style.css");

    if cfg!(feature = "embedded") {
        Command::new("cargo")
            .args(&["doc"])
            .args(&["--lib"])
            .args(&["--workspace"])
            .args(&["--exclude", "ui"])
            .args(&["--target-dir", "./docs/rustdoc/tools/rustdoc_web/bundle/target"])
            .args(&["--config", &format!("build.rustdocflags = [\"--extend-css\", \"{}\"]", css_path.display())])
            .current_dir("../../../../../")
            .status()
            .unwrap();
    } else {
        Command::new("cargo")
            .args(&["doc"])
            .args(&["--lib"])
            .args(&["--workspace"])
            .args(&["--exclude", "ui"])
            .args(&["--target-dir", "./docs/rustdoc/tools/rustdoc_web/bundle/target"])
            .current_dir("../../../../../")
            .status()
            .unwrap();
    }

    println!("cargo:rerun-if-changed=style.css");
    println!("cargo:rerun-if-changed=../../../../../catalog");
}