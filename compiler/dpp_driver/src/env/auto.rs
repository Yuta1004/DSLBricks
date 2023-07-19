use crate::env::Environment;

#[allow(non_camel_case_types)]
pub struct Auto;

impl Environment for Auto {
    fn name() -> &'static str {
        "Auto"
    }
}
