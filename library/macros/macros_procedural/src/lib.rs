mod r#impl;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn dslblock(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = args.into();
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::dsl_block_attr_macro_impl(args, ast).into()
}

#[proc_macro_derive(DSLBlockBuilder, attributes(component))]
pub fn derive_dsl_block_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::dsl_block_builder_proc_macro_impl(ast).into()
}
