pub enum BlocklyIR {
    NoConnection(BlocklyIRBody),
    TopBottomConnections(BlocklyIRBody),
    TopConnection(BlocklyIRBody),
    BottomConnection(BlocklyIRBody),
}

pub struct BlocklyIRBody {
    pub ty: String,
    pub components: Vec<BlocklyIRComponent>,
}

impl BlocklyIR {
    pub fn new_no_connection<T: Into<String>>(ty: T, components: Vec<BlocklyIRComponent>) -> Self {
        let body = BlocklyIRBody {
            ty: ty.into(),
            components,
        };
        BlocklyIR::NoConnection(body)
    }

    pub fn new_top_bottom_connections<T: Into<String>>(ty: T, components: Vec<BlocklyIRComponent>) -> Self {
        let body = BlocklyIRBody {
            ty: ty.into(),
            components,
        };
        BlocklyIR::TopBottomConnections(body)
    }

    pub fn new_top_connection<T: Into<String>>(ty: T, components: Vec<BlocklyIRComponent>) -> Self {
        let body = BlocklyIRBody {
            ty: ty.into(),
            components,
        };
        BlocklyIR::TopConnection(body)
    }

    pub fn new_bottom_connection<T: Into<String>>(ty: T, components: Vec<BlocklyIRComponent>) -> Self {
        let body = BlocklyIRBody {
            ty: ty.into(),
            components,
        };
        BlocklyIR::BottomConnection(body)
    }
}

pub enum BlocklyIRComponent {
    Text {
        title: String,
    },
    Variable {
        title: String,
        name: String,
    },
    TextInput {
        title: String,
        name: String,
    },
    BlockInput {
        title: String,
        name: String,
    }
}

impl BlocklyIRComponent {
    pub fn new_text<T: Into<String>>(title: T) -> Self {
        BlocklyIRComponent::Text { title: title.into() }
    }

    pub fn new_variable<T, U>(title: T, name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        BlocklyIRComponent::Variable {
            title: title.into(),
            name: name.into()
        }
    }

    pub fn new_text_input<T, U>(title: T, name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        BlocklyIRComponent::TextInput {
            title: title.into(),
            name: name.into(),
        }
    }

    pub fn new_block_input<T, U>(title: T, name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        BlocklyIRComponent::BlockInput {
            title: title.into(),
            name: name.into(),
        }
    }
}
