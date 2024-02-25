mod parser;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn parser(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = attrs.to_string();
    let target_ast = parse_macro_input!(input as DeriveInput);
    parser::parser_attr_macro_impl(attrs, target_ast).into()
}
