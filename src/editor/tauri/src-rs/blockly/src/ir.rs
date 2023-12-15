pub struct BlocklyIR {
    pub ty: String,
    pub components: Vec<BlocklyIRComponent>,
}

impl BlocklyIR {
    pub fn new<T: Into<String>>(ty: T, components: Vec<BlocklyIRComponent>) -> Self {
        BlocklyIR {
            ty: ty.into(),
            components,
        }
    }
}

pub enum BlocklyIRComponent {
    Text {
        title: String,
    },
    Variable {
        title: String,
    },
    TextInput {
        title: String,
    },
    BlockInput {
        title: String,
    }
}
