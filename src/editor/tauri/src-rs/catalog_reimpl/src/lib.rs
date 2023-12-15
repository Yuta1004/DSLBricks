use std::collections::HashMap;

use blockly::ir::{BlocklyIR, BlocklyIRComponent};
use catalog::primitive::number::{Integer, Float};
use catalog::prelude::*;

macro_rules! blockly_ir {
    ($brick:ident) => {{
        let brick = $brick::default();
        BlocklyIR::new(
            DSLBrickMeta::name(&brick),
            vec![
                BlocklyIRComponent::new_text(DSLBrickMeta::start(&brick)),
            ],
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
