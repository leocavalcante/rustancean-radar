use actix_web::{Error, get, HttpResponse, post};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::web::Json;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::query_dsl::filter_dsl::FilterDsl;
use reqwest::header::USER_AGENT;

use crate::application::{DevRequest, GitHubUser};
use crate::establish_connection;
use crate::models::{Dev, NewDev};
use crate::schema;

#[get("/devs")]
pub async fn index() -> Result<HttpResponse, Error> {
    let conn = establish_connection();
    let devs = schema::devs::table.load::<Dev>(&conn)
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(devs))
}

#[post("/devs")]
pub async fn store(input: Json<DevRequest>) -> Result<HttpResponse, Error> {
    use schema::devs::dsl::github;

    let conn = establish_connection();

    if let Ok(dev) = schema::devs::table
        .filter(github.eq(&input.github))
        .first::<Dev>(&conn) {
        return Ok(HttpResponse::Ok().json(&dev));
    }

    let github_user: GitHubUser = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}", input.github).as_str())
        .header(USER_AGENT, "github.com/leocavalcante/rustancean-radar")
        .send().await.map_err(ErrorBadRequest)?
        .json::<GitHubUser>()
        .await.map_err(ErrorBadRequest)?;

    let techs = crate::utils::csv_to_vec(input.techs.to_string());

    let new_dev = NewDev {
        github: input.github.as_str(),
        name: github_user.name.as_str(),
        avatar_url: github_user.avatar_url.as_str(),
        bio: github_user.bio.as_str(),
        techs,
        lat: &input.lat,
        lng: &input.lng,
    };

    let dev = diesel::insert_into(schema::devs::table)
        .values(&new_dev)
        .get_result::<Dev>(&conn)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&dev))
}