mod r#impl;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DSLBlockBuilder, attributes(component))]
pub fn derive_dsl_block_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::dsl_block_builder_proc_macro_impl(ast).into()
}
