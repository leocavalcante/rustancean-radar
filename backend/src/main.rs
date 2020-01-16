#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate reqwest;

use actix_web::{App, error, HttpResponse, HttpServer, post, web};
use actix_web::dev::Service;
use diesel::{Connection, PgConnection};
use reqwest::header::USER_AGENT;
use serde::Serialize;

use crate::models::Dev;

mod models;
mod application;

#[derive(Serialize)]
struct HelloWorld {
    message: String,
}

#[post("/devs")]
async fn sign_dev(info: web::Json<application::DevRequest>) -> Result<HttpResponse, error::Error> {
    let github_user = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}", info.github).as_str())
        .header(USER_AGENT, "github.com/leocavalcante/rustancean-radar")
        .send().await.map_err(error::ErrorBadRequest)?
        .json::<application::GitHubUser>()
        .await.map_err(error::ErrorBadRequest)?;

    let techs: Vec<&str> = info.techs.split(",").map(str::trim).collect();

    println!("{:?}", techs);
    let message = HelloWorld { message: "Hello OminiStack".to_string() };
    Ok(HttpResponse::Ok().json(message))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| App::new().service(sign_dev))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}

fn conn() -> PgConnection {
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL at .env file");
    PgConnection::establish(&db_url).expect("Could not connect to Postgres")
}