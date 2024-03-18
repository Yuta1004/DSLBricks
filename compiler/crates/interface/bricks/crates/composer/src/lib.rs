mod r#impl;

use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn combine_bricks(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    r#impl::combine_brick_attr_macro_impl(ast).into()
}
