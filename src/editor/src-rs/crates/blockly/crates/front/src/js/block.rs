mod string;

use std::fmt::Display;

use crate::ir;

use string::{InitString, TypeString};

pub struct Block {
    ty: TypeString,
    init: InitString,
}

impl From<&ir::Block> for Block {
    fn from(ir: &ir::Block) -> Self {
        Block {
            ty: TypeString::from(ir),
            init: InitString::from(ir),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
            Blockly.Blocks["{}"] = {{
                init: {}
            }}
        "#,
            self.ty, self.init
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{ir, js};

    #[test]
    fn simple() {
        let components = vec![
            ir::BlockComponent::new_text("Test"),
            ir::BlockComponent::new_variable("variable", "variable"),
            ir::BlockComponent::new_text_input("text", "text"),
            ir::BlockComponent::new_block_input("block", "block"),
        ];
        let ir = ir::Block::new_no_connection("test", components);
        let block = js::Block::from(&ir);
        let _ = format!("{}", block);
    }
}
