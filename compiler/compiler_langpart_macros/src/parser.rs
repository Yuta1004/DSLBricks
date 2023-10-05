use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(super) fn parser_attr_macro_impl(attrs: String, target_ast: DeriveInput) -> TokenStream {
    let (parser, token, semantics, rule_table) = parse_attr(attrs);

    let enum_name = &target_ast.ident;
    let data_enum = if let Data::Enum(data_enum) = &target_ast.data {
        data_enum
    } else {
        panic!("\"Tokenize\" proc-macro is only implemented for enum.")
    };

    let mut enum_rule_table = vec![];
    for variant in &data_enum.variants {
        let variant = &variant.ident;
        if let Some(rule) = rule_table.get(&variant.to_string()) {
            enum_rule_table.push(quote! { #enum_name :: #variant => #rule });
        } else {
            panic!("Variant \"{}\" is not mapping for any rule.", variant);
        }
    }

    quote! {
        #[derive(EnumIter, Clone, Copy)]
        #target_ast

        impl Syntax<#semantics, #token> for #enum_name {
            type Parser = #parser <#semantics, #enum_name, #token>;

            fn to_rule(&self) -> Rule<#token> {
                match self {
                    #( #enum_rule_table, )*
                }
            }
        }
    }
}

fn parse_attr(
    attrs: String,
) -> (
    TokenStream,
    TokenStream,
    TokenStream,
    HashMap<String, TokenStream>,
) {
    let mut parser = quote! {};
    let mut token = quote! {};
    let mut semantics = quote! {};
    let mut rule_table = HashMap::new();
    for attr in attrs.to_string().split(',') {
        let attr: Vec<&str> = attr.split("=>").collect();
        let (name, value) = (attr[0].trim(), attr[1..].join(""));
        match name {
            "kind" => parser = value.trim().parse().unwrap(),
            "token" => token = value.trim().parse().unwrap(),
            "semantics" => semantics = value.trim().parse().unwrap(),
            "bnf" => rule_table = parse_bnf(&token.to_string(), value),
            _ => {}
        }
    }

    (parser, token, semantics, rule_table)
}

fn parse_bnf(token: &str, bnf: String) -> HashMap<String, TokenStream> {
    let rules_list: Vec<&str> = bnf.trim().split(';').collect();
    rules_list[..rules_list.len() - 1]
        .into_iter()
        .flat_map(|rules| parse_bnf_rules(token, rules))
        .collect()
}

fn parse_bnf_rules(token: &str, rules: &str) -> HashMap<String, TokenStream> {
    let rules: Vec<&str> = rules.trim().split(':').collect();
    if rules.len() < 2 {
        panic!("BNF Syntax(Rule) => LEFT : RIGHT1 | RIGHT2 | .. | RIGHTn ;");
    }

    let (left, rights) = (rules[0].trim(), rules[1..].join(""));
    let left = quote! { #left };
    rights
        .split('|')
        .into_iter()
        .map(|rule| {
            let (variant, relems) = parse_bnf_rule(token, rule);
            let rule = quote! {
                Rule::from((
                    RuleElem::nonterm(#left),
                    vec![ #( #relems, )* ]
                ))
            };
            (variant.to_string(), rule)
        })
        .collect()
}

fn parse_bnf_rule<'a, 'b>(token: &'a str, rule: &'b str) -> (&'b str, Vec<TokenStream>) {
    let token: TokenStream = token.parse().unwrap();
    let rule: Vec<&str> = rule.trim().split('$').collect();
    if rule.len() < 2 {
        panic!("BNF Syntax(Right) => ELEM1 ELEM2 .. ELEMn $ Variant");
    }

    let (rule, variant) = (rule[0], rule[1]);
    let relems = rule
        .split([' ', '\n'])
        .into_iter()
        .filter(|relem| relem.len() != 0)
        .map(|relem| {
            let relem = relem.trim();
            if relem.starts_with("\"") {
                let relem = &relem[1..relem.len() - 1];
                let relem: TokenStream = relem.parse().unwrap();
                quote! { RuleElem::term(#token :: #relem) }
            } else {
                quote! { RuleElem::nonterm(#relem) }
            }
        })
        .collect();

    (variant, relems)
}
