use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, ItemFn};

enum InitArgument<'a> {
    NameSpace(&'a str, &'a str), // struct_name, namespace
    Property(&'a str, &'a str),  // struct_name, property
}

impl<'a> From<InitArgument<'a>> for TokenStream {
    fn from(arg: InitArgument<'a>) -> TokenStream {
        match arg {
            InitArgument::NameSpace(struct_name, namespace) => {
                let start = struct_name;
                let struct_name: TokenStream = struct_name.parse().unwrap();
                let fullname = format!("{}.{}", namespace, struct_name);

                quote! {
                    impl DSLBrickMeta for #struct_name {
                        fn name(&self) -> &'static str {
                            #fullname
                        }

                        fn start(&self) -> &'static str {
                            #start
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

pub(super) fn dsl_brick_attr_macro_impl(args: TokenStream, ast: DeriveInput) -> TokenStream {
    let struct_namet = &ast.ident;
    let struct_name = struct_namet.to_string();

    let impls = args
        .to_string()
        .split(',')
        .flat_map(|arg| {
            let arg = arg.split('=').collect::<Vec<&str>>();
            let left = arg[0].trim();
            let right = arg[1].trim();

            match left {
                "namespace" => vec![InitArgument::NameSpace(&struct_name, right)],
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

pub(super) fn dsl_brick_builder_proc_macro_impl(ast: DeriveInput) -> TokenStream {
    let data_struct = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("\"DSLBrickBuilder proc-macro is only implemented for struct.\"");
    };

    let struct_name = ast.ident;

    let struct_fields = data_struct
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
                let constraints = right;

                Field {
                    is_multiple,
                    name: field.ident.to_token_stream().to_string(),
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

pub(super) fn dsl_brick_baker_proc_macro_impl(ast: DeriveInput) -> TokenStream {
    if let Data::Struct(_) = ast.data {
        // do nothing
    } else {
        panic!("\"DSLBrickBaker proc-macro is only implemented for struct.\"");
    };

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

pub(super) fn combine_brick_attr_macro_impl(ast: ItemFn) -> TokenStream {
    quote! { #ast }
}
