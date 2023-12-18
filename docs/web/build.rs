use std::process::Command;

fn main() {
    Command::new("cargo")
        .args(&["doc"])
        .args(&["-p", "catalog"])
        .args(&["--exclude", "docs_web"])
        .current_dir("../../")
        .status()
        .unwrap();
}
