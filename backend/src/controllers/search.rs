use actix_web::{Error, get, HttpResponse};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Query;
use diesel::prelude::*;

use crate::{establish_connection, schema};
use crate::application::SearchQuery;
use crate::models::Dev;
use crate::utils;

#[get("/search")]
pub async fn search(query: Query<SearchQuery>) -> Result<HttpResponse, Error> {
    use schema::devs::dsl;

    let conn = establish_connection();
    let techs = utils::csv_to_vec(query.techs.to_string());

    let devs = schema::devs::table.filter(
        dsl::techs.overlaps_with(techs)
    ).get_results::<Dev>(&conn)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(devs))
}
