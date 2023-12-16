pub mod __export {
    pub use blockly_front::ir::{BlocklyIR, BlocklyIRComponent};
}

#[macro_export]
macro_rules! blockly_ir {
    (
        [Base]
        Kind: $kind:ident,
        Type: $type:expr,
        [Components]
        $($name:ident : $($arg:expr)+),* $(,)?
    ) => {{
        use $crate::__export::*;

        let mut components = vec![];
        $(blockly_ir!(@ components += $name $($arg)+));*;

        match stringify!($kind) {
            "no_connection" => BlocklyIR::new_no_connection($type, components),
            "top_bottom_connections" => BlocklyIR::new_top_bottom_connections($type, components),
            "top_connection" => BlocklyIR::new_top_connection($type, components),
            "bottom_connection" => BlocklyIR::new_bottom_connection($type, components),
            _ => unimplemented!(),
        }
    }};

    (@ $vec:ident += Text $text:expr) => {
        $vec.push(BlocklyIRComponent::new_text($text))
    };

    (@ $vec:ident += Variable $text:expr) => {
        $vec.push(BlocklyIRComponent::new_variable($text, $text))
    };

    (@ $vec:ident += TextInput $text:expr) => {
        $vec.push(BlocklyIRComponent::new_text_input($text, $text))
    };

    (@ $vec:ident += BlockInput $text:expr) => {
        $vec.push(BlocklyIRComponent::new_block_input($text, $text))
    };

    (@ $vec:ident += BlockInputs $text_list:expr) => {{
        $vec.append(&mut $text_list
            .iter()
            .map(|text| BlocklyIRComponent::new_block_input(*text, *text))
            .collect::<Vec<BlocklyIRComponent>>()
        )
    }};
}