use std::env;
use std::path::Path;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let css_path = current_dir.join("rustdoc");
    if !Path::new(&css_path).exists() {
        panic!("The 'rustdoc' directory does not exist.\nPlease run `make doc-embedded` from 'src/rustdoc_web/crates/rustdoc_bundle' directory.");
    }
}
