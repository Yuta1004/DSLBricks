use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{ItemFn, ItemStruct, Stmt, Local, Expr};

enum InitArgument<'a, 'b> {
    Meta(&'a str, &'b Vec<String>, &'a str), // struct_name, struct_fields, namespace
    Property(&'a str, &'a str),  // struct_name, property
}

impl<'a, 'b> From<InitArgument<'a, 'b>> for TokenStream {
    fn from(arg: InitArgument<'a, 'b>) -> TokenStream {
        match arg {
            InitArgument::Meta(struct_name, struct_fields, namespace) => {
                let start = struct_name;
                let struct_name: TokenStream = struct_name.parse().unwrap();
                let fullname = format!("{}.{}", namespace, struct_name);
                let struct_fields = struct_fields.join(",");

                quote! {
                    impl DSLBrickMeta for #struct_name {
                        fn name(&self) -> &'static str {
                            #fullname
                        }

                        fn start(&self) -> &'static str {
                            #start
                        }

                        fn components(&self) -> &[&'static str] {
                            &[#struct_fields]
                        }
                    }
                }
            }
            InitArgument::Property(struct_name, property) => {
                let struct_name: TokenStream = struct_name.parse().unwrap();
                let property: TokenStream = property.parse().unwrap();

                quote! { impl #property for #struct_name {} }
            }
        }
    }
}

pub(super) fn dslbrick_attr_macro_impl(args: TokenStream, ast: ItemStruct) -> TokenStream {
    let struct_namet = &ast.ident;
    let struct_name = struct_namet.to_string();

    let struct_fields = ast
        .fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string())
        .collect::<Vec<String>>();

    let impls = args
        .to_string()
        .split(',')
        .flat_map(|arg| {
            let arg = arg.split('=').collect::<Vec<&str>>();
            let left = arg[0].trim();
            let right = arg[1].trim();

            match left {
                "namespace" => vec![InitArgument::Meta(&struct_name, &struct_fields, right)],
                "property" => right
                    .split('+')
                    .map(|property| InitArgument::Property(&struct_name, property))
                    .collect::<Vec<InitArgument>>(),
                _ => panic!("Argument \"{}\" is not implemented.", left),
            }
        })
        .map(Into::<TokenStream>::into)
        .collect::<Vec<TokenStream>>();

    quote! {
        #[derive(DSLBrickBuilder, DSLBrickBaker)]
        #ast
        #( #impls )*
    }
}

#[derive(Debug)]
struct Field {
    is_multiple: bool,
    name: String,
    constraints: TokenStream,
}

impl From<Field> for TokenStream {
    fn from(field: Field) -> TokenStream {
        match field.is_multiple {
            true => Field::into_multiple(field),
            false => Field::into_single(field),
        }
    }
}

impl Field {
    fn into_single(self) -> TokenStream {
        let fname = format_ident!("set_{}", self.name);
        let rname = format_ident!("ref_{}", self.name);
        let name: TokenStream = self.name.parse().unwrap();
        let constraints = self.constraints;

        quote! {
            pub fn #fname<T>(self: Rc<Self>, #name: Rc<T>) -> Rc<Self>
            where
                T: DSLGeneratable + #constraints + 'static,
            {
                {
                    let #rname = &mut *self.#name.borrow_mut();
                    *#rname = Some(rule! { #name -> [#name] });
                }
                self
            }
        }
    }

    fn into_multiple(self) -> TokenStream {
        let fname = format_ident!("add_{}", self.name);
        let name: TokenStream = self.name.parse().unwrap();
        let constraints = self.constraints;

        quote! {
            pub fn #fname<T>(self: Rc<Self>, #name: Rc<T>) -> Rc<Self>
            where
                T: DSLGeneratable + #constraints + 'static,
            {
                self.#name
                    .borrow_mut()
                    .push(rule! { #name -> [#name] });
                self
            }
        }
    }
}

pub(super) fn dslbrick_builder_proc_macro_impl(ast: ItemStruct) -> TokenStream {
    let struct_name = ast.ident;

    let struct_fields = ast
        .fields
        .clone()
        .into_iter()
        .map(|field| {
            if let Some(attr) = field.attrs.get(0) {
                let arg = attr.parse_args::<syn::ExprAssign>().unwrap();
                let left = arg.left.into_token_stream();
                let right = arg.right.into_token_stream();

                let is_multiple = match left.to_string().as_str() {
                    "single" => false,
                    "multiple" => true,
                    _ => panic!(
                        "Argument \"{}\" is not supported in this implementation.",
                        left
                    ),
                };
                let name = field.ident.to_token_stream().to_string();
                let constraints = right;

                Field {
                    is_multiple,
                    name,
                    constraints,
                }
            } else {
                panic!("Argument \"single\" or \"multiple\" is missing.");
            }
        })
        .collect::<Vec<Field>>();

    let setters = struct_fields
        .into_iter()
        .map(Into::<TokenStream>::into);

    quote! {
        impl #struct_name {
            #( #setters )*
        }
    }
}

pub(super) fn dslbrick_baker_proc_macro_impl(ast: ItemStruct) -> TokenStream {
    let struct_name = ast.ident;

    quote! {
        impl DSLGeneratable for #struct_name
        where
            Self: DSLBrick,
        {
            fn name(&self) -> &'static str {
                DSLBrickMeta::name(self)
            }

            fn start(&self) -> &'static str {
                DSLBrickMeta::start(self)
            }

            fn design(&self) -> RuleSet {
                DSLBrickAssertion::assert(self);

                let name = DSLBrickMeta::name(self);
                let design = DSLBrickDesign::design(self);
                (name, design).into()
            }
        }
    }
}

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
