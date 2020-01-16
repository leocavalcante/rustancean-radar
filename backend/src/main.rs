#[macro_use]
extern crate actix_web;

use actix_web::{App, Error, HttpResponse, HttpServer};
use actix_web::error::ErrorBadRequest;
use actix_web::web::Json;
use diesel::RunQueryDsl;
use reqwest::header::USER_AGENT;

use backend::application::{DevRequest, GitHubUser};
use backend::establish_connection;
use backend::models::{Dev, NewDev};
use backend::schema::devs;

#[post("/devs")]
async fn sign_dev(info: Json<DevRequest>) -> Result<HttpResponse, Error> {
    let github_user: GitHubUser = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}", info.github).as_str())
        .header(USER_AGENT, "github.com/leocavalcante/rustancean-radar")
        .send().await.map_err(ErrorBadRequest)?
        .json::<GitHubUser>()
        .await.map_err(ErrorBadRequest)?;

    let techs: Vec<&str> = info.techs.split(",").map(str::trim).collect();

    let dev = NewDev {
        github: info.github.as_str(),
        name: github_user.name.as_str(),
        avatar_url: github_user.avatar_url.as_str(),
        bio: github_user.bio.as_str(),
        techs,
    };

    let conn = establish_connection();

    diesel::insert_into(devs::table)
        .values(&dev)
        .get_result::<Dev>(&conn);

    Ok(HttpResponse::Ok().json(dev))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(sign_dev))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}