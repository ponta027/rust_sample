use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Duration, Local};
use log::info;

use serde::Deserialize;
mod data;

#[get("/posts")]
async fn index() -> impl Responder {
    info!("Called Index");
    let posts = data::get_all();
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    for item in &posts {
        body_str += &format!("<div><a href=\"/posts/{}\">", item.id);
        body_str += &format!("<div>{} {}</div>", item.sender, item.posted);
        body_str += &format!("<div><p>{} </p></div>", item.content.replace("\n", "<br/>"));
        body_str += &format!("</a</div>");
    }
    body_str += "<div><a href=\"/posts/new\">作成</a></div>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/new")]
pub async fn new() -> impl Responder {
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += include_str!("../static/form.html");
    body_str += include_str!("../static/footer.html");
    body_str = body_str.replace("{{action}}", "create");
    body_str = body_str.replace("{{id}}", "0");
    body_str = body_str.replace("{{posted}}", "");
    body_str = body_str.replace("{{sender}}", "");
    body_str = body_str.replace("{{content}}", "");
    body_str = body_str.replace("{{button}}", "登録");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/{id}")]
async fn show(info: web::Path<i32>) -> impl Responder {
    info!("Call show");
    let info = info.into_inner();
    let post = data::get(info.try_into().unwrap());
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += "<div>";
    if post.id != 0 {
        body_str += &format!("<div>投稿者：{}</div>", post.sender);
        body_str += &format!("<div>投稿日時：{}</div>", post.posted);
        body_str += &format!(
            "<div>投稿内容：<br />{}</div>",
            post.content.replace("\n", "<br />")
        );
        body_str += &format!("<div><a href=\"/posts/{}/edit\">編集</a> ", info);
        body_str += &format!("<a href=\"/posts/{}/delete\">削除</a><div>", info);
    } else {
        body_str += "見つかりません。";
    }
    body_str += "</div>";
    body_str += "<div><a href=\"/posts\">一覧へ</a></div>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[derive(Deserialize, Debug)]
pub struct CreateForm {
    id: i32,
    posted: String,
    sender: String,
    content: String,
}
#[post("/posts/create")]
pub async fn create(params: web::Form<CreateForm>) -> impl Responder {
    let now: DateTime<Local> = Local::now();
    let mut message = data::Message {
        id: 0,
        posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    message = data::create(message);
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
    //
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page Not found!")
}
