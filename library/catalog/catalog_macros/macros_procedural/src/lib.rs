mod r#impl;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, ItemStruct, ItemFn};

#[proc_macro_attribute]
pub fn dslbrick(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = args.into();
    let ast = parse_macro_input!(input as ItemStruct);
    r#impl::dslbrick_attr_macro_impl(args, ast).into()
}

#[proc_macro_derive(DSLBrickBuilder, attributes(component))]
pub fn derive_dslbrick_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    r#impl::dslbrick_builder_proc_macro_impl(ast).into()
}

#[proc_macro_derive(DSLBrickBaker)]
pub fn derive_dslbrick_baker(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    r#impl::dslbrick_baker_proc_macro_impl(ast).into()
}

#[proc_macro_attribute]
pub fn combine_bricks(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
)  -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    r#impl::combine_brick_attr_macro_impl(ast).into()
}
