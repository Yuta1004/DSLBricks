mod set_where;

#[proc_macro_attribute]
pub fn cfg_where(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = args.into();
    let ast = input.into();
    set_where::set_where(args, ast).into()
}
