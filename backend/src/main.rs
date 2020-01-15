#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}