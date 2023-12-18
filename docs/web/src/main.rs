use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;

fn main() -> anyhow::Result<()> {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5555)
        .finalize()?;

    rocket::custom(config)
        .mount("/", StaticFiles::from("./target/doc/"))
        .launch();

    Ok(())
}
