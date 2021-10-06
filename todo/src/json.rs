use std::borrow::Cow;

use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

// The type to represent the ID of a message.
type Id = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageList = Mutex<Vec<String>>;
type Messages<'r> = &'r State<MessageList>;
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    id: Option<Id>,
    message: Cow<'r, str>,
}

#[post("/", format = "json", data = "<message>")]
async fn new(message: Json<Message<'_>>, list: Messages<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    list.push(message.message.to_string());
    json!({ "status": "ok", "id": id })
}

#[put("/<id>", format = "json", data = "<message>")]
async fn update(id: Id, message: Json<Message<'_>>, list: Messages<'_>) -> Option<Value> {
    match list.lock().await.get_mut(id) {
        Some(existing) => {
            *existing = message.message.to_string();
            Some(json!({ "status": "ok" }))
        }
        None => None,
    }
}
#[get("/<id>/del", format = "json")]
async fn del(id: Id, list: Messages<'_>) -> Option<Value> {
    let mut list = list.lock().await;
    list.remove(id);
    Some(json!({ "status": "ok" }))
}

#[get("/<id>", format = "json")]
async fn get(id: Id, list: Messages<'_>) -> Option<Json<Message<'_>>> {
    let list = list.lock().await;

    Some(Json(Message {
        id: Some(id),
        message: list.get(id)?.to_string().into(),
    }))
}

#[get("/all", format = "json")]
async fn get_all(list: Messages<'_>) -> Option<Json<Vec<Message<'_>>>> {
    let list2 = list.lock().await;
    let mut cnt = 0;
    let mut data = Vec::new();

    for i in list2.clone() {
        println!("{:?}", i);
        data.push(Message {
            id: i.id,
            message: i.message.to_string().into(),
        });
        cnt = cnt + 1;
    }
    Some(Json(data))
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket
            .mount("/json", routes![new, update, get, get_all, del])
            .register("/json", catchers![not_found])
            .manage(MessageList::new(vec![]))
    })
}
