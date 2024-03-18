use proc_macro2::TokenStream;
use syn::ItemFn;
use quote::quote;
use quote::ToTokens;

pub fn main_attr_macro_impl(ast: ItemFn) -> TokenStream {
    let sig = ast.sig;
    let block = ast.block;

    let fn_keyword = sig.fn_token;
    let fn_ret_type = sig.output;

    let dsl_letname = sig.inputs[0].to_token_stream().to_string();
    let dsl_letname = dsl_letname
        .split(':')
        .next()
        .unwrap()
        .parse::<TokenStream>()
        .unwrap();

    quote! {
        include!(concat!(env!("OUT_DIR"), "/DSL.rs"));

        #fn_keyword main () #fn_ret_type {
            let #dsl_letname = DSL::gen().unwrap();
            {
                #block
            }
        }
    }
}
