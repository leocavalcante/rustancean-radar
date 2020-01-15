use actix_web::{App, Error, HttpResponse, HttpServer, post, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HelloWorld {
    message: String,
}

#[derive(Debug, Deserialize)]
struct PostRequest {
    name: String,
    email: String,
}

#[post("/")]
async fn index(info: web::Json<PostRequest>) -> Result<HttpResponse, Error> {
    println!("{:?}", info);
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