use std::io::Cursor;
use std::path::PathBuf;

use rocket::config::Config;
use rocket::{Request, Response};
use rocket::response::Responder;
use rocket::http::ContentType;
use rocket::{get, routes, launch};

use embed::Document;

struct StaticFileResponse(ContentType, String);

impl From<(ContentType, String)> for StaticFileResponse {
    fn from((content_type, content_body): (ContentType, String)) -> Self {
        StaticFileResponse(content_type, content_body)
    }
}

impl<'r> Responder<'r, 'static> for StaticFileResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let StaticFileResponse (content_type, content_body) = self;
        Response::build()
            .header(content_type)
            .raw_header("X-Frame-Options","ALLOW-FROM *")
            .sized_body(content_body.len(), Cursor::new(content_body))
            .ok()
    }
}

#[get("/<path..>")]
fn static_file<'r>(path: PathBuf) -> impl Responder<'r, 'static> {
    let path = path.to_str().unwrap();
    let ext = path.split('.').last().unwrap();
    let file = match Document::get(path) {
        Some(file) => file,
        None => panic!("Not found"),
    };

    let content_type = ContentType::from_extension(ext).unwrap();
    let data = file.data.as_ref();
    let data = std::str::from_utf8(data).unwrap().to_string();

    StaticFileResponse::from((content_type, data))
}

#[launch]
fn rocket() -> _ {
    let config = Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 5555));

    rocket::build()
        .configure(config)
        .mount("/", routes![static_file])
}
