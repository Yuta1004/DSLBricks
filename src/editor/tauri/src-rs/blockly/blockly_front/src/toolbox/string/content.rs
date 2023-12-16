use std::fmt::Display;

use crate::ir::BlocklyIR;

pub struct BlocklyContentString(String);

impl From<&BlocklyIR> for BlocklyContentString {
    fn from(ir: &BlocklyIR) -> Self {
        BlocklyContentString(ir.ty.to_string())
    }
}

impl Display for BlocklyContentString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{ kind: "block", type: "{}" }}"#, self.0)
    }
}
