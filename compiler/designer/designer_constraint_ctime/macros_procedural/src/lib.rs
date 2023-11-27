use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn impl_constraints(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: TokenStream = args.into();
    let input = parse_macro_input!(input as DeriveInput);

    let impls = args
        .to_string()
        .split(",")
        .map(|constraint| {
            let target = &input.ident;
            let constraint: TokenStream = constraint.parse().unwrap();
            quote! { impl #constraint for #target {} }
        })
        .collect::<Vec<TokenStream>>();

    quote! { #input #(#impls)* }.into()
}
