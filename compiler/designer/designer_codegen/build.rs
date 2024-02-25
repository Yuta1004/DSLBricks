fn main() {
    println!("cargo:rerun-if-changed=template/Rust.txt");
    println!("cargo:rerun-if-changed=build.rs");
}
