use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{ItemFn, Stmt, Local, Expr};

enum DSLParts {
    Setup(TokenStream, TokenStream),                     // new, method-chains
    SetupWithLet(TokenStream, TokenStream, TokenStream), // ident, new, method-chains
    RawCode(TokenStream),
}

impl From<Local> for DSLParts {
    fn from(local: Local) -> Self {
        if let Some((_, right)) = &local.init {
            let ident = local.pat.into_token_stream();
            match DSLParts::from(*right.to_owned()) {
                DSLParts::Setup(new, methods) => DSLParts::SetupWithLet(ident, new, methods),
                DSLParts::RawCode(code) => DSLParts::RawCode(quote! { let #ident = #code; }),
                _ => unimplemented!(),
            }
        } else {
            DSLParts::RawCode(local.to_token_stream())
        }
    }
}

impl From<Expr> for DSLParts {
    fn from(expr: Expr) -> Self {
        if let Expr::Struct(expr_struct) = expr {
            let struct_ident = expr_struct.path.get_ident().unwrap();
            let struct_setters = expr_struct
                .fields
                .iter()
                .map(|field| {
                    match &field.expr {
                        Expr::Path(lit) => {
                            let fname = format_ident!("set_{}", field.member);
                            let expr = lit.path.to_token_stream();
                            quote! { .#fname(Rc::clone(&#expr)) }
                        }
                        Expr::Array(array) => {
                            let fname = format_ident!("add_{}", field.member);
                            let exprs = &array.elems.iter().collect::<Vec<&Expr>>();
                            quote! { #( .#fname(Rc::clone(&#exprs)) )* }
                        }
                        _ => quote! {},
                    }
                });

            DSLParts::Setup(
                quote! { #struct_ident::new(); },
                quote! { #( #struct_setters )*; },
            )
        } else {
            DSLParts::RawCode(expr.into_token_stream())
        }
    }
}

impl From<Stmt> for DSLParts {
    fn from(stmt: Stmt) -> Self {
        DSLParts::RawCode(stmt.to_token_stream())
    }
}

impl FromIterator<DSLParts> for TokenStream {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = DSLParts>,
    {
        let (mut pre, mut body) = (vec![], vec![]);
        for part in iter.into_iter() {
            match part {
                DSLParts::Setup(new, methods) => {
                    pre.push(quote! { let __product = #new; });
                    body.push(quote! { let __product = __product #methods; });
                }
                DSLParts::SetupWithLet(ident, new, methods) => {
                    pre.push(quote! { let #ident = #new; });
                    body.push(quote! { let #ident = #ident #methods;});
                },
                DSLParts::RawCode(stmt) => body.push(stmt),
            }
        }
        pre.extend(body);
        pre.into_iter().collect::<TokenStream>()
    }
}

pub(super) fn combine_brick_attr_macro_impl(ast: ItemFn) -> TokenStream {
    let sig = ast.sig;
    let block = ast.block.stmts;

    let parts = block
        .into_iter()
        .map(|stmt| {
            match stmt {
                Stmt::Expr(expr) => DSLParts::from(expr),
                Stmt::Local(local) => DSLParts::from(local),
                _ => DSLParts::from(stmt),
            }
        });

    let block = parts
        .into_iter()
        .collect::<TokenStream>();

    quote! {
        #sig {
            fn __inner() -> impl DSLGeneratable {
                { #block __product }.unwrap()
            }
            build_dsl!(__inner())
        }
    }
}
