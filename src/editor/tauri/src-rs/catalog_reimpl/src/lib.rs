use std::collections::HashMap;

use blockly::ir::BlocklyIR;
use catalog::primitive::number::{Integer, Float};
use catalog::prelude::*;

macro_rules! blockly_ir {
    ($brick:ident) => {{
        let brick = $brick::default();
        BlocklyIR::new(
            DSLBrickMeta::name(&brick),
            vec![],
        )
    }};
}

pub fn catalog() -> HashMap<&'static str, Vec<BlocklyIR>> {
    let mut catalog = HashMap::new();

    catalog.insert(
        "Primitive",
        vec![
            blockly_ir!(Integer),
            blockly_ir!(Float),
        ]
    );

    catalog
}
