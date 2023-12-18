use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../../target/doc"]
pub struct Document;
