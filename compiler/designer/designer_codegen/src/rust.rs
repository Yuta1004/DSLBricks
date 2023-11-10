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
    let token_defs = gen_token_code(&dsl.tokens)?;
    let syntax_defs = gen_syntax_code()?;
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

fn gen_token_code(tokens: &Vec<&str>) -> anyhow::Result<String> {
    let token_code = tokens
        .iter()
        .map(|token| format!("#[token(regex=\"{}\")]\n{},", token, token))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(token_code)
}

fn gen_syntax_code() -> anyhow::Result<String> {
    Ok("".to_string())
}
