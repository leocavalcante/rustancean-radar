use actix_web::{App, HttpServer};

use backend::controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(controllers::dev::store))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}