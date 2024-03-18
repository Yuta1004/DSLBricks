use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::ItemFn;

pub fn build_attr_macro_impl(ast: ItemFn) -> TokenStream {
    let sig = ast.sig;
    let attrs = ast.attrs.iter().fold(quote! {}, |attrs, attr| {
        let attr = attr.to_token_stream();
        quote! { #attrs #attr }
    });
    let block = ast.block;

    let fn_keyword = sig.fn_token;
    let fn_ret_type = sig.output;

    quote! {
        #fn_keyword main () {
            #attrs
            fn __inner() #fn_ret_type #block

            let root_brick = DSLBrick::into(__inner());
            let dsl_code = codegen::rust(root_brick).unwrap();
            let out_dir = std::env::var_os("OUT_DIR").unwrap();
            let dst_path = std::path::Path::new(&out_dir).join("DSL.rs");
            std::fs::write(&dst_path, dsl_code).unwrap();
        }
    }
}
