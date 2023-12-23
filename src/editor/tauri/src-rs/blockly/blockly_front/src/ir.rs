pub enum Block {
    NoConnection(BlockBody),
    TopBottomConnections(BlockBody),
    TopConnection(BlockBody),
    BottomConnection(BlockBody),
}

pub struct BlockBody {
    pub ty: String,
    pub components: Vec<BlockComponent>,
}

impl Block {
    pub fn new_no_connection<T: Into<String>>(ty: T, components: Vec<BlockComponent>) -> Self {
        let body = BlockBody {
            ty: ty.into(),
            components,
        };
        Block::NoConnection(body)
    }

    pub fn new_top_bottom_connections<T: Into<String>>(ty: T, components: Vec<BlockComponent>) -> Self {
        let body = BlockBody {
            ty: ty.into(),
            components,
        };
        Block::TopBottomConnections(body)
    }

    pub fn new_top_connection<T: Into<String>>(ty: T, components: Vec<BlockComponent>) -> Self {
        let body = BlockBody {
            ty: ty.into(),
            components,
        };
        Block::TopConnection(body)
    }

    pub fn new_bottom_connection<T: Into<String>>(ty: T, components: Vec<BlockComponent>) -> Self {
        let body = BlockBody {
            ty: ty.into(),
            components,
        };
        Block::BottomConnection(body)
    }
}

pub enum BlockComponent {
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
    },
    CheckBoxInput {
        title: String,
        name: String,
    },
}

impl BlockComponent {
    pub fn new_text<T: Into<String>>(title: T) -> Self {
        BlockComponent::Text { title: title.into() }
    }

    pub fn new_variable<T, U>(title: T, name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        BlockComponent::Variable {
            title: title.into(),
            name: name.into()
        }
    }

    pub fn new_text_input<T, U>(title: T, name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        BlockComponent::TextInput {
            title: title.into(),
            name: name.into(),
        }
    }

    pub fn new_block_input<T, U>(title: T, name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        BlockComponent::BlockInput {
            title: title.into(),
            name: name.into(),
        }
    }

    pub fn new_checkbox_input<T, U>(title: T, name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        BlockComponent::CheckBoxInput {
            title: title.into(),
            name: name.into(),
        }
    }
}
