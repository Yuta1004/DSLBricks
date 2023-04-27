use depagerpp::builder::prelude::*;
use depagerpp::builder::Builder;

struct OrigEnvSettings;

impl Environment for OrigEnvSettings {
    fn name() -> &'static str {
        "Original"
    }
}

#[derive(Default)]
struct OrigTarget;

impl Target<OrigEnvSettings> for OrigTarget {
    fn build(&self, _lang: &()) {
        // do nothing
    }
}

#[test]
fn check_orig_target() {
    Builder::new().target(OrigTarget::default()).build()
}
