#[macro_use]
extern crate rocket;

mod routes;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index])
}
