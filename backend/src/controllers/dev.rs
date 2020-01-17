use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::query_dsl::filter_dsl::FilterDsl;
use reqwest::header::USER_AGENT;
use warp::Rejection;

use crate::application::{DevRequest, GitHubUser};
use crate::establish_connection;
use crate::models::{Dev, NewDev};
use crate::schema;

pub async fn index() -> Result<impl warp::Reply, Rejection> {
    let conn = establish_connection();
    let devs = schema::devs::table.load::<Dev>(&conn)
        .unwrap();

    Ok(warp::reply::json(&devs))
}

pub async fn store(input: DevRequest) -> Result<impl warp::Reply, Rejection> {
    use schema::devs::dsl::github;

    let conn = establish_connection();

    if let Ok(dev) = schema::devs::table
        .filter(github.eq(&input.github))
        .first::<Dev>(&conn) {
        return Ok(warp::reply::json(&dev));
    }

    let github_user: GitHubUser = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}", input.github).as_str())
        .header(USER_AGENT, "github.com/leocavalcante/rustancean-radar")
        .send().await
        .unwrap()
        .json::<GitHubUser>().await
        .unwrap();

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
        .unwrap();

    Ok(warp::reply::json(&dev))
}