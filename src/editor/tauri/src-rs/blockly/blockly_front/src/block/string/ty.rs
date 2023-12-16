use std::fmt::Display;

use crate::ir::BlocklyIR;

pub struct BlocklyTypeString(String);

impl From<&BlocklyIR> for BlocklyTypeString {
    fn from(ir: &BlocklyIR) -> Self {
        BlocklyTypeString(ir.ty.to_string())
    }
}

impl Display for BlocklyTypeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
