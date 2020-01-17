use std::convert::Infallible;

use diesel::expression::sql_literal::sql;
use diesel::prelude::*;
use warp::Rejection;

use crate::{establish_connection, schema};
use crate::application::SearchQuery;
use crate::models::Dev;
use crate::utils;

pub async fn search(query: SearchQuery) -> Result<impl warp::Reply, Rejection> {
    use schema::devs::dsl;

    let conn = establish_connection();
    let techs = utils::csv_to_vec(query.techs.to_string());

    let devs = schema::devs::table
        .filter(dsl::techs.overlaps_with(techs))
        .filter(sql(format!("2 * 3961 * asin(sqrt((sin(radians(({0} - lat) / 2))) ^ 2 + cos(radians(lat)) * cos(radians({0})) * (sin(radians(({1} - lng) / 2))) ^ 2)) < 10", query.lat, query.lng).as_str()))
        .get_results::<Dev>(&conn)
        .unwrap();

    Ok(warp::reply::json(&devs))
}