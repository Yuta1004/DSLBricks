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

impl BlocklyIRComponent {
    pub fn new_text<T: Into<String>>(title: T) -> Self {
        BlocklyIRComponent::Text { title: title.into() }
    }

    pub fn new_variable<T: Into<String>>(title: T) -> Self {
        BlocklyIRComponent::Variable { title: title.into() }
    }

    pub fn new_text_input<T: Into<String>>(title: T) -> Self {
        BlocklyIRComponent::TextInput { title: title.into() }
    }

    pub fn new_block_input<T: Into<String>>(title: T) -> Self {
        BlocklyIRComponent::BlockInput { title: title.into() }
    }
}
