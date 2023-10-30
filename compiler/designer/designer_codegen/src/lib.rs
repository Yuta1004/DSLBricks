use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};

use design::DSLDesign;

#[allow(non_snake_case)]
#[derive(Serialize)]
struct CodeTemplate {
    NAME: String,
    TOKEN_DEFS: String,
    SYNTAX_DEFS: String,
    BNF: String,
}

pub fn gen(design: Box<dyn DSLDesign>) -> anyhow::Result<String> {
    let name = format!("{:?}", design);
    let token_defs = gen_token_code()?;
    let (syntax_defs, bnf) = gen_syntax_code()?;
    let context = CodeTemplate {
        NAME: name,
        TOKEN_DEFS: token_defs,
        SYNTAX_DEFS: syntax_defs,
        BNF: bnf,
    };

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("Template", include_str!("../template/template.txt"))?;

    Ok(tt.render("Template", &context)?)
}

fn gen_token_code() -> anyhow::Result<String> {
    Ok("".to_string())
}

fn gen_syntax_code() -> anyhow::Result<(String, String)> {  // (syntax_defs, bnf)
    Ok(("".to_string(), "".to_string()))
}
