#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;

mod todo;

#[cfg(test)]
mod tests;

mod json;

<<<<<<< HEAD
mod routes;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index])
=======
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
>>>>>>> 2e7b4116e4215efc333fb7a23dbd9742abe37912
}
