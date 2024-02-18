use actix_web::{get,  post, web,  HttpResponse,  Responder,error,Error};
use chrono::{DateTime,  Local};
use log::info;
use utoipa::ToSchema;

use serde::{Deserialize,Serialize};


pub mod data;

#[get("/posts")]
async fn index(tmpl: web::Data<tera::Tera>) -> Result<impl Responder ,Error>{
    info!("Called Index");
    let posts = data::get_all();
    let mut ctx = tera::Context::new();
    ctx.insert("posts",&posts);
    let s = tmpl.render("index.html",&ctx).map_err(|e|  error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html;charset=utf-8").body(s))

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub url_name: String
}

#[get("/posts/new")]
//pub async fn new(tmpl: web::Data<tera::Tera>) -> impl Responder {
pub async fn new(tmpl: web::Data<tera::Tera>) ->Result< impl Responder ,Error>{
    let mut ctx = tera::Context::new();
    let data=Project{
        id:1,
        name:String::from("test"),
        url_name:String::from("http://example.com"),
    };
    ctx.insert("action", "create");
    ctx.insert("id", "0");
    ctx.insert("posted", "");
    ctx.insert("sender", "");
    ctx.insert("content", "");
    ctx.insert("button", "登録");
    ctx.insert("Test","Sample");
    ctx.insert("data",&data);
    let s = tmpl.render("new.html",&ctx).map_err(|e|  error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html;charset=utf-8").body(s))
}

#[get("/posts/{id}")]
async fn show(info: web::Path<i32>,tmpl: web::Data<tera::Tera>) -> Result<impl Responder,Error> {
    info!("Call show");
    let info = info.into_inner();
    let post = data::get(info.try_into().unwrap());
    let mut ctx = tera::Context::new();
    ctx.insert("post",&post);
    ctx.insert("info",&info);
    let s = tmpl.render("show.html",&ctx).map_err(|e|  error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html;charset=utf-8").body(s))
}

#[derive(Deserialize, Debug,ToSchema)]
pub struct CreateForm {
    //id: i32,
    //posted: String,
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
#[utoipa::path(get, 
    path = "/json/posts", 
    tag="sample",
    responses(
        ( status=200, description="all message")
    )
)]
#[get("/json/posts")]
async fn json_posts() -> impl Responder {
    let posts = data::get_all();
    web::Json(posts)
}


#[utoipa::path(get, 
    path="/json/post/{id}",
    tag="sample",
    responses(
        (status=200, description="specified message",body=Message)
    ),
    params(
        ("id"=i32 , Path ,description="")
    )
)]
#[get("/json/post/{id}")]
async fn json_show(info: web::Path<i32>) -> impl Responder {
    let info = info.into_inner();
    let post = data::get(info.try_into().unwrap());
    web::Json(post)
}

#[utoipa::path(post, 
    path="/json/posts/create",
    tag="sample",
    responses(
        (status=200, description="specified message",body=Message)
    ),
    request_body=CreateForm
)]

#[post("/json/posts/create")]
pub async fn json_create(params: web::Form<CreateForm>) -> impl Responder {
    let now: DateTime<Local> = Local::now();
    let mut message = data::Message {
        id: 0,
        posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    message = data::create(message);
    web::Redirect::to(format!("/json/posts/{}", message.id)).see_other()
}


