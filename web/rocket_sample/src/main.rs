#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;

mod json;
mod todo;

#[cfg(test)]
mod tests;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}
#[get("/sample")]
fn sample() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, sample, todo::index])
        .attach(Template::fairing())
        .attach(json::stage())
}
