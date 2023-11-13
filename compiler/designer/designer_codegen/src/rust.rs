use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};

use design::{DSLDesign, DSLGeneratable};

#[allow(non_snake_case)]
#[derive(Serialize)]
struct CodeTemplate {
    NAME: String,
    TOKEN_DEFS: String,
    SYNTAX_DEFS: String,
    BNF: String,
}

pub fn rust(dsl: impl DSLGeneratable) -> anyhow::Result<String> {
    let dsl = DSLDesign::from(dsl);

    let name = format!("{}", dsl.name);
    let token_defs = gen_token_code(&dsl.token_defs())?;
    let syntax_defs = gen_syntax_code(&dsl.syntax_defs())?;
    let bnf = dsl.bnf();
    let context = CodeTemplate {
        NAME: name,
        TOKEN_DEFS: token_defs,
        SYNTAX_DEFS: syntax_defs,
        BNF: bnf,
    };

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("Rust", include_str!("../template/Rust.txt"))?;

    Ok(tt.render("Rust", &context)?)
}

fn gen_token_code(token_defs: &Vec<&str>) -> anyhow::Result<String> {
    let token_variants_code = token_defs
        .iter()
        .map(|token| format!("#[token(regex=\"{}\")]\n{},", token, token))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(token_variants_code)
}

fn gen_syntax_code(syntax_defs: &Vec<String>) -> anyhow::Result<String> {
    let syntax_variants_code = syntax_defs
        .iter()
        .map(|name| format!("{},", name))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(syntax_variants_code)
}
