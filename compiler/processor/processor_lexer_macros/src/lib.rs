mod lexer;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn lexer(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: TokenStream = input.into();
    lexer::lexer_attr_macro_impl(ast).into()
}

#[proc_macro_derive(Tokenize, attributes(token))]
pub fn derive_tokenize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    lexer::tokenize_proc_macro_impl(ast).into()
}
