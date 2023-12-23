use std::fmt::{Display, Write};

use crate::ir;

pub struct ToolBoxString(String);

impl From<&ir::ToolBox> for ToolBoxString {
    fn from(toolbox: &ir::ToolBox) -> Self {
        let mut body = String::new();

        write!(&mut body, "[").unwrap();
        for item in &toolbox.items {
            match item {
                ir::ToolBoxItem::Block(block) => {
                    let block = match block {
                        ir::Block::NoConnection(body) => body,
                        ir::Block::TopBottomConnections(body) => body,
                        ir::Block::TopConnection(body) => body,
                        ir::Block::BottomConnection(body) => body,
                    };
                    write!(
                        &mut body,
                        r#"{{ kind: "block", type: "{}" }},"#,
                        block.ty,
                    ).unwrap();
                }
                ir::ToolBoxItem::ToolBox(toolbox) => {
                    write!(
                        &mut body,
                        r#"{{ kind: "category", colour: "100", name: "{}", contents: {} }},"#,
                        toolbox.name,
                        ToolBoxString::from(toolbox),
                    ).unwrap();
                }
            }
        }
        write!(&mut body, "]").unwrap();

        ToolBoxString(body)
    }
}

impl Display for ToolBoxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
