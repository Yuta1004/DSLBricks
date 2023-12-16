use std::fmt::Display;

use crate::ir::BlocklyIR;

pub struct BlocklyTypeString(String);

impl From<&BlocklyIR> for BlocklyTypeString {
    fn from(ir: &BlocklyIR) -> Self {
        let body = match ir {
            BlocklyIR::NoConnection(body) => body,
            BlocklyIR::TopBottomConnections(body) => body,
            BlocklyIR::TopConnection(body) => body,
            BlocklyIR::BottomConnection(body) => body,
        };
        BlocklyTypeString(body.ty.to_string())
    }
}

impl Display for BlocklyTypeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
