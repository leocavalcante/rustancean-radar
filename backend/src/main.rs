use actix_web::{App, Error, HttpResponse, HttpServer, post, Responder, web};
use serde::{Deserialize, Serialize};

use application::DevRequest;

mod domain;
mod application;

#[derive(Serialize)]
struct HelloWorld {
    message: String,
}

#[post("/devs")]
async fn sign_dev(info: web::Json<DevRequest>) -> Result<HttpResponse, Error> {
    println!("{:?}", info);
    let message = HelloWorld { message: "Hello OminiStack".to_string() };
    Ok(HttpResponse::Ok().json(message))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(sign_dev))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}