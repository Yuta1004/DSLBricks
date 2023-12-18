use std::env;
use std::process::Command;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let css_path = current_dir.join("style.css");

    Command::new("cargo")
        .args(&["doc"])
        .args(&["--lib"])
        .args(&["--config", &format!("build.rustdocflags = [\"--extend-css\", \"{}\"]", css_path.display())])
        .current_dir("../../")
        .status()
        .unwrap();
}
