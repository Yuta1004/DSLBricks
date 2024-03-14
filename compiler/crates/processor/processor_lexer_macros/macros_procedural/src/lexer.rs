use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, ExprLit, Lit};

pub(super) fn lexer_attr_macro_impl(ast: TokenStream) -> TokenStream {
    let extra_derives = if cfg!(feature = "with-serde") {
        quote! { Serialize, Deserialize }
    } else {
        quote! {}
    };

    quote! {
        #[derive(Tokenize, Clone, Copy, Hash, PartialEq, Eq, #extra_derives)]
        #ast
    }
}

pub(super) fn tokenize_proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_enum = if let Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("\"Tokenize\" proc-macro is only implemented for enum.")
    };

    let enum_name = ast.ident;

    let enum_variants = data_enum.variants.clone().into_iter().map(|variant| {
        let variant = variant.ident.clone();
        quote! { #enum_name :: #variant }
    });

    let enum_regex_table: Vec<TokenStream> = data_enum
        .variants
        .into_iter()
        .map(|variant| {
            let mut regex = "".to_string();
            for attr in variant.attrs {
                let arg = attr.parse_args::<syn::Meta>().unwrap();
                let arg = arg.require_name_value().unwrap();
                if arg.path.is_ident("regex") {
                    if let syn::Expr::Lit(ExprLit {
                        lit: Lit::Str(litstr),
                        ..
                    }) = &arg.value
                    {
                        let s = litstr.token().to_string();
                        regex = if s.starts_with("r\"") {
                            format!("^{}", &s[2..s.len() - 1])
                        } else {
                            format!("^{}", &s[1..s.len() - 1])
                        };
                        break;
                    }
                } else {
                    panic!("\"token\" atrribute must contain \"regex\" argument.");
                }
            }
            if regex.is_empty() {
                panic!("All variants must have \"token\" attribute.");
            }

            let variant = variant.ident;

            quote! { #enum_name :: #variant => #regex }
        })
        .collect();

    quote! {
        impl TokenSet for #enum_name {
            fn iter() -> Box<dyn Iterator<Item = Self>> {
                Box::new(vec![
                    #( #enum_variants, )*
                ].into_iter())
            }

            fn to_regex(token: &Self) -> &'static str {
                match token {
                    #( #enum_regex_table, )*
                    _ => unimplemented!(),
                }
            }

            fn ignore_str() -> &'static str {
                "^([ \t\n])+"
            }
        }
    }
}
