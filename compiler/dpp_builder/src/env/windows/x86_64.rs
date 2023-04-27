use crate::env::Environment;

#[allow(non_camel_case_types)]
pub struct Windows_X86_64;

impl Environment for Windows_X86_64 {
    fn name() -> String {
        "Windows_X86_64".to_string()
    }
}
