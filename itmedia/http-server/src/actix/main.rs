use crate::handler::data::Message;
use crate::handler::CreateForm;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::io::Result;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handler;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello , Actix Web")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(
        paths(
            handler::json_posts,
            handler::json_show,
            handler::json_create,
            ),
        components(
            schemas(Message,
                CreateForm)
        ),
        tags((name="sample" , description="sample tags"))
        )
    ]
    struct ApiDoc;
    HttpServer::new(|| {
        App::new()
            .service(handler::index)
            .service(handler::new)
            .service(handler::create)
            .service(handler::show)
            .service(handler::json_posts)
            .service(handler::json_show)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/opanapi.json", ApiDoc::openapi()),
            )
            .default_service(web::to(handler::not_found))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

/*
fn main() {
    println!("Hello World/n");
}
*/
