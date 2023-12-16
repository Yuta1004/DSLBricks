pub mod __export {
    pub use blockly_front::ir::{BlocklyIR, BlocklyIRComponent};
}

#[macro_export]
macro_rules! blockly_ir {
    (
        [Base]
        Type: $type:expr,
        [Components]
        $($name:ident : $($arg:expr)+),* $(,)?
    ) => {{
        use $crate::__export::*;

        let mut components = vec![];
        $(blockly_ir!(@ components += $name $($arg)+));*;

        BlocklyIR::new($type, components)
    }};

    (@ $vec:ident += Text $text:expr) => {
        $vec.push(BlocklyIRComponent::new_text($text))
    };

    (@ $vec:ident += Variable $text:expr) => {
        $vec.push(BlocklyIRComponent::new_variable($text))
    };

    (@ $vec:ident += TextInput $text:expr) => {
        $vec.push(BlocklyIRComponent::new_text_input($text))
    };

    (@ $vec:ident += BlockInput $text:expr) => {
        $vec.push(BlocklyIRComponent::new_block_input($text))
    };

    (@ $vec:ident += BlockInputs $text_list:expr) => {{
        $vec.append(&mut $text_list
            .iter()
            .map(|text| BlocklyIRComponent::new_block_input(*text))
            .collect::<Vec<BlocklyIRComponent>>()
        )
    }};
}
