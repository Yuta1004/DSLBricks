use std::fmt::Display;

use crate::ir;

pub struct TypeString(String);

impl From<&ir::Block> for TypeString {
    fn from(ir: &ir::Block) -> Self {
        let body = match ir {
            ir::Block::NoConnection(body) => body,
            ir::Block::TopBottomConnections(body) => body,
            ir::Block::TopConnection(body) => body,
            ir::Block::BottomConnection(body) => body,
        };
        TypeString(body.ty.to_string())
    }
}

impl Display for TypeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
