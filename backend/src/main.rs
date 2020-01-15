use actix_web::{App, Error, get, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HelloWorld {
    message: String,
}

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    let message = HelloWorld { message: "Hello OminiStack".to_string() };
    Ok(HttpResponse::Ok().json(message))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}