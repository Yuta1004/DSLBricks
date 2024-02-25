use blockly::front::ir;
use blockly::front::macros::block;
use catalog::statement::c::If;
use catalog::primitive::number::integer::DecimalInteger;
use catalog::primitive::number::fraction::DecimalFraction;
use catalog::prelude::*;

pub fn catalog() -> ir::ToolBox {
    let default = ir::ToolBox::new("Default")
        .push_block(
            block! {
                [Base]
                Kind: top_bottom_connections,
                Type: "brick",

                [Components]
                Variable: "DSLBrick",
            }
        );

    let primitive = ir::ToolBox::new("Primitive")
        .push_block(irgen::<DecimalInteger>())
        .push_block(irgen::<DecimalFraction>());

    let statement = ir::ToolBox::new("Statement")
        .push_block(irgen::<If>());

    ir::ToolBox::new_root()
        .push_toolbox(default)
        .push_toolbox(primitive)
        .push_toolbox(statement)
}

fn irgen<T>() -> ir::Block
where
    T: Default + DSLBrick,
{
    let brick = T::default();
    block! {
        [Base]
        Kind: no_connection,
        Type: DSLBrickMeta::name(&brick),

        [Components]
        Text: DSLBrickMeta::start(&brick),
        CheckBoxInput: "root",
        Variable: "variable",
        TextInput: "property",
        BlockInputs: DSLBrickMeta::components(&brick),
    }
}
