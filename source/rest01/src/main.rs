//https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
#[post("/welcome_user")]
async fn welcome_user(info: web::Json<Info>) -> impl Responder {
    //Ok(format!("Welcome {}!", info.username))
    let response = format!("username: {}", info.username);
    HttpResponse::Ok().body(response)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/todo")]
async fn index_todo() -> impl Responder {
    HttpResponse::Ok().body("{'todo': 'none'}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index_todo)
            .service(echo)
            .service(welcome_user)
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}