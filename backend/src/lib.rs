#[macro_use]
extern crate diesel;

use diesel::PgConnection;
use diesel::prelude::*;

pub mod schema;
pub mod models;
pub mod application;
pub mod controllers;
pub mod utils;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}