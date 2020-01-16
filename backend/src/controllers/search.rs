use actix_web::{Error, get, HttpResponse};
use actix_web::web::Query;

use crate::application::SearchQuery;

#[get("/search")]
pub async fn search(query: Query<SearchQuery>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(query.0))
}
