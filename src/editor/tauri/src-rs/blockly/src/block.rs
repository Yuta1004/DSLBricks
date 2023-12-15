mod string;

use std::fmt::Display;

use crate::ir::BlocklyIR;

use string::{
    BlocklyTypeString,
    BlocklyInitString,
};

pub struct BlocklyBlock {
    ty: BlocklyTypeString,
    init: BlocklyInitString,
}

impl From<&BlocklyIR> for BlocklyBlock {
    fn from(ir: &BlocklyIR) -> Self {
        BlocklyBlock {
            ty: BlocklyTypeString::from(ir),
            init: BlocklyInitString::from(ir),
        }
    }
}

impl Display for BlocklyBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"
            Blockly.Block["{}"] = {{
                init: {}
            }}
        "#, self.ty, self.init)
    }
}

#[cfg(test)]
mod test {
    use super::BlocklyBlock;
    use crate::ir::{BlocklyIR, BlocklyIRComponent};

    #[test]
    fn simple() {
        let components = vec![
            BlocklyIRComponent::Text { title: "Test".to_string() },
            BlocklyIRComponent::Variable { title: "variable".to_string() },
            BlocklyIRComponent::TextInput { title: "text".to_string() },
            BlocklyIRComponent::BlockInput { title: "block".to_string() },
        ];
        let ir = BlocklyIR::new("test", components);
        let block = BlocklyBlock::from(&ir);
        let _ = format!("{}", block);
    }
}
