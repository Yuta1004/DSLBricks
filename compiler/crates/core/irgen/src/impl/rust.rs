use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};
use vfs::VfsPath;

use design::DSLDesign;

pub fn rust(design: DSLDesign, vfs: VfsPath) -> anyhow::Result<VfsPath> {
    let name = "DSL".to_string();
    let token_defs = gen_token_code(&design.ruleset.token_defs())?;
    let syntax_defs = gen_syntax_code(&design.ruleset.syntax_defs())?;
    let bnf = String::from(&design.ruleset);
    let derive_serde = if cfg!(feature = "with-serde") {
        "#[derive(Serialize, Deserialize)]".to_string()
    } else {
        "".to_string()
    };
    let context = CodeTemplate {
        NAME: name,
        TOKEN_DEFS: token_defs,
        SYNTAX_DEFS: syntax_defs,
        BNF: bnf,
        DERIVE_SERDE: derive_serde,
    };

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("Rust", include_str!("../../template/Rust.txt"))?;

    let code = tt.render("Rust", &context)?;
    vfs.join("DSL.rs")?
        .create_file()?
        .write_all(code.as_bytes())?;

    Ok(vfs)
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct CodeTemplate {
    NAME: String,
    TOKEN_DEFS: String,
    SYNTAX_DEFS: String,
    BNF: String,
    DERIVE_SERDE: String,
}

fn gen_token_code(token_defs: &[(&String, &str)]) -> anyhow::Result<String> {
    let token_variants_code = token_defs
        .iter()
        .map(|(id, regex)| format!("#[token(regex=r\"{}\")]\n{},", regex, id))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(token_variants_code)
}

fn gen_syntax_code(syntax_defs: &[String]) -> anyhow::Result<String> {
    let syntax_variants_code = syntax_defs
        .iter()
        .map(|name| format!("{},", name))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(syntax_variants_code)
}
