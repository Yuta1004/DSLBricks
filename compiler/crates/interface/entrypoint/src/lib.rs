mod r#impl;

use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn build(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    r#impl::build::build_attr_macro_impl(ast).into()
}

#[proc_macro_attribute]
pub fn main(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    r#impl::main::main_attr_macro_impl(ast).into()
}
