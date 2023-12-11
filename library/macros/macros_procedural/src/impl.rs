use proc_macro2::TokenStream;
use quote::{quote, ToTokens, format_ident};
use syn::{DeriveInput, Data};

enum InitArgument<'a> {
    NameSpace(&'a str, &'a str),     // struct_name, namespace
    Property(&'a str, &'a str), // struct_name, property
}

impl<'a> Into<TokenStream> for InitArgument<'a> {
    fn into(self) -> TokenStream {
        match self {
            InitArgument::NameSpace(struct_name, namespace) => {
                let start = struct_name;
                let struct_name: TokenStream = struct_name.parse().unwrap();
                let fullname = format!("{}.{}", namespace, struct_name);

                quote! {
                    impl DSLGeneratable for #struct_name {
                        fn name(&self) -> &'static str {
                            #fullname
                        }

                        fn start(&self) -> &'static str {
                            #start
                        }

                        fn design(&self) -> RuleSet {
                            #struct_name::design(self)
                        }
                    }
                }
            },
            InitArgument::Property(struct_name, property) => {
                let struct_name: TokenStream = struct_name.parse().unwrap();
                let property: TokenStream = property.parse().unwrap();

                quote! { impl #property for #struct_name {} }
            },
        }
    }
}

pub(super) fn dsl_block_attr_macro_impl(args: TokenStream, ast: DeriveInput) -> TokenStream {
    let struct_namet = &ast.ident;
    let struct_name = struct_namet.to_string();

    let impls = args
        .to_string()
        .split(",")
        .flat_map(|arg| {
            let arg = arg.split("=").collect::<Vec<&str>>();
            let left = arg[0].trim();
            let right = arg[1].trim();

            match left {
                "namespace" => vec![InitArgument::NameSpace(&struct_name, right)],
                "property" => {
                    right.split("+")
                        .into_iter()
                        .map(|property| InitArgument::Property(&struct_name, property))
                        .collect::<Vec<InitArgument>>()
                },
                _ => panic!("Argument \"{}\" is not implemented.", left),
            }
        })
        .map(|arg| Into::<TokenStream>::into(arg))
        .collect::<Vec<TokenStream>>();

    quote! {
        #[derive(DSLBlockBuilder)]
        #ast
        #( #impls )*

        impl #struct_namet {
            pub fn new() -> Rc<Self>
            where
                Self: Default,
            {
                Rc::new(Self::default())
            }

            pub fn unwrap(self: Rc<Self>) -> Self {
                Rc::into_inner(self).unwrap()
            }
        }
    }
}

#[derive(Debug)]
struct Field {
    is_multiple: bool,
    name: String,
    constraints: TokenStream,
}

impl Into<TokenStream> for Field {
    fn into(self) -> TokenStream {
        match self.is_multiple {
            true => Field::into_multiple(self),
            false => Field::into_single(self),
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
                T: #constraints + 'static,
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
                T: #constraints + 'static,
            {
                self.#name
                    .borrow_mut()
                    .push(rule! { #name -> [#name] });
                self
            }
        }
    }
}

pub(super) fn dsl_block_builder_proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_struct = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("\"DSLBlockBuilder proc-macro is only implemented for struct.\"");
    };

    let struct_name = ast.ident;

    let struct_fields = data_struct
        .fields
        .clone()
        .into_iter()
        .map(|field| {
            let mut new_field = None;
            for attr in field.attrs {
                let arg = attr.parse_args::<syn::ExprAssign>().unwrap();
                let left = arg.left.into_token_stream();
                let right = arg.right.into_token_stream();

                let is_multiple = match left.to_string().as_str() {
                    "single" => false,
                    "multiple" => true,
                    _ => panic!("Argument \"{}\" is not supported in this implementation.", left),
                };
                let constraints = right;

                new_field = Some(Field {
                    is_multiple,
                    name: field.ident.to_token_stream().to_string(),
                    constraints,
                })
            }
            new_field.unwrap()
        })
        .map(|field| Into::<TokenStream>::into(field))
        .collect::<Vec<TokenStream>>();

    quote! {
        impl #struct_name {
            #( #struct_fields )*
        }
    }.into()
}
