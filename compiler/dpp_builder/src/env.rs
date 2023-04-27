mod windows;

pub use windows::x86_64::Windows_X86_64;

pub trait Environment {
    fn name() -> String;
}
