#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket_dyn_templates::tera;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

mod json;

#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![files])
        .attach(Template::fairing())
        .attach(json::stage())
}
