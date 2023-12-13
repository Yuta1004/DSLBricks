mod r#impl;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn dslbrick(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = args.into();
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::dsl_brick_attr_macro_impl(args, ast).into()
}

#[proc_macro_derive(DSLBrickBuilder, attributes(component))]
pub fn derive_dsl_brick_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    r#impl::dsl_brick_builder_proc_macro_impl(ast).into()
}
