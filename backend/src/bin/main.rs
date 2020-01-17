use actix_cors::Cors;
use actix_web::{App, http::header, HttpServer, middleware::Logger};

use backend::controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .wrap(Cors::new()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .finish())
        .service(controllers::dev::store)
        .service(controllers::dev::index)
        .service(controllers::search::search))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}