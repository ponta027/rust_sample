use rocket::http::ContentType;

use rocket_dyn_templates::tera;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

/*+
 * see.
 * https://api.rocket.rs/v0.5-rc/rocket/response/content/index.html
 * https://github.com/magiclen/rocket-include-tera/blob/master/examples/hello_world_pages.rs
 * */
#[get("/todo")]
pub fn index() -> (ContentType, Template) {
    let mut context = HashMap::new();
    context.insert("title", "Title");
    context.insert("body", "Hello, world!");
    (ContentType::HTML, Template::render("index", &context))
}
