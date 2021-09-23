#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod json;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(json::stage())
}
