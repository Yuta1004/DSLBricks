use std::fmt::Display;

use crate::ir::BlocklyIR;

pub struct BlocklyContentString(String);

impl From<&BlocklyIR> for BlocklyContentString {
    fn from(ir: &BlocklyIR) -> Self {
        let body = match ir {
            BlocklyIR::NoConnection(body) => body,
            BlocklyIR::TopBottomConnections(body) => body,
            BlocklyIR::TopConnection(body) => body,
            BlocklyIR::BottomConnection(body) => body,
        };
        BlocklyContentString(body.ty.to_string())
    }
}

impl Display for BlocklyContentString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{ kind: "block", type: "{}" }}"#, self.0)
    }
}
