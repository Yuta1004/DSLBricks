mod string;

use std::fmt::Display;

use string::BlocklyContentString;

use crate::ir::BlocklyIR;

pub struct BlocklyToolBox {
    name: String,
    contents: Vec<BlocklyContentString>,
}

impl BlocklyToolBox {
    pub fn new<T: Into<String>>(name: T, irs: &[BlocklyIR]) -> Self {
        BlocklyToolBox {
            name: name.into(),
            contents: irs.into_iter().map(BlocklyContentString::from).collect(),
        }
    }
}

impl Display for BlocklyToolBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents = self
            .contents
            .iter()
            .map(|content| format!("{}", content))
            .collect::<Vec<String>>()
            .join(",");

        write!(f, r#"
            {{
                kind: "category",
                name: "{}",
                contents: [{}]
            }}
        "#, self.name, contents)
    }
}

#[cfg(test)]
mod test {
    use super::BlocklyToolBox;
    use crate::ir::BlocklyIR;

    #[test]
    fn simple() {
        let irs = vec![
            BlocklyIR::new("test1", vec![]),
            BlocklyIR::new("test2", vec![]),
            BlocklyIR::new("test3", vec![]),
        ];
        let toolbox = BlocklyToolBox::new("Group", &irs);
        let _ = format!("{}", toolbox);
    }
}
