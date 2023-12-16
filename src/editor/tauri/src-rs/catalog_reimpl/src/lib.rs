use std::collections::HashMap;

use blockly::front::ir::BlocklyIR;
use blockly::front::macros::blockly_ir;
use catalog::statement::c::If;
use catalog::primitive::number::{Integer, Float};
use catalog::prelude::*;

pub fn catalog() -> HashMap<&'static str, Vec<BlocklyIR>> {
    let mut catalog = HashMap::new();

    catalog.insert(
        "Default",
        vec![
            blockly_ir! {
                [Base]
                Kind: top_bottom_connections,
                Type: "brick",

                [Components]
                Variable: "DSLBrick",
            }
        ]
    );

    catalog.insert(
        "Primitive",
        vec![
            irgen::<Integer>(),
            irgen::<Float>(),
        ]
    );

    catalog.insert(
        "Statement",
        vec![
            irgen::<If>(),
        ]
    );

    catalog
}

fn irgen<T>() -> BlocklyIR
where
    T: Default + DSLBrick,
{
    let brick = T::default();
    blockly_ir! {
        [Base]
        Kind: no_connection,
        Type: DSLBrickMeta::name(&brick),

        [Components]
        Text: DSLBrickMeta::start(&brick),
        Variable: "variable",
        TextInput: "property",
        BlockInputs: DSLBrickMeta::components(&brick),
    }
}
