mod string;

use std::fmt::Display;

use string::ToolBoxString;

use crate::ir;

pub struct ToolBox {
    contents: ToolBoxString,
}

impl From<&ir::ToolBox> for ToolBox {
    fn from(toolbox: &ir::ToolBox) -> Self {
        ToolBox {
            contents: ToolBoxString::from(toolbox),
        }
    }
}

impl Display for ToolBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
            {{
                kind: "categoryToolbox",
                contents: {}
            }}
        "#,
            self.contents
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{ir, js};

    #[test]
    fn simple() {
        let ir = ir::ToolBox::new("toolbox")
            .push_block(ir::Block::new_no_connection("test1", vec![]))
            .push_block(ir::Block::new_no_connection("test2", vec![]))
            .push_block(ir::Block::new_no_connection("test3", vec![]));
        let toolbox = js::ToolBox::from(&ir);
        let _ = format!("{}", toolbox);
    }
}
