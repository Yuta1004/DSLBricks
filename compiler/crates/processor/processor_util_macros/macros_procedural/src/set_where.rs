use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn set_where(args: TokenStream, ast: TokenStream) -> TokenStream {
    let (cfg_cond, trait_bounds) = parse_args(args);
    let rebuilded_ast = rebuild_tokenstream(trait_bounds, ast.clone());

    quote! {
        #[cfg(#cfg_cond)]
        #rebuilded_ast

        #[cfg(not(#cfg_cond))]
        #ast
    }
}

fn parse_args(ast: TokenStream) -> (TokenStream, TokenStream) {
    let ast_s = ast.to_string();
    let mut args = ast_s.split(',');

    let cfg_cond: TokenStream = args.next().unwrap().parse().unwrap();
    let trait_bounds: TokenStream = args.collect::<Vec<&str>>().join(",").parse().unwrap();

    (quote!(#cfg_cond), quote!(#trait_bounds))
}

fn rebuild_tokenstream(trait_bounds: TokenStream, ast: TokenStream) -> TokenStream {
    let mut ast = ast.into_iter();

    // Part 1 : pub trait <Ident>
    let mut ast_heads = vec![];
    let (mut where_t, mut block_t) = (None, None);
    for token in ast.by_ref() {
        let token_s = token.clone().span().source_text().unwrap();
        if token_s.starts_with("where") {
            where_t = Some(token);
            break;
        }
        if token_s.starts_with('{') {
            block_t = Some(token);
            break;
        }
        ast_heads.push(token);
    }
    let ast_heads = ast_heads.into_iter().collect::<TokenStream>();

    // Part 2 : where ...
    let ast_bounds = quote! { where #trait_bounds, };

    // Part 3 : { ... }
    let ast_body = if where_t.is_some() {
        ast.collect::<TokenStream>()
    } else if let Some(block) = block_t {
        block.into()
    } else {
        quote! {}
    };

    quote! { #ast_heads #ast_bounds #ast_body }
}
