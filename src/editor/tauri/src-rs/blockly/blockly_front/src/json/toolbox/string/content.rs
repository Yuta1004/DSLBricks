use std::fmt::Display;

use crate::ir;

pub struct ContentString(String);

impl From<&ir::Block> for ContentString {
    fn from(ir: &ir::Block) -> Self {
        let body = match ir {
            ir::Block::NoConnection(body) => body,
            ir::Block::TopBottomConnections(body) => body,
            ir::Block::TopConnection(body) => body,
            ir::Block::BottomConnection(body) => body,
        };
        ContentString(body.ty.to_string())
    }
}

impl Display for ContentString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{ kind: "block", type: "{}" }}"#, self.0)
    }
}
